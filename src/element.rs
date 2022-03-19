use std::fmt::{Debug, Formatter};

use crate::parser::lexer::LangToken;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ElementKind {
    Null,

    Boolean,
    Int,
    Float,
    String,
    Name,

    Array,
    Object,

    Binary,
    FunctionCall,
}

#[derive(Clone, PartialEq)]
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

#[macro_export]
macro_rules! string_element {
    ($string:literal) => {
        Element::StringElement(String::from($string))
    };
    ($string:expr) => {
        Element::StringElement($string)
    }
}

#[macro_export]
macro_rules! object_element {
    [$($name:expr => $element:expr),*] => {
        Element::ObjectElement(vec![$(($name, $element)),*])
    }
}

#[macro_export]
macro_rules! array_element {
    [$($element:expr),*] => {
        Element::ArrayElement(vec![$($element),*])
    }
}

impl From<Element> for ElementKind {
    fn from(element: Element) -> Self {
        element.kind()
    }
}

impl Element {
    pub fn kind(&self) -> ElementKind {
        match self {
            Element::NullElement => ElementKind::Null,
            Element::BooleanElement(_) => ElementKind::Boolean,
            Element::IntElement(_) => ElementKind::Int,
            Element::FloatElement(_) => ElementKind::Float,
            Element::StringElement(_) => ElementKind::String,
            Element::NameElement(_) => ElementKind::Name,
            Element::ArrayElement(_) => ElementKind::Array,
            Element::ObjectElement(_) => ElementKind::Object,
            Element::BinaryElement { .. } => ElementKind::Binary,
            Element::FunctionCallElement { .. } => ElementKind::FunctionCall,
        }
    }
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
