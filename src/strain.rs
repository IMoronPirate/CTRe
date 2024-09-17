#[derive(Clone)]
pub enum ProbOutcome {
    S(f64),
    I(f64),
    R(f64),
    Inconclusive(f64),
    TechnicalError(f64),
}

impl ProbOutcome {
    pub(crate) fn value(&self) -> f64 {
        match &self {
            ProbOutcome::S(x) => *x,
            ProbOutcome::I(x) => *x,
            ProbOutcome::R(x) => *x,
            ProbOutcome::Inconclusive(x) => *x,
            ProbOutcome::TechnicalError(x) => *x,
        }
    }
}

pub struct Strain {
    p: Vec<ProbOutcome>,
}

impl Strain {
		/// Generate a new Strain check for consistency in input probabilities
    pub fn new(p: &[ProbOutcome; 5]) -> Self {
        assert_eq!(
            p.iter().map(|x| x.value()).sum::<f64>(),
            1.0,
            "Sum of probability not 1."
        );
        assert!(p.iter().all(|x| x.value() >= 0f64), "Negative probability.");
        let mut tot_states = 0;
        for state in p {
            match state {
                ProbOutcome::S(_) => tot_states += 1,
                ProbOutcome::I(_) => tot_states += 10,
                ProbOutcome::R(_) => tot_states += 100,
                ProbOutcome::Inconclusive(_) => tot_states += 1000,
                ProbOutcome::TechnicalError(_) => tot_states += 10_000,
            }
        }
        assert_eq!(tot_states, 11111, "Missing one possible state outcome.");
        Strain { p: p.to_vec() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "Sum of probability not 1.")]
    fn check_if_greater_then_1_prob() {
        Strain::new(&[
            ProbOutcome::S(0.9),
            ProbOutcome::R(0.1),
            ProbOutcome::I(0.5),
            ProbOutcome::Inconclusive(0.),
            ProbOutcome::TechnicalError(0.),
        ]);
    }

    #[test]
    #[should_panic(expected = "Negative probability.")]
    fn check_if_less_than_zero_prob() {
        Strain::new(&[
            ProbOutcome::S(0.9),
            ProbOutcome::R(0.2),
            ProbOutcome::I(-0.1),
            ProbOutcome::Inconclusive(0.),
            ProbOutcome::TechnicalError(0.),
        ]);
    }

    #[test]
    #[should_panic(expected = "Missing one possible state outcome.")]
    fn check_if_missing_state() {
        Strain::new(&[
            ProbOutcome::S(0.9),
            ProbOutcome::R(0.1),
            ProbOutcome::R(0.0),
            ProbOutcome::Inconclusive(0.),
            ProbOutcome::TechnicalError(0.),
        ]);
    }
}
