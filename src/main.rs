use clap::Parser;

mod app;

use app::JsonGenerator;

use crate::app::proceed;

fn main() {
    let json_generator: JsonGenerator = JsonGenerator::parse();

    let input_file = json_generator.input;

    if !input_file.ends_with(".jg") {
        eprintln!("Error: Input file must have a .jg extension");
        std::process::exit(1);
    }

    proceed(&input_file);
}