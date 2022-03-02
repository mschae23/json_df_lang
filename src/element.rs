#[derive(Clone, Debug)]
pub enum Element {
    NullElement,

    BooleanElement(bool),
    IntElement(i32),
    FloatElement(f64),
    StringElement(String),
    NameElement(String),

    ArrayElement(Vec<Element>),
    ObjectElement(Vec<(Element, Element)>), // Key-value pair for object
}
