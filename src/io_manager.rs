use colored::*;
use std::io;
use std::io::Write;

use crate::equation_error::EquationError;
use crate::equation_result::EquationResult;
use crate::token_stream::EquationInput;

pub fn read_input() -> EquationInput {
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

pub fn print_output(result: Result<EquationResult, EquationError>, variable_name: String) {
    let output = match result {
        Ok(success_result) => format!("{variable_name} = {success_result}"),
        Err(error) => format!("{}", error.log_message().red()),
    };
    println!("{output}");
}
