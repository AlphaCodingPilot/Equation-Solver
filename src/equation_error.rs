#[derive(PartialEq, Debug)]
pub enum EquationError {
    EmptyEquation,
    EmptyVariableName,
    InvalidElement(String),
    MissingOperation,
    ParenthesisError,
    InvalidSeparator,
    InvalidSeparatorAmount,
    InvalidOperation,
    DivisionByZero,
    TooHighDegree { max_degree: i32 },
    ComplexNumbers,
}
