use ordered_float::OrderedFloat;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

use AdditiveOperationType::*;
use EquationElement::*;
use EquationResult::*;
use EquationSideType::*;
use MultiplicativeOperationType::*;
use OperationType::*;
use ValueType::*;

const MAX_DEGREE: i32 = 2;

pub fn solve_equation(input: &EquationInput) -> Result<EquationResult, String> {
    let mut equation = Equation::new(input.elements()?)?;
    equation.solve()
}

pub struct EquationInput {
    equation: String,
    pub variable_name: String,
}

impl EquationInput {
    pub fn new(equation: String, variable_name: String) -> Self {
        Self {
            equation,
            variable_name,
        }
    }

    fn elements(&self) -> Result<Vec<EquationElement>, String> {
        if self.equation.is_empty() {
            return Err(String::from("Equation is empty"));
        }
        if self.variable_name.is_empty() {
            return Err(String::from("Variable name was not specified"));
        }

        let mut elements = Vec::new();
        let mut current_value = String::new();
        for element in self.equation.chars() {
            let element = match element {
                ' ' => {
                    add_value_to_elements(&mut elements, &mut current_value, &self.variable_name)?;
                    continue;
                }
                '+' => Operation(AdditiveOperation(Addition)),
                '-' => Operation(AdditiveOperation(Subtraction)),
                '*' => Operation(MultiplicativeOperation(Multiplication)),
                '/' => Operation(MultiplicativeOperation(Division)),
                '=' => Separator,
                '(' => OpeningParenthesis,
                ')' => ClosingParenthesis,
                _ => {
                    current_value.push(element);
                    continue;
                }
            };
            add_value_to_elements(&mut elements, &mut current_value, &self.variable_name)?;
            elements.push(element);
        }
        add_value_to_elements(&mut elements, &mut current_value, &self.variable_name)?;
        Ok(elements)
    }
}

fn add_value_to_elements(
    elements: &mut Vec<EquationElement>,
    value: &mut String,
    variable_name: &str,
) -> Result<(), String> {
    if value.is_empty() {
        return Ok(());
    }
    let element = Value(match *value == variable_name {
        true => Variable,
        false => match value.parse::<f64>() {
            Ok(value) => Constant(value),
            Err(_) => {
                return Err(format!(
                    "The equation contains an invalid element: {}",
                    value
                ));
            }
        },
    });
    elements.push(element);
    *value = String::new();
    Ok(())
}

#[derive(PartialEq, Debug)]
pub enum EquationResult {
    Solutions(Vec<f64>),
    Unsolvable,
    InfiniteSolutions { exceptions: Vec<f64> },
}

#[derive(Clone)]
enum EquationElement {
    Value(ValueType),
    Operation(OperationType),
    Separator,
    OpeningParenthesis,
    ClosingParenthesis,
}

#[derive(Clone)]
enum ValueType {
    Constant(f64),
    Variable,
}

#[derive(Clone)]
enum OperationType {
    AdditiveOperation(AdditiveOperationType),
    MultiplicativeOperation(MultiplicativeOperationType),
}

#[derive(Clone)]
enum AdditiveOperationType {
    Addition,
    Subtraction,
}

#[derive(Clone)]
enum MultiplicativeOperationType {
    Multiplication,
    Division,
}

struct Equation {
    left_hand_side: Term,
    right_hand_side: Term,
}

