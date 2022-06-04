#[derive(Clone)]
pub enum EquationElement {
    Value(ValueType),
    Operation(OperationType),
    Separator,
    OpeningParenthesis,
    ClosingParenthesis,
}

#[derive(Clone)]
pub enum ValueType {
    Constant(f64),
    Variable,
}

#[derive(Clone)]
pub enum OperationType {
    AdditiveOperation(AdditiveOperationType),
    MultiplicativeOperation(MultiplicativeOperationType),
}

#[derive(Clone)]
pub enum AdditiveOperationType {
    Addition,
    Subtraction,
}

#[derive(Clone)]
pub enum MultiplicativeOperationType {
    Multiplication,
    Division,
}
