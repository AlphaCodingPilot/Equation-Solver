use crate::equation_error::EquationError::*;
use crate::equation_result::EquationResult::*;
use crate::solve_equation;
use crate::token_stream::EquationInput;

macro_rules! test_solutions {
    ($equation:expr) => {
        let input = EquationInput::new(String::from($equation), String::from("x"));
        assert_eq!(solve_equation::solve_equation(&input), Ok(Unsolvable));
    };
    ($equation:expr, $($solution:expr),*) => {
        let input = EquationInput::new(String::from($equation), String::from("x"));
        let mut solutions = Vec::new();
        $(
            solutions.push($solution as f64);
        )*
        assert_eq!(solve_equation::solve_equation(&input), Ok(Solutions(solutions)));
    };
}

macro_rules! test_error {
    ($equation:expr, $error:expr) => {
        let input = EquationInput::new(String::from($equation), String::from("x"));
        assert_eq!(solve_equation::solve_equation(&input), Err($error));
    };
    ($equation:expr, $variable_name:expr, $error:expr) => {
        let input = EquationInput::new(String::from($equation), String::from($variable_name));
        assert_eq!(solve_equation::solve_equation(&input), Err($error));
    };
}

macro_rules! test_exceptions {
    ($equation:expr) => {
        let input = EquationInput::new(String::from($equation), String::from("x"));
        assert_eq!(solve_equation::solve_equation(&input), Ok(InfiniteSolutions { exceptions: Vec::new() }));
    };
    ($equation:expr, $($exception:expr),*) => {
        let input = EquationInput::new(String::from($equation), String::from("x"));
        let mut exceptions = Vec::new();
        $(
            exceptions.push($exception as f64);
        )*
        assert_eq!(solve_equation::solve_equation(&input), Ok(InfiniteSolutions { exceptions }));
    };
}

#[test]
fn equation_error() {
    test_error!("", EmptyEquation);
    test_error!("x + 1 = 2", "", EmptyVariableName);
    test_error!("1 + 2 = 3", NoOccurrencesOfVariable);
    test_error!("1 + 3some_element = x", InvalidElement(String::from("some_element")));
    test_error!("3 + x 5 = 2", MissingOperation);
    test_error!("3(x+(1+2) = 4", ParenthesisError);
    test_error!("3 += x", InvalidSeparator);
    test_error!("3 = 1 + 2 = x", InvalidSeparatorAmount);
    test_error!("5 + * x = 8", InvalidOperation);
    test_error!("x = 1/0", DivisionByZero);
    test_error!("2x*x*x + 4 = 0", TooHighDegree { max_degree: 2 });
    test_error!("x*x = -1", ComplexNumbers);
}

#[test]
fn linear_equation() {
    test_solutions!("2x = 6", 3);
    test_solutions!("x + 1 = 2", 1);
    test_solutions!("2x(2+3)(5-4) = 2(20+5)", 5);
    test_solutions!("1/x + 3/(x*x) = 5/(x*x)", 2);
    test_solutions!("5x - 4x - 3 = -x + 5", 4);
}

#[test]
fn unsolvable_equation() {
    test_solutions!("x = x + 1");
    test_solutions!("1/(x*x*x+1) = 1/(x*x*x+1) + 1");
}

#[test]
fn infinite_solutions() {
    test_exceptions!("x + 3 = x + 3");
    test_exceptions!("x = 1/(1/x)", 0);
    test_exceptions!("1/((x-1)(x+1)) = 1/((x-1)(x+1))", 1, -1);
}

#[test]
fn different_variable_name() {
    let input = EquationInput::new(String::from("2unknown + 4 = 8"), String::from("unknown"));
    assert_eq!(
        solve_equation::solve_equation(&input),
        Ok(Solutions(vec![2.0]))
    );
}

#[test]
fn quadratic_equation() {
    test_solutions!("3x*x + 6x = 9", 1, -3);
}

#[test]
fn factorized_polynomial() {
    test_solutions!("2x*x = 6x", 0, 3);
    test_solutions!("2*x*x = 0", 0);
}

#[test]
fn order_of_operations() {
    test_solutions!("1 + 2*3 = x", 7);
    test_solutions!("x(1 + 2) = 9", 3);
}
