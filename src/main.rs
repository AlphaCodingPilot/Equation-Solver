use colored::*;
use std::io;
use std::io::Write;
use std::process;

mod equation;
mod equation_element;
mod equation_error;
mod equation_result;
mod equation_side;
mod exceptions_in_domain;
mod nested_term;
mod solve_equation;
mod term;
mod token_stream;

#[cfg(test)]
mod tests;

use equation_error::EquationError::{self, *};
use equation_result::EquationResult::{self, *};
use token_stream::EquationInput;

fn main() {
    let input = read_input();
    let equation_result = solve_equation::solve_equation(&input);
    output(equation_result, input.variable_name);
}

fn read_input() -> EquationInput {
    println!("Enter equation");

    let mut equation = String::new();
    io::stdin()
        .read_line(&mut equation)
        .expect("Failed to read line");
    let equation = equation.trim().to_string();

    print!("Solve for ");
    io::stdout().flush().expect("Failed to flush output");

    let mut variable_name = String::new();
    io::stdin()
        .read_line(&mut variable_name)
        .expect("Failed to read line");
    let variable_name = variable_name.trim().to_string();

    EquationInput::new(equation, variable_name)
}

fn output(result: Result<EquationResult, EquationError>, variable_name: String) {
    let output = match result {
        Ok(result) => result_output(result, variable_name),
        Err(error) => {
            eprintln!("{}", error_output(error).red());
            process::exit(1);
        }
    };

    println!("{output}");
}

fn result_output(result: EquationResult, variable_name: String) -> String {
    match result {
        Solutions(values) if values.len() == 1 => {
            format!("{variable_name} = {}", values[0])
        }
        Solutions(values) => format!(
            "{variable_name} = {{{}}}",
            values
                .iter()
                .map(|value| value.to_string())
                .collect::<Vec<String>>()
                .join(", ")
        ),
        Unsolvable => format!("{}", "The equation has no solutions".yellow()),
        InfiniteSolutions { exceptions } => match exceptions.is_empty() {
            true => format!("{variable_name} = R"),
            false => format!(
                "{variable_name} = R\\{{{}}}",
                exceptions
                    .iter()
                    .map(|exception| exception.to_string())
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
        },
    }
}

fn error_output(error: EquationError) -> String {
    match error {
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
        InvalidSeparatorAmount => String::from("An equation must contain exactly one equals sign"),
        InvalidOperation => String::from("Equation contains invalid operation"),
        DivisionByZero => String::from("Division by zero is undefined"),
        ComplexNumbers => String::from("Complex numbers are not supported"),
    }
}
