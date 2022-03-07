use std::fmt::{Debug, Formatter};

use crate::parser::lexer::LangToken;

#[derive(Clone)]
pub enum Element {
    NullElement,

    BooleanElement(bool),
    IntElement(i32),
    FloatElement(f64),
    StringElement(String),
    NameElement(String),

    ArrayElement(Vec<Element>),
    ObjectElement(Vec<(Element, Element)>), // Key-value pair for object

    // Additional (not JSON)
    BinaryElement {
        left: Box<Element>,
        operator: LangToken,
        right: Box<Element>,
    },

    FunctionCallElement {
        receiver: Option<Box<Element>>,
        name: String,
        arguments: Option<Vec<Element>>,
    },
}

impl Debug for Element {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Element::NullElement => f.write_str("null"),
            Element::BooleanElement(value) => write!(f, "{}", value),
            Element::IntElement(value) => write!(f, "{}", value),
            Element::FloatElement(value) => write!(f, "{}", value),
            Element::StringElement(value) => write!(f, "\"{}\"", value),
            Element::NameElement(value) => write!(f, "{}", value),
            Element::ArrayElement(values) => {
                if values.is_empty() {
                    return write!(f, "[]");
                }

                f.write_str("[ ")?;

                for (i, value) in values.iter().enumerate() {
                    write!(f, "{:?}", value)?;

                    if i < values.len() - 1 {
                        f.write_str(", ")?;
                    }
                }

                f.write_str(" ]")
            },
            Element::ObjectElement(fields) => {
                if fields.is_empty() {
                    return write!(f, "{{}}");
                }

                f.write_str("{ ")?;

                for (i, (key, value)) in fields.iter().enumerate() {
                    write!(f, "{0:?}: {1:?}", key, value)?;

                    if i < fields.len() - 1 {
                        f.write_str(", ")?;
                    }
                }

                f.write_str(" }")
            },
            Element::BinaryElement { left, operator, right } =>
                write!(f, "({0:?} {1} {2:?})", *left, operator.text(), *right),

            Element::FunctionCallElement { receiver, name, arguments } => {
                if let Some(receiver) = receiver {
                    write!(f, "{:?}.", *receiver)?;
                }

                f.write_str(name)?;

                if let Some(args) = arguments {
                    f.write_str("(")?;

                    for (i, arg) in args.iter().enumerate() {
                        write!(f, "{:?}", arg)?;

                        if i < args.len() - 1 {
                            f.write_str(", ")?;
                        }
                    }

                    f.write_str(")")?;
                }

                Ok(())
            },
        }
    }
}
