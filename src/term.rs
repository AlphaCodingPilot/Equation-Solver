use ordered_float::OrderedFloat;
use std::collections::HashMap;
use std::collections::HashSet;

use crate::equation_error::EquationError::{self, *};
use crate::equation_result::EquationResult::{self, *};

const MAX_DEGREE: i32 = 2;

#[derive(Clone)]
pub struct Term {
    pub addends: HashMap<i32, f64>,
    pub exceptions_in_domain: HashSet<OrderedFloat<f64>>,
}

impl Term {
    pub fn new() -> Self {
        Self {
            addends: HashMap::new(),
            exceptions_in_domain: HashSet::new(),
        }
    }

    pub fn new_multiplier() -> Self {
        Self {
            addends: HashMap::from([(0, 1.0)]),
            exceptions_in_domain: HashSet::new(),
        }
    }

    pub fn zeroes(&self) -> Result<EquationResult, EquationError> {
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
            return Err(TooHighDegree {
                max_degree: MAX_DEGREE,
            });
        }

        let mut solutions = Vec::new();
        if lowest_exponent > 0 && !self.exceptions_in_domain.contains(&OrderedFloat(0.0)) {
            solutions.push(0.0);
        }

        let mut roots = normalized_term
            .roots()?
            .iter()
            .map(|value| OrderedFloat(*value))
            .filter(|value| !self.exceptions_in_domain.contains(value))
            .collect::<Vec<OrderedFloat<f64>>>();

        roots.sort();

        let mut roots = roots.iter().rev().map(|value| value.into_inner()).collect();

        solutions.append(&mut roots);
        Ok(Solutions(solutions))
    }

    fn roots(&self) -> Result<Vec<f64>, EquationError> {
        let a = *self.addends.get(&2).unwrap_or(&0.0);
        let b = *self.addends.get(&1).unwrap_or(&0.0);
        let c = *self.addends.get(&0).unwrap_or(&0.0);

        if a == 0.0 {
            return Ok(vec![-c / b]);
        }

        if b * b - 4.0 * a * c < 0.0 {
            return Err(ComplexNumbers);
        }

        Ok(vec![
            (-b + (b * b - 4.0 * a * c).sqrt()) / (2.0 * a),
            (-b - (b * b - 4.0 * a * c).sqrt()) / (2.0 * a),
        ])
    }

    pub fn multiply_term(&mut self, other: &Term) {
        let mut addends = HashMap::new();
        for (exponent, coefficient) in self.addends.iter() {
            for (other_exponent, other_coefficient) in other.addends.iter() {
                *addends.entry(exponent + other_exponent).or_insert(0.0) +=
                    coefficient * other_coefficient;
            }
        }

        self.addends = addends;
    }

    pub fn increase_exponents(&mut self, power: i32) {
        let new_addends = self
            .addends
            .iter()
            .map(|(exponent, coefficient)| (exponent + power, *coefficient))
            .collect();

        self.addends = new_addends;
    }

    pub fn add_exceptions_in_domain_of_divisor(
        &mut self,
        divisor: &Term,
    ) -> Result<(), EquationError> {
        let exceptions_in_domain = match divisor.zeroes()? {
            Solutions(values) => values.into_iter().map(OrderedFloat).collect(),
            Unsolvable => HashSet::new(),
            InfiniteSolutions { .. } => return Err(DivisionByZero),
        };

        self.exceptions_in_domain
            .extend(exceptions_in_domain.iter());

        Ok(())
    }

    pub fn degree(&self) -> Option<i32> {
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

    pub fn lowest_exponent(&self) -> i32 {
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
