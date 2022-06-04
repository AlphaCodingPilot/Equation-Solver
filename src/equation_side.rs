use crate::term::Term;

pub struct EquationSide {
    pub term: Term,
    pub multiplier: Term,
    pub side: EquationSideType,
}

impl EquationSide {
    pub fn new(side: EquationSideType) -> Self {
        Self {
            term: Term::new(),
            multiplier: Term::new_multiplier(),
            side,
        }
    }

    pub fn push_multiplier(&mut self) {
        self.term.multiply_term(&self.multiplier);
        self.multiplier = Term::new_multiplier();
    }
}

pub enum EquationSideType {
    LeftHandSide,
    RightHandSide,
}
