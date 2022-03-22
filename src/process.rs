use crate::element::Element;
use crate::{object_element, string_element};
use crate::processor::{ProcessError, ProcessResult};

pub fn process_operators(element: Element) -> ProcessResult {
    match element {
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
    }
}

pub fn process_functions(element: Element) -> ProcessResult {
    match element {
        Element::FunctionCallElement { receiver: None, name, arguments: Some(mut args) } => {
            if args.len() == 3 {
                todo!()
            } else if args.len() == 2 {
                if name == "min" || name == "max" {
                    ProcessResult::from_element(object_element!(string_element!("type") => string_element!(String::from("minecraft:") + &name),
                        string_element!("argument1") => args.swap_remove(0), string_element!("argument2") => args.swap_remove(0)
                    ))
                } else {
                    ProcessResult::from_element(Element::FunctionCallElement { receiver: None, name, arguments: Some(args) })
                }
            } else if args.len() == 1 {
                if name == "interpolated" || name == "flat_cache" || name == "cache_2d" || name == "cache_once" || name == "cache_all_in_cell" || name == "abs" {
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
            if name == "interpolated" || name == "flat_cache" || name == "cache_2d" || name == "cache_once" || name == "cache_all_in_cell" || name == "abs" {
                ProcessResult::from_element(object_element!(string_element!("type") => string_element!(String::from("minecraft:") + &name),
                    string_element!("argument") => *receiver
                ))
            } else {
                ProcessResult::from_element(Element::FunctionCallElement { receiver: Some(receiver), name, arguments: Some(args) })
            }
        }
        _ => ProcessResult::from_element(element),
    }
}
