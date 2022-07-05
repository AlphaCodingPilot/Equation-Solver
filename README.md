# Equation Solver
A equation solver as an API or usable as a cli application. Written in Rust.

![GitHub last commit](https://img.shields.io/github/last-commit/AlphaCodingPilot/equation-solver) ![GitHub issues](https://img.shields.io/github/issues-raw/AlphaCodingPilot/equation-solver) ![Lines of code](https://img.shields.io/tokei/lines/github/AlphaCodingPilot/equation-solver?label=lines%20of%20code)

```
Enter equation ← Output
-6 + 9/x = 3x ← Input

Output    Input
↓         ↓
Solve for x
x = {1, -3} ← Output
```

## Installation
Clone the repository:
```
git clone https://github.com/AlphaCodingPilot/Equation-Solver.git
```
## Usage (terminal)
1. Run the main.rs file in the src directory.
2. Enter an equation in the terminal.
3. Specify the variable you want to solve for after 'Solve for '

## Usage (API)
1. Create an instance of equation_input::EquationInput using the equation_input::EquationInput::new(equation, variable_name) function, parsing the equation and the variable name you want to solve for as strings.
2. Use the solve_equation::solve_equation(input) function parsing the previously created equation-input. This returns a Result<equation_result::EquationResult, equation_error::EquationError> type.
3. Evaluate the result: the equation_result::EquationResult type has three variants: Solutions(Vec<f64>), Unsolvable, InfiniteSolutions { exceptions: Vec<f64>}. You can evaluate each of the equation_error::EquationError variants manually or get a log-message using the log_message() method.

## Contributing
Pull requests are welcome. Before doing major changes please open a pull request first to discus it.
