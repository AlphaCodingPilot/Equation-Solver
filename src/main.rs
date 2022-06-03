use colored::*;
use std::io;
use std::io::Write;
use std::process;

use equation_solver::EquationInput;
use equation_solver::EquationResult;
use equation_solver::EquationResult::*;

fn main() {
    let input = read_input();
    let equation_result = equation_solver::solve_equation(&input);
    output_result(equation_result, input.variable_name);
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

fn output_result(result: Result<EquationResult, String>, variable_name: String) {
    let output = match result {
        Ok(result) => match result {
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
            InfiniteSolutions { exceptions } => match exceptions == Vec::new() {
                true => format!("{variable_name} = {{ℝ}}"),
                false => format!(
                    "{variable_name} = {{ℝ}}/{{{}}}",
                    exceptions
                        .iter()
                        .map(|exception| exception.to_string())
                        .collect::<Vec<String>>()
                        .join(", ")
                ),
            },
        },
        Err(message) => {
            eprintln!("{}", message.red());
            process::exit(1);
        }
    };

    println!("{output}");
}
