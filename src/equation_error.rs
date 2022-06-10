#[derive(PartialEq, Debug)]
pub enum EquationError {
    EmptyEquation,
    EmptyVariableName,
    NoOccurrencesOfVariable,
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
