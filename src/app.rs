use clap::Parser;

#[derive(Parser, Debug)]
#[clap(name="json-generator")]
#[clap(about="A simple JSON generator CLI tool")]
pub struct JsonGenerator {
    pub input: String,
}

fn is_variable(s: &str) -> bool {
    // Matches: name = value (value can be quoted or unquoted)
    let s = s.trim();
    if let Some(idx) = s.find('=') {
        let (left, right) = s.split_at(idx);
        let left = left.trim();
        let right = right[1..].trim(); // skip '='
        !left.is_empty() && !right.is_empty() && !left.contains(' ') && !left.contains("->")
    } else {
        false
    }
}

fn is_object(s: &str) -> bool {
    // Matches: name -> { ... }
    let s = s.trim();
    if let Some(idx) = s.find("->") {
        let (left, right) = s.split_at(idx);
        let left = left.trim();
        let right = right[2..].trim(); // skip '->'
        !left.is_empty() && right.starts_with('{') && right.ends_with('}')
    } else {
        false
    }
}

fn resolve_variable_value(
    key: &str,
    variables: &std::collections::HashMap<String, String>,
    seen: &mut std::collections::HashSet<String>,
    used_keys: &mut std::collections::HashSet<String>,
) -> Option<String> {
    if seen.contains(key) {
        eprintln!("⚠️  Circular reference detected for variable '{}'", key);
        return None;
    }
    seen.insert(key.to_string());
    used_keys.insert(key.to_string());

    let value = variables.get(key)?;
    // Si la valeur est un tableau JSON, on traite récursivement ses éléments
    if value.starts_with('[') && value.ends_with(']') {
        let tokens: Vec<&str> = value
            .trim_matches(|c| c == '[' || c == ']')
            .split(',')
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .collect();

        let mut resolved_parts = vec![];
        for token in tokens {
            if variables.contains_key(token) {
                if let Some(resolved) = resolve_variable_value(token, variables, seen, used_keys) {
                    resolved_parts.push(resolved);
                } else {
                    // boucle détectée, on garde le token brut
                    resolved_parts.push(token.to_string());
                }
            } else {
                // littéral
                resolved_parts.push(token.to_string());
            }
        }
        seen.remove(key);
        Some(format!("[{}]", resolved_parts.join(", ")))
    } else if value.starts_with('{') && value.ends_with('}') {
        // Objet défini dans une variable
        let inner_keys: Vec<&str> = value
            .trim_matches(|c| c == '{' || c == '}')
            .split(',')
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .collect();

        let mut inner_json_parts = vec![];

        for key in inner_keys.iter() {
            if let Some(inner_val) = resolve_variable_value(key, variables, seen, used_keys) {
                used_keys.insert(key.to_string());
                inner_json_parts.push(format!("\"{}\": {}", key, inner_val));
            } else {
                eprintln!("⚠️  Variable '{}' used in '{}' is undefined or invalid", key, value);
            }
        }

        seen.remove(key);
        return Some(format!("{{ {} }}", inner_json_parts.join(", ")));
    } else {
        // Autres types : on retourne la valeur brute
        seen.remove(key);
        Some(value.clone())
    }
}

pub fn proceed(input_file: &str) {
    let content: String = std::fs::read_to_string(input_file)
        .expect("Failed to read input file");

    // Supprimer commentaires et lignes vides
    let lines: Vec<&str> = content.lines()
        .map(|line| {
            let line = line.trim();
            let line = if let Some(pos) = line.find('#') {
                &line[..pos]
            } else {
                line
            };
            let line = if let Some(pos) = line.find("//") {
                &line[..pos]
            } else {
                line
            };
            line.trim()
        })
        .filter(|line| !line.is_empty())
        .collect();

    let mut variables: std::collections::HashMap<String, String> = std::collections::HashMap::new();
    let mut objects: std::collections::HashMap<String, String> = std::collections::HashMap::new();

    for line in lines {
        if is_variable(line) {
            let parts: Vec<&str> = line.splitn(2, '=').collect();
            if parts.len() == 2 {
                let key = parts[0].trim().to_string();
                let value = parts[1].trim().to_string();
                variables.insert(key, value);
            }
        } else if is_object(line) {
            let parts: Vec<&str> = line.splitn(2, "->").collect();
            if parts.len() == 2 {
                let key = parts[0].trim().to_string();
                let value = parts[1].trim().to_string();
                objects.insert(key, value);
            }
        } else {
            eprintln!("Invalid line format: {}", line);
            break;
        }
    }

    // Résoudre toutes les variables récursivement
    let mut resolved_variables = std::collections::HashMap::new();
    let mut used_keys: std::collections::HashSet<String> = std::collections::HashSet::new();
    for key in variables.keys() {
        let mut seen = std::collections::HashSet::new();
        if let Some(val) = resolve_variable_value(key, &variables, &mut seen, &mut used_keys) {
            resolved_variables.insert(key.clone(), val);
        }
    }

    let mut objects_json: std::collections::HashMap<String, String> = std::collections::HashMap::new();

    for (obj_key, obj_value) in &objects {
        let inner_keys: Vec<&str> = obj_value
            .trim_matches(|c| c == '{' || c == '}')
            .split(',')
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .collect();

        let mut inner_json_parts = vec![];

        for key in inner_keys {
            if let Some(val) = resolved_variables.get(key) {
                inner_json_parts.push(format!("\"{}\": {}", key, val));
                used_keys.insert(key.to_string());
            } else {
                eprintln!("⚠️  Variable '{}' not defined", key);
            }
        }

        let joined = inner_json_parts
            .iter()
            .map(|entry| format!("    {}", entry))
            .collect::<Vec<_>>()
            .join(",\n");
        let obj_json = format!("  \"{}\": {{\n{}\n  }}", obj_key, joined);
        objects_json.insert(obj_key.clone(), obj_json);
    }

    // Warn about unused variables
    for key in resolved_variables.keys() {
        if !used_keys.contains(key) {
            eprintln!("⚠️  Variable '{}' was defined but never used", key);
        }
    }

    let mut final_json = String::new();
    final_json.push_str("{\n");

    for value in objects_json.values() {
        final_json.push_str(&format!("{},\n", value));
    }

    if final_json.ends_with(",\n") {
        final_json.truncate(final_json.len() - 2);
    }

    final_json.push_str("\n}");

    let output_file = input_file.replace(".jg", ".json");
    std::fs::write(&output_file, final_json)
        .expect("Failed to write output file");
    println!("Output written to: {output_file}");
}