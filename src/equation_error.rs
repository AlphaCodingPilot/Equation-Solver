use EquationError::*;

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

impl EquationError {
    pub fn log_message(&self) -> String {
        let message = match self {
            InvalidElement(element) => format!("Equation contains an invalid element: {element}"),
            TooHighDegree { max_degree } => format!(
                "Polynomial equations with a degree greater than {max_degree} are not supported"
            ),
            EmptyEquation => String::from("Empty equation"),
            EmptyVariableName => String::from("Variable name was not specified"),
            NoOccurrencesOfVariable => String::from("Variable does not occur in the equation"),
            MissingOperation => String::from("Equation is missing an operation"),
            ParenthesisError => String::from("Equation contains invalid parenthesis"),
            InvalidSeparator => String::from("Equation contains invalid equals sign"),
            InvalidSeparatorAmount => {
                String::from("An equation must contain exactly one equals sign")
            }
            InvalidOperation => String::from("Equation contains invalid operation"),
            DivisionByZero => String::from("Division by zero is undefined"),
            ComplexNumbers => String::from("Complex numbers are not supported"),
        };
        format!("ERROR: {message}")
    }
}
