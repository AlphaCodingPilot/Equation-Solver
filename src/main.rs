mod equation;
mod equation_element;
mod equation_error;
mod equation_result;
mod equation_side;
mod exceptions_in_domain;
mod io_manager;
mod nested_term;
mod solve_equation;
mod term;
#[cfg(test)]
mod tests;
mod token_stream;

fn main() {
    let input = io_manager::read_input();
    let equation_result = solve_equation::solve_equation(&input);
    io_manager::print_output(equation_result, input.variable_name);
}
