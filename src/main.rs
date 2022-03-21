use std::path::PathBuf;

use worldgen_lang::{format, io};
use worldgen_lang::parser::LangParser;
use worldgen_lang::parser::lexer::LangLexer;
use worldgen_lang::process;
use worldgen_lang::processor::ElementProcessor;

fn main() {
    /* let source = r#"
{ "enabled": true, "example_factor": 5.3, "fields": [ { "type": "minecraft:something", value: 99 }.interpolated(), 20 + 3 * min(9, 3 + 5) ] }
    "#;

    let _source = r#" [ 384.1, "Hello\tworld\u0021" ] "#;

    let _source = r#"5"#; */

    let args: Vec<String> = std::env::args().skip(1).collect(); // Skip the executable path

    if args.len() != 2 {
        eprintln!("There must be exactly two command-line arguments (input and output)!");
        return;
    }

    let input = PathBuf::from(&args[0]);
    let output = PathBuf::from(&args[1]);

    let mut processor = ElementProcessor::new();
    processor.add_postprocessor(process::process_operators);
    processor.add_postprocessor(process::process_functions);

    io::process(input, output, &mut |input_str| {
        let lexer = LangLexer::new(&input_str); // Moved into parser
        let mut parser = LangParser::new(lexer);

        let element = match parser.parse_full() {
            Ok(element) => element,
            Err(err) => {
                println!("Error while parsing: {}", err);
                return None;
            },
        };

        let result = processor.process(element);

        if !result.errors.is_empty() {
            println!();
            println!("Errors: {:?}", result.errors);
            return None;
        } else if !result.warnings.is_empty() {
            println!();
            println!("Warnings: {:?}", result.warnings);
            return None;
        } else if result.element.is_none() {
            println!();
            println!("Error during processing.");
            return None;
        }

        let formatted_result = match format::format_json(result.element.unwrap(), format::Options::Pretty { indentation: 2 }) {
            Ok(result) => result,
            Err(err) => {
                println!();
                println!("{}", err);
                return None;
            }
        };

        Some(formatted_result)
    });

    println!("Done.");
}
