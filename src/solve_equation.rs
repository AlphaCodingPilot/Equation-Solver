use crate::equation::Equation;
use crate::equation_error::EquationError;
use crate::equation_result::EquationResult;
use crate::token_stream::EquationInput;

pub fn solve_equation(input: &EquationInput) -> Result<EquationResult, EquationError> {
    let token_stream = input.token_stream()?;
    let equation = Equation::generate(token_stream)?;
    equation.set_zero().zeroes()
}
