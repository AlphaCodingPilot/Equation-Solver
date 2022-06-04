use crate::equation::Equation;
use crate::equation_error::EquationError;
use crate::equation_input::EquationInput;
use crate::equation_result::EquationResult;

pub fn solve_equation(input: &EquationInput) -> Result<EquationResult, EquationError> {
    let mut equation = Equation::new(input.elements()?)?;
    equation.solve()
}
