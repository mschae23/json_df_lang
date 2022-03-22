use std::path::PathBuf;

use worldgen_lang::{format, io};
use worldgen_lang::parser::LangParser;
use worldgen_lang::parser::lexer::LangLexer;
use worldgen_lang::processor::{BinaryOperator, ElementProcessor, NoArgFunction, OneArgFunction, TwoArgsFunction};

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
    // processor.add_postprocessor(process::process_operators);
    // processor.add_postprocessor(process::process_functions);

    processor.add_no_arg_function(NoArgFunction::new(String::from("blend_alpha")));
    processor.add_no_arg_function(NoArgFunction::new(String::from("blend_offset")));
    processor.add_no_arg_function(NoArgFunction::new(String::from("beardifier")));
    processor.add_no_arg_function(NoArgFunction::new(String::from("old_blended_noise")));
    processor.add_no_arg_function(NoArgFunction::new(String::from("end_islands")));

    processor.add_one_arg_function(OneArgFunction::new_with_method_syntax(String::from("abs")));
    processor.add_one_arg_function(OneArgFunction::new_with_method_syntax(String::from("half_negative")));
    processor.add_one_arg_function(OneArgFunction::new_with_method_syntax(String::from("quarter_negative")));
    processor.add_one_arg_function(OneArgFunction::new_with_method_syntax(String::from("square")));
    processor.add_one_arg_function(OneArgFunction::new_with_method_syntax(String::from("cube")));
    processor.add_one_arg_function(OneArgFunction::new_with_method_syntax(String::from("squeeze")));
    processor.add_one_arg_function(OneArgFunction::new_with_method_syntax(String::from("interpolated")));
    processor.add_one_arg_function(OneArgFunction::new_with_method_syntax(String::from("flat_cache")));
    processor.add_one_arg_function(OneArgFunction::new_with_method_syntax(String::from("cache_2d")));
    processor.add_one_arg_function(OneArgFunction::new_with_method_syntax(String::from("cache_once")));
    processor.add_one_arg_function(OneArgFunction::new_with_method_syntax(String::from("cache_all_in_cell")));
    processor.add_one_arg_function(OneArgFunction::new_with_method_syntax(String::from("slide")));

    processor.add_two_args_function(TwoArgsFunction::new_without_method_syntax(String::from("min")));
    processor.add_two_args_function(TwoArgsFunction::new_without_method_syntax(String::from("max")));

    processor.add_binary_operator(BinaryOperator::new(String::from("+"), String::from("add")));
    processor.add_binary_operator(BinaryOperator::new(String::from("*"), String::from("mul")));

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
