use crate::equation_element::{
    AdditiveOperationType::*,
    EquationElement::{self, *},
    MultiplicativeOperationType::*,
    OperationType::*,
    ValueType::*,
};
use crate::equation_error::EquationError::{self, *};

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

    pub fn token_stream(&self) -> Result<Vec<EquationElement>, EquationError> {
        if self.equation.is_empty() {
            return Err(EmptyEquation);
        }
        if self.variable_name.is_empty() {
            return Err(EmptyVariableName);
        }

        let mut elements = Vec::new();
        let mut current_value = String::new();
        let mut number = true;
        for element in self.equation.chars() {
            let element = match element {
                ' ' => {
                    add_value_to_elements(&mut elements, &mut current_value, &self.variable_name)?;
                    number = true;
                    continue;
                }
                '+' => Operation(AdditiveOperation(Addition)),
                '-' => Operation(AdditiveOperation(Subtraction)),
                '*' => Operation(MultiplicativeOperation(Multiplication)),
                '/' => Operation(MultiplicativeOperation(Division)),
                '=' => Separator,
                '(' => OpeningParenthesis,
                ')' => ClosingParenthesis,
                ',' => {
                    current_value.push('.');
                    continue;
                }
                _ => {
                    if current_value.is_empty() && !element.is_digit(10) {
                        number = false;
                    }
                    if number && !current_value.is_empty() && !element.is_digit(10) {
                        add_value_to_elements(
                            &mut elements,
                            &mut current_value,
                            &self.variable_name,
                        )?;
                        number = false;
                    }
                    current_value.push(element);
                    continue;
                }
            };
            add_value_to_elements(&mut elements, &mut current_value, &self.variable_name)?;
            number = true;
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
) -> Result<(), EquationError> {
    if value.is_empty() {
        return Ok(());
    }
    if &*value == "i" {
        return Err(ComplexNumbers);
    }

    let element = Value(match *value == variable_name {
        true => Variable,
        false => match value.parse::<f64>() {
            Ok(value) => Constant(value),
            Err(_) => {
                return Err(InvalidElement(value.to_owned()));
            }
        },
    });
    elements.push(element);
    *value = String::new();
    Ok(())
}
