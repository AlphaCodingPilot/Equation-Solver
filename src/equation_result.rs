use std::fmt::{Display, Formatter, Result as FmtResult};
use EquationResult::*;

#[derive(PartialEq, Debug)]
pub enum EquationResult {
    Solutions(Vec<f64>),
    Unsolvable,
    InfiniteSolutions { exceptions: Vec<f64> },
}

impl Display for EquationResult {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        let output = match self {
            Solutions(values) if values.len() == 1 => values[0].to_string(),
            Solutions(values) => format!(
                "{{{}}}",
                values
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
            Unsolvable => String::from("{}"),
            InfiniteSolutions { exceptions } => match exceptions.is_empty() {
                true => String::from("R"),
                false => format!(
                    "R\\{{{}}}",
                    exceptions
                        .iter()
                        .map(ToString::to_string)
                        .collect::<Vec<String>>()
                        .join(", ")
                ),
            },
        };
        write!(f, "{}", output)
    }
}
