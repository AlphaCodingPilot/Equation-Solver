use crate::equation_error::EquationError::*;
use crate::equation_result::EquationResult::*;
use crate::solve_equation;
use crate::token_stream::EquationInput;

const TOLERANCE: f64 = 0.001;

macro_rules! test_solutions {
    ($equation:expr) => {
        let input = EquationInput::new(String::from($equation), String::from("x"));
        let equation_result = solve_equation::solve_equation(&input);
        assert_eq!(equation_result, Ok(Unsolvable), "Equation '{}' should be unsolvable but the outcome of the equation is {:?}", $equation, equation_result);
    };
    ($equation:expr, $($solution:expr),*) => {
        let input = EquationInput::new(String::from($equation), String::from("x"));
        let mut expected_solutions = Vec::new();
        $(
            expected_solutions.push($solution as f64);
        )*
        let actual_solutions = match solve_equation::solve_equation(&input) {
            Ok(solution) => match solution {
                Solutions(solutions) => solutions,
                Unsolvable => panic!("Equation '{}' is unsolvable but it should have solutions: {:?}", $equation, expected_solutions),
                InfiniteSolutions { .. } => panic!("Equation '{}' has infinite solutions but it should have finite solutions: {:?}", $equation, expected_solutions),
            }
            Err(error) => panic!("Equation '{}' should have solutions {:?} but an error occurred: {}", $equation, expected_solutions, error.log_message())
        };
        assert_eq!(expected_solutions.len(), actual_solutions.len(), "Equation '{}' has {} solutions {:?} but it should have {} solutions {:?}", $equation, actual_solutions.len(), actual_solutions, expected_solutions.len(), expected_solutions);
        for i in 0..expected_solutions.len() {
            assert!((expected_solutions[i] - actual_solutions[i]).abs() < TOLERANCE, "Solution {} in Equation '{}' should be within tolerance range ({}) of {} but it is actually {}", i, $equation, TOLERANCE, expected_solutions[i], actual_solutions[i]);
        }
    };
}

macro_rules! test_error {
    ($equation:expr, $error:expr) => {
        let input = EquationInput::new(String::from($equation), String::from("x"));
        test_error_from_input!(input, $equation, $error);
    };
    ($equation:expr, $variable_name:expr, $error:expr) => {
        let input = EquationInput::new(String::from($equation), String::from($variable_name));
        test_error_from_input!(input, $equation, $error);
    };
}

macro_rules! test_error_from_input {
    ($input:expr, $equation:expr, $error:expr) => {
        let equation_result = solve_equation::solve_equation(&$input);
        assert_eq!(
            equation_result,
            Err($error),
            "Equation '{}' should return error: {:?} but the outcome of the equation is {:?}",
            $equation,
            $error,
            equation_result
        );
    };
}

macro_rules! test_exceptions {
    ($equation:expr) => {
        let input = EquationInput::new(String::from($equation), String::from("x"));
        let equation_result = solve_equation::solve_equation(&input);
        assert_eq!(equation_result, Ok(InfiniteSolutions { exceptions: Vec::new() }), "Equation '{}' should have infinite solutions with no exceptions but the outcome of the equation is {:?}", $equation, equation_result);
    };
    ($equation:expr, $($exception:expr),*) => {
        let input = EquationInput::new(String::from($equation), String::from("x"));
        let mut expected_exceptions = Vec::new();
        $(
            expected_exceptions.push($exception as f64);
        )*
        let actual_exceptions = match solve_equation::solve_equation(&input) {
            Ok(solution) => match solution {
                Solutions(solutions) => panic!("Equation '{}' has solutions {:?} but it should have infinite solutions with exceptions {:?}", $equation, solutions, expected_exceptions),
                Unsolvable => panic!("Equation '{}' is unsolvable but it should have infinite solutions with exceptions {:?}", $equation, expected_exceptions),
                InfiniteSolutions { exceptions } => exceptions,
            }
            Err(error) => panic!("Equation '{}' should have infinite solutions with exception {:?} but an error occurred: {}", $equation, expected_exceptions, error.log_message())
        };
        assert_eq!(expected_exceptions.len(), actual_exceptions.len(), "Equation '{}' has {} exceptions {:?} but it should have {} exceptions {:?}", $equation, actual_exceptions.len(), actual_exceptions, expected_exceptions.len(), expected_exceptions);
        for i in 0..expected_exceptions.len() {
            assert!((expected_exceptions[i] - actual_exceptions[i]).abs() < TOLERANCE, "Solution {} in Equation '{}' should be within tolerance range ({}) of {} but it is actually {}", i, $equation, TOLERANCE, expected_exceptions[i], actual_exceptions[i]);
        }
    };
}

#[test]
fn equation_error() {
    test_error!("", EmptyEquation);
    test_error!("x + 1 = 2", "", EmptyVariableName);
    test_error!("1 + 2 = 3", NoOccurrencesOfVariable);
    test_error!(
        "1 + 3some_element = x",
        InvalidElement(String::from("some_element"))
    );
    test_error!("3 + x 5 = 2", MissingOperation);
    test_error!("3(x+(1+2) = 4", ParenthesisError);
    test_error!("3 + = x", InvalidSeparator);
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
    let input = EquationInput::new(String::from("2variable + 4 = 8"), String::from("variable"));
    assert_eq!(
        solve_equation::solve_equation(&input),
        Ok(Solutions(vec![2.0]))
    );
}

#[test]
fn quadratic_equation() {
    test_solutions!("3x*x + 6x = 9", 1, -3);
    test_solutions!("πx + 2e = pi/x", 0.45711609753521537, -2.1876280563997454);
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
