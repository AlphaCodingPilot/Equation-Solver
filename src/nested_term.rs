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
            .extend(&self.multiplier.exceptions_in_domain);

        self.multiplier = Term::new_multiplier();
    }

    pub fn multiply_value(
        &mut self,
        value: &ValueType,
        previous_element: &EquationElement,
    ) -> Result<(), EquationError> {
        match value {
            Constant(_) => {
                if let Value(_) | ClosingParenthesis = previous_element {
                    return Err(MissingOperation);
                }
            }
            Variable => {
                if let Value(Variable) = previous_element {
                    return Err(MissingOperation);
                }

                if let Value(Constant(_)) | ClosingParenthesis = previous_element {
                    self.multiplicative_operation = Multiplication;
                }
            }
        }

        match self.multiplicative_operation {
            Multiplication => self.multiplier.multiply_value(value),
            Division => self.multiplier.divide_value(value)?,
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
                self.multiplier.multiply_term(term);
            }
            Division => {
                self.multiplier
                    .divide_term(term, &mut self.term, other_equation_side)?;
            }
        }
        self.multiplier
            .exceptions_in_domain
            .extend(&term.exceptions_in_domain);
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
