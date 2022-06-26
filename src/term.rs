use ordered_float::OrderedFloat;
use std::collections::HashMap;
use std::collections::HashSet;

use crate::equation_element::{
    SymbolType::*,
    ValueType::{self, *},
};
use crate::equation_error::EquationError::{self, *};
use crate::equation_result::EquationResult::{self, *};
use crate::equation_side::EquationSide;
use crate::exceptions_in_domain::ExceptionsInDomain::{self, *};

const MAX_DEGREE: i32 = 2;

#[derive(Clone, Debug)]
pub struct Term {
    pub addends: HashMap<i32, f64>,
    pub exceptions_in_domain: ExceptionsInDomain,
}

impl Term {
    pub fn new() -> Self {
        Self {
            addends: HashMap::new(),
            exceptions_in_domain: Known(HashSet::new()),
        }
    }

    pub fn new_multiplier() -> Self {
        Self {
            addends: HashMap::from([(0, 1.0)]),
            exceptions_in_domain: Known(HashSet::new()),
        }
    }

    pub fn zeroes(&self) -> Result<EquationResult, EquationError> {
        let lowest_exponent = self.lowest_exponent();
        let factorized_variable = lowest_exponent > 0;
        let mut normalized_term = self.clone();
        normalized_term.increase_exponents(-lowest_exponent);

        let degree = normalized_term.degree();
        let degree = match degree {
            None => {
                let exceptions = self
                    .exceptions_in_domain
                    .unwrap_or(|degree| TooHighDegree {
                        degree,
                        max_degree: MAX_DEGREE,
                    })?;
                let mut exceptions = exceptions.iter().collect::<Vec<&OrderedFloat<f64>>>();

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
                    match factorized_variable && self.exceptions_in_domain.zero_is_valid() {
                        true => Solutions(vec![0.0]),
                        false => Unsolvable,
                    },
                )
            }
            Some(degree) => degree,
        };

        if degree > MAX_DEGREE {
            return Err(TooHighDegree {
                degree,
                max_degree: MAX_DEGREE,
            });
        }

        let exceptions_in_domain = self
            .exceptions_in_domain
            .unwrap_or(|degree| TooHighDegree {
                degree,
                max_degree: MAX_DEGREE,
            })?;

        let mut solutions = Vec::new();
        if factorized_variable && !exceptions_in_domain.contains(&OrderedFloat(0.0)) {
            solutions.push(0.0);
        }

        let mut roots = normalized_term
            .roots()?
            .iter()
            .map(|value| OrderedFloat(*value))
            .filter(|value| !exceptions_in_domain.contains(value))
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

    pub fn multiply_value(&mut self, value: &ValueType) {
        match value {
            Number(constant) => self.multiply_constant(constant),
            Symbol(symbol) => match symbol {
                Variable => self.increase_exponents(1),
                Constant(constant) => self.multiply_constant(constant),
            },
        }
    }

    pub fn divide_value(&mut self, value: &ValueType) -> Result<(), EquationError> {
        match value {
            Number(constant) => self.divide_constant(constant)?,
            Symbol(symbol) => match symbol {
                Variable => {
                    self.increase_exponents(-1);
                    self.exceptions_in_domain.insert_zero();
                }
                Constant(constant) => self.divide_constant(constant)?,
            },
        }
        Ok(())
    }

    pub fn multiply_constant(&mut self, constant: &f64) {
        for (_, coefficient) in self.addends.iter_mut() {
            *coefficient *= constant;
        }
    }

    pub fn divide_constant(&mut self, constant: &f64) -> Result<(), EquationError> {
        if *constant == 0.0 {
            return Err(DivisionByZero);
        }

        for (_, coefficient) in self.addends.iter_mut() {
            *coefficient /= constant;
        }
        Ok(())
    }

    pub fn increase_exponents(&mut self, power: i32) {
        let new_addends = self
            .addends
            .iter()
            .map(|(exponent, coefficient)| (exponent + power, *coefficient))
            .collect();

        self.addends = new_addends;
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

    pub fn divide_term(
        &mut self,
        other: &Term,
        nested_term: &mut Term,
        equation_side_multiplier: &mut Term,
        other_equation_side: &mut EquationSide,
    ) -> Result<(), EquationError> {
        self.add_exceptions_in_domain_of_divisor(other)?;
        let addends = other
            .addends
            .iter()
            .filter(|(_, coefficient)| **coefficient != 0.0)
            .collect::<Vec<(&i32, &f64)>>();
        if addends.len() == 1 {
            let (exponent, coefficient) = addends[0];
            self.increase_exponents(-exponent);
            self.divide_constant(coefficient).unwrap();
            return Ok(());
        }
        nested_term.multiply_term(other);
        equation_side_multiplier.multiply_term(other);
        other_equation_side.multiplier.multiply_term(other);
        Ok(())
    }

    pub fn add_exceptions_in_domain_of_divisor(
        &mut self,
        divisor: &Term,
    ) -> Result<(), EquationError> {
        match &mut self.exceptions_in_domain {
            Known(exceptions) => {
                let exceptions_in_domain = match divisor.zeroes() {
                    Ok(Solutions(values)) => values.into_iter().map(OrderedFloat).collect(),
                    Ok(Unsolvable) => HashSet::new(),
                    Ok(InfiniteSolutions { .. }) => return Err(DivisionByZero),
                    Err(TooHighDegree { degree, .. }) => {
                        let zero_is_valid = !exceptions.contains(&OrderedFloat(0.0))
                            && !divisor.zero_is_a_solution();
                        self.exceptions_in_domain = Unknown {
                            zero_is_valid,
                            degree,
                        };
                        return Ok(());
                    }
                    Err(error) => return Err(error),
                };

                exceptions.extend(exceptions_in_domain.iter());
            }
            Unknown { zero_is_valid, .. } => {
                if !divisor.zero_is_a_solution() {
                    *zero_is_valid = false;
                }
            }
        }
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

    pub fn zero_is_a_solution(&self) -> bool {
        self.lowest_exponent() > 0
    }
}

impl PartialEq for Term {
    fn eq(&self, other: &Self) -> bool {
        self.addends == other.addends
    }
}
