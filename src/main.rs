#![feature(box_patterns)]

use worldgen_lang::element::Element;
use worldgen_lang::parser::LangParser;
use worldgen_lang::parser::lexer::LangLexer;
use worldgen_lang::processor::{ElementProcessor, ProcessResult};

fn main() {
    let source = r#"
{ "enabled": true, "example_factor": 5.3, "fields": [ { "type": "minecraft:something", value: 99 }, 20 + 3 * min(9, 3 + 5) ] }
    "#;

    let _source = r#" [ 384.1, "Hello\tworld\u0021" ] "#;

    let _source = r#"5"#;

    let lexer = LangLexer::new(source); // Moved into parser
    let mut parser = LangParser::new(lexer);

    let element = match parser.parse_element() {
        Ok(element) => {
            println!("{:?}", element);
            element
        },
        Err(err) => {
            println!("Error while parsing: {}", err);
            Element::NullElement
        },
    };

    let mut processor = ElementProcessor::new();
    processor.add_postprocessor(|element| match element {
        Element::BinaryElement { left: box Element::IntElement(left_value), operator, right: box Element::IntElement(right_value) } => {
            match operator.text() {
                "+" => ProcessResult::from_element(Element::IntElement(left_value + right_value)),
                "-" => ProcessResult::from_element(Element::IntElement(left_value - right_value)),
                "*" => ProcessResult::from_element(Element::IntElement(left_value * right_value)),
                "/" => ProcessResult::from_element(Element::IntElement(left_value / right_value)),
                _ => ProcessResult::from_element(Element::BinaryElement {
                    left: Box::new(Element::IntElement(left_value)),
                    operator,
                    right: Box::new(Element::IntElement(right_value)) }),
            }
        },
        _ => ProcessResult::from_element(element),
    });
    processor.add_postprocessor(|element| match &element {
        Element::FunctionCallElement { receiver: None, name, arguments: Some(args) } => {
            if args.len() == 2 {
                if name == "min" {
                    if let Element::IntElement(left) = args[0] {
                        if let Element::IntElement(right) = args[1] {
                            return ProcessResult::from_element(Element::IntElement(std::cmp::min(left, right)))
                        }
                    }
                } else if name == "max" {
                    if let Element::IntElement(left) = args[0] {
                        if let Element::IntElement(right) = args[1] {
                            return ProcessResult::from_element(Element::IntElement(std::cmp::max(left, right)))
                        }
                    }
                }
            }

            ProcessResult::from_element(element)
        },

        _ => ProcessResult::from_element(element),
    });

    let result = processor.process(element);

    println!("Processed element: ");
    println!("{:?}", result.element());

    if !result.errors.is_empty() {
        println!();
        println!("Errors: {:?}", result.errors);
    } else if !result.warnings.is_empty() {
        println!();
        println!("Warnings: {:?}", result.warnings);
    }

    /*
    loop {
        match lexer.scan_token() {
            Ok(token) if *token.token_type() == LangTokenType::Eof => break,
            Ok(token) => println!("{:?}", token),
            Err(e) => {
                println!("Error: {:?}", e);
                break
            },
        }
    }
    */

    println!("Done.");
}
