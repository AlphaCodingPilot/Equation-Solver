#[derive(Clone, PartialEq)]
pub enum EquationElement {
    Value(ValueType),
    Operation(OperationType),
    Separator,
    OpeningParenthesis,
    ClosingParenthesis,
}

#[derive(Clone, PartialEq)]
pub enum ValueType {
    Constant(f64),
    Variable,
}

#[derive(Clone, PartialEq)]
pub enum OperationType {
    AdditiveOperation(AdditiveOperationType),
    MultiplicativeOperation(MultiplicativeOperationType),
}

#[derive(Clone, PartialEq)]
pub enum AdditiveOperationType {
    Addition,
    Subtraction,
}

#[derive(Clone, PartialEq)]
pub enum MultiplicativeOperationType {
    Multiplication,
    Division,
}
