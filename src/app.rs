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

fn is_array(s: &str) -> bool {
    // Matches: name = [value1, value2, ...]
    let s = s.trim();
    if let Some(idx) = s.find('=') {
        let (left, right) = s.split_at(idx);
        let left = left.trim();
        let right = right[1..].trim(); // skip '='
        !left.is_empty() && right.starts_with('[') && right.ends_with(']')
    } else {
        false
    }
}

fn is_json_object(s: &str) -> bool {
    // Matches: name = { name: value, ... }
    let s = s.trim();

    if let Some(idx) = s.find('=') {
        let (left, right) = s.split_at(idx);
        let left = left.trim();
        let right = right[1..].trim(); // skip '='
        !left.is_empty() && right.starts_with('{') && right.ends_with('}')
    } else {
        false
    }
}

pub fn proceed(input_file: &str) {
    let content: String = std::fs::read_to_string(input_file)
        .expect("Failed to read input file");

    let lines: Vec<&str> = content.lines()
        .filter(|line| !line.trim().is_empty() && !line.starts_with('#'))
        .map(|line| line.trim())
        .collect::<Vec<&str>>();

    let mut variables: std::collections::HashMap<String, String> = std::collections::HashMap::new();
    let mut objects: std::collections::HashMap<String, String> = std::collections::HashMap::new();

    for line in lines {
        if is_variable(line) {
            if is_array(line) {
                let parts: Vec<&str> = line.splitn(2, '=').collect();
                if parts.len() == 2 {
                    let key = parts[0].trim().to_string();
                    let value = parts[1].trim().to_string();
                    variables.insert(key, value);
                }
            } else if is_json_object(line) {
                let parts: Vec<&str> = line.splitn(2, '=').collect();

                if parts.len() == 2 {
                    let key = parts[0].trim().to_string();
                    let value = parts[1].trim().to_string();

                    variables.insert(key, value);
                }
            } else {
                let parts: Vec<&str> = line.splitn(2, '=').collect();
                if parts.len() == 2 {
                    let key = parts[0].trim().to_string();
                    let value = parts[1].trim().to_string();
                    variables.insert(key, value);
                }
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

    let mut variables_json: std::collections::HashMap<String, String> = std::collections::HashMap::new();

    // replace variables in objects by their values
    for (key, value) in &variables {
        let value_json = format!("\"{key}\": {value}");

        variables_json.insert(key.clone(), value_json);
    }

    let mut objects_json: std::collections::HashMap<String, String> = std::collections::HashMap::new();

    for (obj_key, obj_value) in &objects {
        let mut replaced_value = obj_value.clone();
        for (var_key, var_json) in &variables_json {
            replaced_value = replaced_value.replace(var_key, var_json);
        }
        
        let obj_json = format!("\"{obj_key}\": {replaced_value}");

        objects_json.insert(obj_key.clone(), obj_json);
    }

    let mut final_json = String::new();
    final_json.push_str("{\n");
    
    for (_, value) in &objects_json {
        final_json.push_str(&format!("  {},\n", value));
    }
    if final_json.ends_with(",\n") {
        final_json.truncate(final_json.len() - 2); // remove trailing comma
    }
    final_json.push_str("\n}");

    // write the final JSON to a file
    let output_file = input_file.replace(".jg", ".json");
    std::fs::write(&output_file, final_json)
        .expect("Failed to write output file");
    println!("Output written to: {output_file}");
}