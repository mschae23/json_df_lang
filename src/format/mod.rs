use std::fmt::{Display, Formatter, Write};

use crate::element::Element;
use crate::util;

pub enum Options {
    Pretty {
        indentation: i32,
    },
    Compressed,
}

pub enum SymbolKind {
    Symbol, Function, Operator,
}

impl Display for SymbolKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SymbolKind::Symbol => f.write_str("symbol"),
            SymbolKind::Function => f.write_str("function"),
            SymbolKind::Operator => f.write_str("operator"),
        }
    }
}

pub enum Error {
    UnresolvedSymbol(SymbolKind, String, Element),
    NotSimpleElement(Element),
    FormatError(std::fmt::Error),
}

impl From<std::fmt::Error> for Error {
    fn from(error: std::fmt::Error) -> Self {
        Error::FormatError(error)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::UnresolvedSymbol(kind, name, element) => write!(f, "Unresolved {} '{}': {:?}", kind, name, element),
            Error::NotSimpleElement(element) => write!(f, "Element is not simple (internal formatting failure): {:?}", element),
            Error::FormatError(err) => write!(f, "Formatting error: {}", err),
        }
    }
}

type Result = std::result::Result<String, Error>;

pub fn format_json(element: Element, options: Options) -> Result {
    let output = String::new();

    match options {
        Options::Pretty { indentation } => format_json_pretty(&element, output, &" ".repeat(indentation as usize), ""),
        Options::Compressed => format_json_compressed(&element, output),
    }
}

fn format_json_pretty(element: &Element, mut output: String, indentation: &str, indent: &str) -> Result {
    match &element {
        Element::ArrayElement(values) => {
            let sub_indent = String::from(indent) + indentation;
            output.write_str("[\n")?;

            for (i, element) in values.iter().enumerate() {
                output.write_str(&sub_indent)?;
                output = format_json_pretty(element, output, indentation, &sub_indent)?;

                if i < values.len() - 1 {
                    output.write_str(",\n")?;
                } else {
                    output.write_str("\n")?;
                }
            }

            output.write_str(indent)?;
            output.write_str("]")?;
            Ok(output)
        },
        Element::ObjectElement(fields) => {
            let sub_indent = String::from(indent) + indentation;
            output.write_str("{\n")?;

            for (i, (key, value)) in fields.iter().enumerate() {
                output.write_str(&sub_indent)?;

                output = format_json_pretty(key, output, indentation, &sub_indent)?;
                output.write_str(": ")?;
                output = format_json_pretty(value, output, indentation, &sub_indent)?;

                if i < fields.len() - 1 {
                    output.write_str(",\n")?;
                } else {
                    output.write_str("\n")?;
                }
            }

            output.write_str(indent)?;
            output.write_str("}")?;
            Ok(output)
        },
        Element::BinaryElement { operator, .. } => Err(Error::UnresolvedSymbol(SymbolKind::Operator, String::from(operator.text()), element.clone())),
        Element::FunctionCallElement { name, .. } => Err(Error::UnresolvedSymbol(SymbolKind::Function, name.clone(), element.clone())),
        Element::NameElement(name) => Err(Error::UnresolvedSymbol(SymbolKind::Symbol, name.clone(), element.clone())),
        _ => format_json_simple(element, output),
    }
}

fn format_json_compressed(element: &Element, mut output: String) -> Result {
    match &element {
        Element::ArrayElement(values) => {
            output.write_str("[")?;

            for element in values {
                output = format_json_compressed(element, output)?;
            }

            output.write_str("]")?;
            Ok(output)
        },
        Element::ObjectElement(fields) => {
            output.write_str("{")?;

            for (key, value) in fields {
                output = format_json_compressed(key, output)?;
                output.write_str(":")?;
                output = format_json_compressed(value, output)?;
            }

            output.write_str("}")?;
            Ok(output)
        },
        Element::BinaryElement { operator, .. } => Err(Error::UnresolvedSymbol(SymbolKind::Operator, String::from(operator.text()), element.clone())),
        Element::FunctionCallElement { name, .. } => Err(Error::UnresolvedSymbol(SymbolKind::Function, name.clone(), element.clone())),
        Element::NameElement(name) => Err(Error::UnresolvedSymbol(SymbolKind::Symbol, name.clone(), element.clone())),
        _ => format_json_simple(element, output),
    }
}

fn format_json_simple(element: &Element, mut output: String) -> Result {
    match element {
        Element::NullElement => output.push_str("null"),
        Element::BooleanElement(value) => output.push_str(if *value { "true" } else { "false" }),
        Element::IntElement(value) => write!(output, "{}", value)?,
        Element::FloatElement(value) => write!(output, "{}", value)?,
        Element::StringElement(value) => write!(output, "\"{}\"", util::escape_str(&value))?,
        Element::NameElement(name) => write!(output, "{}", name)?,
        _ => return Err(Error::NotSimpleElement(element.clone())),
    };

    Ok(output)
}
