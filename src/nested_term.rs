use ordered_float::OrderedFloat;

use crate::equation_element::{
    AdditiveOperationType::{self, *},
    EquationElement::{self, *},
    MultiplicativeOperationType::{self, *},
    OperationType::{self, *},
    ValueType::{self, *},
};
use crate::equation_error::EquationError::{self, *};
use crate::equation_side::EquationSide;
use crate::term::Term;

pub struct NestedTerm {
    pub term: Term,
    pub multiplier: Term,
    additive_operation: AdditiveOperationType,
    multiplicative_operation: MultiplicativeOperationType,
}

impl NestedTerm {
    pub fn new() -> Self {
        Self {
            term: Term::new(),
            multiplier: Term::new_multiplier(),
            additive_operation: Addition,
            multiplicative_operation: Multiplication,
        }
    }

    pub fn push_multiplier(&mut self) {
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

    pub fn multiply_value(&mut self, value: &ValueType) -> Result<(), EquationError> {
        match value {
            Constant(constant) => match self.multiplicative_operation {
                Multiplication => {
                    for (_, coefficient) in self.multiplier.addends.iter_mut() {
                        *coefficient *= constant
                    }
                }
                Division => {
                    if *constant == 0.0 {
                        return Err(DivisionByZero);
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

    pub fn merge_term(
        &mut self,
        term: &Term,
        other_equation_side: &mut EquationSide,
    ) -> Result<(), EquationError> {
        match self.multiplicative_operation {
            Multiplication => {
                self.multiplier.multiply_term(&term);
            }
            Division => {
                self.term.multiply_term(term);
                other_equation_side.multiplier.multiply_term(term);

                self.multiplier.add_exceptions_in_domain_of_divisor(term)?;
            }
        }
        Ok(())
    }

    pub fn set_operation(
        &mut self,
        operation: OperationType,
        previous_element: &EquationElement,
    ) -> Result<(), EquationError> {
        match operation {
            AdditiveOperation(additive_operation) => {
                if let Operation(_) = *previous_element {
                    return Err(MissingOperation);
                }
                if let Value(_) | ClosingParenthesis = *previous_element {
                    self.push_multiplier();
                    self.multiplicative_operation = Multiplication;
                }
                self.additive_operation = additive_operation;
            }
            MultiplicativeOperation(multiplicative_operation) => {
                if let Value(_) | ClosingParenthesis = *previous_element {
                    self.multiplicative_operation = multiplicative_operation;
                } else {
                    return Err(InvalidOperation);
                }
            }
        }
        Ok(())
    }
}
