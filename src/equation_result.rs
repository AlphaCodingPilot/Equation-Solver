#[derive(PartialEq, Debug)]
pub enum EquationResult {
    Solutions(Vec<f64>),
    Unsolvable,
    InfiniteSolutions { exceptions: Vec<f64> },
}