impl Equation {
    fn new(elements: Vec<EquationElement>) -> Result<Self, String> {
        let mut nested_terms = VecDeque::new();
        let mut left_hand_side = EquationSide::new(LeftHandSide);
        let mut right_hand_side = EquationSide::new(RightHandSide);
        let mut current_equation_side = &mut left_hand_side;
        let mut other_equation_side = &mut right_hand_side;
        let mut current_nested_term = NestedTerm::new();
        let mut previous_element = Separator;

        for element in elements {
            match element.clone() {
                Value(value) => {
                    if let Value(_) | ClosingParenthesis = previous_element {
                        return Err(String::from("The equation contains two continuos values"));
                    }
                    current_nested_term.multiply_value(&value)?;
                }
                Operation(operation) => {
                    evaluate_operation(operation, &mut current_nested_term, &previous_element)?
                }
                Separator => {
                    if let Value(_) | ClosingParenthesis = previous_element {
                        if let RightHandSide = current_equation_side.side {
                            return Err(String::from(
                                "The equation contains more than one equals sign",
                            ));
                        }
                        if !nested_terms.is_empty() {
                            return Err(String::from(
                                "The equation is missing closing parenthesis",
                            ));
                        }

                        current_nested_term.push_multiplier();
                        current_equation_side.term = current_nested_term.term;
                        current_equation_side = &mut right_hand_side;
                        other_equation_side = &mut left_hand_side;
                        current_nested_term = NestedTerm::new();
                    } else {
                        return Err(String::from("The equation contains an invalid equals sign"));
                    }
                }
                OpeningParenthesis => {
                    if let Value(_) | ClosingParenthesis = previous_element {
                        return Err(String::from("The equation contains two continuos values"));
                    }
                    nested_terms.push_back(current_nested_term);
                    current_nested_term = NestedTerm::new();
                }
                ClosingParenthesis => {
                    if let Value(_) | ClosingParenthesis = previous_element {
                        current_nested_term.push_multiplier();
                        let mut nested_term = nested_terms
                            .pop_back()
                            .ok_or("Equation is missing opening parenthesis")?;

                        match nested_term.multiplicative_operation {
                            Multiplication => {
                                nested_term
                                    .multiplier
                                    .multiply_term(&current_nested_term.term);
                            }
                            Division => {
                                nested_term.term.multiply_term(&current_nested_term.term);
                                other_equation_side
                                    .multiplier
                                    .multiply_term(&current_nested_term.term);

                                nested_term.multiplier.add_exceptions_in_domain_of_divisor(
                                    &current_nested_term.term,
                                )?;
                            }
                        }
                        current_nested_term = nested_term;
                    } else {
                        return Err(String::from(
                            "Equation contains invalid closing parenthesis",
                        ));
                    }
                }
            }
            previous_element = element;
        }

        if let LeftHandSide = current_equation_side.side {
            return Err(String::from("The equation contains no equals sign"));
        }
        if !nested_terms.is_empty() {
            return Err(String::from("The equation is missing closing parenthesis"));
        }

        current_nested_term.push_multiplier();
        current_equation_side.term = current_nested_term.term;

        left_hand_side.push_multiplier();
        right_hand_side.push_multiplier();

        Ok(Self {
            left_hand_side: left_hand_side.term,
            right_hand_side: right_hand_side.term,
        })
    }

    fn solve(&mut self) -> Result<EquationResult, String> {
        self.set_zero().zeroes()
    }

    fn set_zero(&self) -> Term {
        let mut term = self.left_hand_side.clone();
        for (exponent, coefficient) in self.right_hand_side.addends.iter() {
            *term.addends.entry(*exponent).or_insert(0.0) -= coefficient;
        }
        term
    }
}

fn evaluate_operation(
    operation: OperationType,
    nested_term: &mut NestedTerm,
    previous_element: &EquationElement,
) -> Result<(), String> {
    match operation {
        AdditiveOperation(additive_operation) => {
            if let Operation(_) = *previous_element {
                return Err(String::from(
                    "The equation contains two continuos operations",
                ));
            }
            if let Value(_) | ClosingParenthesis = *previous_element {
                nested_term.push_multiplier();
                nested_term.multiplicative_operation = Multiplication;
            }
            nested_term.additive_operation = additive_operation;
        }
        MultiplicativeOperation(multiplicative_operation) => {
            if let Value(_) | ClosingParenthesis = *previous_element {
                nested_term.multiplicative_operation = multiplicative_operation;
            } else {
                return Err(String::from("The equation contains an invalid operation"));
            }
        }
    }
    Ok(())
}

struct EquationSide {
    term: Term,
    multiplier: Term,
    side: EquationSideType,
}

impl EquationSide {
    fn new(side: EquationSideType) -> Self {
        Self {
            term: Term::new(),
            multiplier: Term::new_multiplier(),
            side,
        }
    }

    fn push_multiplier(&mut self) {
        self.term.multiply_term(&self.multiplier);
        self.multiplier = Term::new_multiplier();
    }
}

enum EquationSideType {
    LeftHandSide,
    RightHandSide,
}

struct NestedTerm {
    term: Term,
    multiplier: Term,
    additive_operation: AdditiveOperationType,
    multiplicative_operation: MultiplicativeOperationType,
}

impl NestedTerm {
    fn new() -> Self {
        Self {
            term: Term::new(),
            multiplier: Term::new_multiplier(),
            additive_operation: Addition,
            multiplicative_operation: Multiplication,
        }
    }

    fn push_multiplier(&mut self) {
        for (exponent, other_coefficient) in self.multiplier.addends.iter() {
            let coefficient = self.term.addends.entry(*exponent).or_insert(0.0);
            match self.additive_operation {
                Addition => *coefficient += other_coefficient,
                Subtraction => *coefficient -= other_coefficient,
            }
        }

        self.term
            .exceptions_in_domain
            .extend(self.multiplier.exceptions_in_domain.iter());
        self.multiplier = Term::new_multiplier();
    }

    fn multiply_value(&mut self, value: &ValueType) -> Result<(), String> {
        match value {
            Constant(constant) => match self.multiplicative_operation {
                Multiplication => {
                    for (_, coefficient) in self.multiplier.addends.iter_mut() {
                        *coefficient *= constant
                    }
                }
                Division => {
                    if *constant == 0.0 {
                        return Err(String::from("Division by zero is undefined"));
                    }
                    for (_, coefficient) in self.multiplier.addends.iter_mut() {
                        *coefficient /= constant
                    }
                }
            },
            Variable => match self.multiplicative_operation {
                Multiplication => self.multiplier.increase_exponents(1),
                Division => {
                    self.multiplier.increase_exponents(-1);
                    self.multiplier
                        .exceptions_in_domain
                        .insert(OrderedFloat(0.0));
                }
            },
        }
        Ok(())
    }
}

