use worldgen_lang::{object_element, string_element};
use worldgen_lang::element::Element;
use worldgen_lang::parser::LangParser;
use worldgen_lang::parser::lexer::LangLexer;
use worldgen_lang::processor::{ElementProcessor, ProcessError, ProcessResult};

fn main() {
    let source = r#"
{ "enabled": true, "example_factor": 5.3, "fields": [ { "type": "minecraft:something", value: 99 }.interpolated(), 20 + 3 * min(9, 3 + 5) ] }
    "#;

    let _source = r#" [ 384.1, "Hello\tworld\u0021" ] "#;

    let _source = r#"5"#;

    let lexer = LangLexer::new(source); // Moved into parser
    let mut parser = LangParser::new(lexer);

    let element = match parser.parse_element() {
        Ok(element) => {
            println!("{:?}", element);
            element
        }
        Err(err) => {
            println!("Error while parsing: {}", err);
            Element::NullElement
        }
    };

    let mut processor = ElementProcessor::new();
    processor.add_postprocessor(|element| match element {
        Element::BinaryElement { left, operator, right } => match operator.text() {
            "+" => ProcessResult::from_element(object_element!(string_element!("type") => string_element!("minecraft:add"),
                string_element!("argument1") => *left, string_element!("argument2") => *right
            )),
            "-" => ProcessResult::new(object_element!(string_element!("type") => string_element!("minecraft:subtract"),
                string_element!("argument1") => *left, string_element!("argument2") => *right
            ), Vec::new(), vec![ProcessError::UnknownOperator(String::from("-"))]),
            "*" => ProcessResult::from_element(object_element!(string_element!("type") => string_element!("minecraft:mul"),
                string_element!("argument1") => *left, string_element!("argument2") => *right
            )),
            // "/" => ProcessResult::from_element(Element::IntElement(left / right)),
            _ => ProcessResult::from_element(Element::BinaryElement {
                left,
                operator: operator.clone(),
                right,
            }),
        },
        _ => ProcessResult::from_element(element),
    });
    processor.add_postprocessor(|element| match element {
        Element::FunctionCallElement { receiver: None, name, arguments: Some(mut args) } => {
            if args.len() == 2 {
                if name == "min" || name == "max" {
                    ProcessResult::from_element(object_element!(string_element!("type") => string_element!(String::from("minecraft:") + &name),
                        string_element!("argument1") => args.swap_remove(0), string_element!("argument2") => args.swap_remove(0)
                    ))
                } else {
                    ProcessResult::from_element(Element::FunctionCallElement { receiver: None, name, arguments: Some(args) })
                }
            } else if args.len() == 1 {
                if name == "interpolated" || name == "flat_cache" || name == "cache_2d" || name == "cache_once" || name == "cache_all_in_cell" {
                    ProcessResult::from_element(object_element!(string_element!("type") => string_element!(String::from("minecraft:") + &name),
                    string_element!("argument") => args.swap_remove(0)
                ))
                } else {
                    ProcessResult::from_element(Element::FunctionCallElement { receiver: None, name, arguments: Some(args) })
                }
            } else {
                ProcessResult::from_element(Element::FunctionCallElement { receiver: None, name, arguments: Some(args) })
            }
        }
        Element::FunctionCallElement { receiver: Some(receiver), name, arguments: Some(args) } if args.len() == 0 => {
            if name == "interpolated" || name == "flat_cache" || name == "cache_2d" || name == "cache_once" || name == "cache_all_in_cell" {
                ProcessResult::from_element(object_element!(string_element!("type") => string_element!(String::from("minecraft:") + &name),
                    string_element!("argument") => *receiver
                ))
            } else {
                ProcessResult::from_element(Element::FunctionCallElement { receiver: Some(receiver), name, arguments: Some(args) })
            }
        }
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

    println!("Done.");
}
