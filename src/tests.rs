use super::EquationInput;
use super::EquationResult::*;

#[test]
fn empty_input() {
    let input = EquationInput::new(String::new(), String::from("x"));
    assert!(super::solve_equation(&input).is_err());
}

#[test]
fn empty_variable_name() {
    let input = EquationInput::new(String::from("x + 1 = 2"), String::new());
    assert!(super::solve_equation(&input).is_err());
}

#[test]
fn simple_equation() {
    let input = EquationInput::new(String::from("x + 1 = 2"), String::from("x"));
    assert_eq!(super::solve_equation(&input), Ok(Solutions(vec![1.0])));
}

#[test]
fn linear_equation() {
    let input = EquationInput::new(String::from("2*x = 6"), String::from("x"));
    assert_eq!(super::solve_equation(&input), Ok(Solutions(vec![3.0])));
}

#[test]
fn unsolvable_equation() {
    let input = EquationInput::new(String::from("x = x + 1"), String::from("x"));
    assert_eq!(super::solve_equation(&input), Ok(Unsolvable));
}

#[test]
fn infinite_solutions() {
    let input = EquationInput::new(String::from("x + 3 = x + 3"), String::from("x"));
    assert_eq!(
        super::solve_equation(&input),
        Ok(InfiniteSolutions {
            exceptions: Vec::new()
        })
    );
}

#[test]
fn infinite_solutions_with_one_exception() {
    let input = EquationInput::new(String::from("1/x = 1/x"), String::from("x"));
    assert_eq!(
        super::solve_equation(&input),
        Ok(InfiniteSolutions {
            exceptions: vec![0.0]
        })
    );
}

#[test]
fn different_variable_name() {
    let input = EquationInput::new(String::from("2*unknown + 4 = 6"), String::from("unknown"));
    assert_eq!(super::solve_equation(&input), Ok(Solutions(vec![1.0])));
}

#[test]
fn different_spaces_in_input() {
    let input = EquationInput::new(String::from("x+2 +  3=7"), String::from("x"));
    assert_eq!(super::solve_equation(&input), Ok(Solutions(vec![2.0])))
}

#[test]
fn invalid_equation() {
    let input = EquationInput::new(String::from("x + * 1 = 2"), String::from("x"));
    assert!(super::solve_equation(&input).is_err());
}

#[test]
fn quadratic_equation() {
    let input = EquationInput::new(String::from("3*x*x + 6*x = 9"), String::from("x"));
    assert_eq!(
        super::solve_equation(&input),
        Ok(Solutions(vec![1.0, -3.0]))
    );
}

#[test]
fn factorized_polynomial() {
    let input = EquationInput::new(String::from("2*x*x = 6*x"), String::from("x"));
    assert_eq!(super::solve_equation(&input), Ok(Solutions(vec![0.0, 3.0])));
}

#[test]
fn factorized_polynomial_with_one_solution() {
    let input = EquationInput::new(String::from("2*x*x = 0"), String::from("x"));
    assert_eq!(super::solve_equation(&input), Ok(Solutions(vec![0.0])));
}

#[test]
fn fractions() {
    let input = EquationInput::new(String::from("1/x + 3/(x*x) = 5/(x*x)"), String::from("x"));
    assert_eq!(super::solve_equation(&input), Ok(Solutions(vec![2.0])));
}

#[test]
fn subtraction_of_products() {
    let input = EquationInput::new(
        String::from("5 * x - 4 * x - 3 = - x + 5"),
        String::from("x"),
    );
    assert_eq!(super::solve_equation(&input), Ok(Solutions(vec![4.0])));
}

#[test]
fn order_of_operations() {
    let input = EquationInput::new(String::from("1 + 2 * 3 = x"), String::from("x"));
    assert_eq!(super::solve_equation(&input), Ok(Solutions(vec![7.0])));
}

#[test]
fn order_of_operations_with_parenthesis() {
    let input = EquationInput::new(String::from("x * (1 + 2) = 9"), String::from("x"));
    assert_eq!(super::solve_equation(&input), Ok(Solutions(vec![3.0])));
}

#[test]
fn domain_of_division() {
    let input = EquationInput::new(
        String::from("1/((x-1)*(x+1)) = 1/((x-1)*(x+1))"),
        String::from("x"),
    );
    assert_eq!(
        super::solve_equation(&input),
        Ok(InfiniteSolutions {
            exceptions: vec![1.0, -1.0]
        })
    )
}

#[test]
fn invalid_parenthesis() {
    let input = EquationInput::new(String::from("x + (x * (2*3)"), String::from("x"));
    assert!(super::solve_equation(&input).is_err());
}