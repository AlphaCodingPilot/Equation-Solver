use ordered_float::OrderedFloat;
use std::collections::HashSet;

use crate::equation_error::EquationError;

use ExceptionsInDomain::*;

#[derive(Clone, Debug)]
pub enum ExceptionsInDomain {
    Known(HashSet<OrderedFloat<f64>>),
    Unknown { zero_is_valid: bool },
}

impl ExceptionsInDomain {
    pub fn extend(&mut self, other: &ExceptionsInDomain) {
        match self {
            Known(exceptions) => match other {
                Known(other_exceptions) => exceptions.extend(other_exceptions.iter()),
                Unknown { zero_is_valid } => {
                    if *zero_is_valid {
                        exceptions.insert(OrderedFloat(0.0));
                    }
                }
            },
            Unknown { zero_is_valid } => {
                if !other.zero_is_valid() {
                    *zero_is_valid = false;
                }
            }
        }
    }

    pub fn insert_zero(&mut self) {
        match self {
            Known(exceptions) => {
                exceptions.insert(OrderedFloat(0.0));
            }
            Unknown { zero_is_valid } => *zero_is_valid = false,
        }
    }

    pub fn unwrap_or(
        &self,
        error: EquationError,
    ) -> Result<HashSet<OrderedFloat<f64>>, EquationError> {
        match self {
            Known(exceptions) => Ok(exceptions.to_owned()),
            Unknown { .. } => Err(error),
        }
    }

    pub fn zero_is_valid(&self) -> bool {
        match self {
            Known(exceptions) => !exceptions.contains(&OrderedFloat(0.0)),
            Unknown { zero_is_valid } => *zero_is_valid,
        }
    }
}