#[derive(Clone)]
struct Term {
    addends: HashMap<i32, f64>,
    exceptions_in_domain: HashSet<OrderedFloat<f64>>,
}

impl Term {
    fn new() -> Self {
        Self {
            addends: HashMap::new(),
            exceptions_in_domain: HashSet::new(),
        }
    }

    fn new_multiplier() -> Self {
        Self {
            addends: HashMap::from([(0, 1.0)]),
            exceptions_in_domain: HashSet::new(),
        }
    }

    fn zeroes(&self) -> Result<EquationResult, String> {
        let mut normalized_term = self.clone();
        let lowest_exponent = self.lowest_exponent();
        normalized_term.increase_exponents(-lowest_exponent);

        let degree = match normalized_term.degree() {
            None => {
                let mut exceptions = self
                    .exceptions_in_domain
                    .iter()
                    .collect::<Vec<&OrderedFloat<f64>>>();
                exceptions.sort();
                let exceptions = exceptions
                    .iter()
                    .rev()
                    .map(|value| value.into_inner())
                    .collect();
                return Ok(InfiniteSolutions { exceptions });
            }
            Some(0) => {
                return Ok(
                    match lowest_exponent > 0
                        && !self.exceptions_in_domain.contains(&OrderedFloat(0.0))
                    {
                        true => Solutions(vec![0.0]),
                        false => Unsolvable,
                    },
                )
            }
            Some(degree) => degree,
        };

        if degree > MAX_DEGREE {
            return Err(format!(
                "Polynomial equations with a degree greater than {MAX_DEGREE} are not supported"
            ));
        }

        let mut solutions = Vec::new();
        if lowest_exponent > 0 && !self.exceptions_in_domain.contains(&OrderedFloat(0.0)) {
            solutions.push(0.0);
        }

        let mut roots = normalized_term
            .roots()?
            .iter()
            .filter(|value| !self.exceptions_in_domain.contains(&OrderedFloat(**value)))
            .copied()
            .collect();

        solutions.append(&mut roots);
        Ok(Solutions(solutions))
    }

    fn roots(&self) -> Result<Vec<f64>, String> {
        let a = *self.addends.get(&2).unwrap_or(&0.0);
        let b = *self.addends.get(&1).unwrap_or(&0.0);
        let c = *self.addends.get(&0).unwrap_or(&0.0);

        if a == 0.0 {
            return Ok(vec![-c / b]);
        }

        if b * b - 4.0 * a * c < 0.0 {
            return Err(String::from("Complex numbers are not supported"));
        }

        Ok(vec![
            (-b + (b * b - 4.0 * a * c).sqrt()) / (2.0 * a),
            (-b - (b * b - 4.0 * a * c).sqrt()) / (2.0 * a),
        ])
    }

    fn multiply_term(&mut self, other: &Term) {
        let mut addends = HashMap::new();
        for (exponent, coefficient) in self.addends.iter() {
            for (other_exponent, other_coefficient) in other.addends.iter() {
                *addends.entry(exponent + other_exponent).or_insert(0.0) +=
                    coefficient * other_coefficient;
            }
        }

        self.addends = addends;
    }

    fn increase_exponents(&mut self, power: i32) {
        let new_addends = self
            .addends
            .iter()
            .map(|(exponent, coefficient)| (exponent + power, *coefficient))
            .collect();

        self.addends = new_addends;
    }

    fn add_exceptions_in_domain_of_divisor(&mut self, divisor: &Term) -> Result<(), String> {
        let exceptions_in_domain = match divisor.zeroes()? {
            Solutions(values) => values.into_iter().map(OrderedFloat).collect(),
            Unsolvable => HashSet::new(),
            InfiniteSolutions { .. } => return Err(String::from("Division by zero is undefined")),
        };

        self.exceptions_in_domain
            .extend(exceptions_in_domain.iter());

        Ok(())
    }

    fn degree(&self) -> Option<i32> {
        let mut degree = None;
        for (exponent, coefficient) in self.addends.iter() {
            if match degree {
                Some(lowest_degree) => *exponent > lowest_degree,
                None => true,
            } && *coefficient != 0.0
            {
                degree = Some(*exponent);
            }
        }
        degree
    }

    fn lowest_exponent(&self) -> i32 {
        let mut lowest_degree = None;
        for (exponent, coefficient) in self.addends.iter() {
            if match lowest_degree {
                Some(lowest_degree) => *exponent < lowest_degree,
                None => true,
            } && *coefficient != 0.0
            {
                lowest_degree = Some(*exponent);
            }
        }
        lowest_degree.unwrap_or_default()
    }
}

#[cfg(test)]
mod tests;
