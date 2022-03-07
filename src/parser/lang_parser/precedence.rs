#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Precedence {
    None = 0,

    Any, // Precedence of expressions in array, object, or group

    Sum,    // Addition and subtraction
    Factor, // Multiplication and division

    Call, // Function call
}
