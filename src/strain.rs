/// All possible outcome of experiment
#[derive(Clone)]
pub enum State {
    S(f64),
    I(f64),
    R(f64),
    Inconclusive(f64),
    TechnicalError(f64),
}

impl State {
    /// Get the value of the state
    pub(crate) fn value(&self) -> f64 {
        match &self {
            State::S(x) => *x,
            State::I(x) => *x,
            State::R(x) => *x,
            State::Inconclusive(x) => *x,
            State::TechnicalError(x) => *x,
        }
    }

    /// Get the encoded position of the state
    pub(crate) fn position(&self) -> usize {
        match &self {
            State::S(_) => 0,
            State::I(_) => 1,
            State::R(_) => 2,
            State::Inconclusive(_) => 3,
            State::TechnicalError(_) => 4,
        }
    }
}

pub struct Strain {
    p: Vec<State>,
}

impl Strain {
    /// Generate a new Strain check for consistency in input probabilities
		//TODO different strains might have different outcome, for instance no I,
		//TODO or not inconclusive
    pub fn new(p: &[State; 5]) -> Self {
        assert_eq!(
            p.iter().map(|x| x.value()).sum::<f64>(),
            1.0,
            "Sum of probability not 1."
        );
        assert!(p.iter().all(|x| x.value() >= 0f64), "Negative probability.");
        let mut tot_states = 0;
        for state in p {
            match state {
                State::S(_) => tot_states += 1,
                State::I(_) => tot_states += 10,
                State::R(_) => tot_states += 100,
                State::Inconclusive(_) => tot_states += 1_000,
                State::TechnicalError(_) => tot_states += 10_000,
            }
        }
        assert_eq!(tot_states, 11111, "Missing one possible state outcome.");
        Strain { p: p.to_vec() }
    }

    fn probabilities_values(ss: &Self) -> Vec<f64> {
        let mut probs = Vec::<f64>::with_capacity(5);
        for state in ss.p.iter() {
            match state {
                State::S(x) => probs[state.position()] = *x,
                State::I(x) => probs[state.position()] = *x,
                State::R(x) => probs[state.position()] = *x,
                State::Inconclusive(x) => probs[state.position()] = *x,
                State::TechnicalError(x) => probs[state.position()] = *x,
            }
        }
        probs
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "Sum of probability not 1.")]
    fn check_if_greater_then_1_prob() {
        Strain::new(&[
            State::S(0.9),
            State::R(0.1),
            State::I(0.5),
            State::Inconclusive(0.),
            State::TechnicalError(0.),
        ]);
    }

    #[test]
    #[should_panic(expected = "Negative probability.")]
    fn check_if_less_than_zero_prob() {
        Strain::new(&[
            State::S(0.9),
            State::R(0.2),
            State::I(-0.1),
            State::Inconclusive(0.),
            State::TechnicalError(0.),
        ]);
    }

    #[test]
    #[should_panic(expected = "Missing one possible state outcome.")]
    fn check_if_missing_state() {
        Strain::new(&[
            State::S(0.9),
            State::R(0.1),
            State::R(0.0),
            State::Inconclusive(0.),
            State::TechnicalError(0.),
        ]);
    }
}
