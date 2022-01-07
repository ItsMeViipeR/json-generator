use std::fs;
use std::env;

/// # json-generator
/// Json generator is a scripting language and a cli tool to create json file easily.
/// #
/// Using:
/// ```
/// a -> ["a", "b", "c"];
/// b -> 3;
/// c -> "foo";
/// d -> { a, b, c }
/// ```
/// #
/// Create a .json file from our .jg file:
/// ```
/// jg ./jgfile.jg
/// ```

fn main() {
    for args in env::args().skip(1) {
        println!("{}", args);
        let a = fs::read_to_string(args[1]).unwrap().replace("\n", "");
        let things = a.split(";").map(|res| res.to_owned()).collect::<Vec<_>>();

        for thing in things {
            let things = thing.split("->").map(|res| res.to_owned()).collect::<Vec<_>>();

            let mut i = 0;

            while i < things.len() {
                if i + 1 < things.len() {
                    let content = fs::read_to_string(format!("{}", args[1].replace(".jg", ""))).unwrap().replace('{', "").replace('}', "");

                    println!("{}", content);

                    if content != "".to_string() {
                        fs::write("./test.json", format!("{{{}, \
                        \"{}\": \"{}\" \
                    }}", content.replace(" }", "}"), things[i].replace(' ', ""), things[i + 1].replace("\"", "").replace(' ', ""))).unwrap();
                    } else {
                        fs::write("./test.json", format!("{{ \
                        \"{}\": \"{}\" \
                    }}", things[i].replace(' ', ""), things[i + 1].replace("\"", "").replace(' ', ""))).unwrap();
                    }

                    i += 1;
                } else {
                    break;
                }
            }
        }
    };
}