use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct ComplexityScore(u32);

impl ComplexityScore {
    pub fn new(score: u32) -> Self {
        Self(score)
    }

    pub fn value(&self) -> u32 {
        self.0
    }

    pub fn from_metrics(
        dependency_count: usize,
        dependency_depth: usize,
        in_conditions: usize,
        out_conditions: usize,
        variables_count: usize,
        on_conditions: usize,
        on_conditions_complexity: usize,
        is_cyclic: bool,
        quantitative_resources: usize,
        control_resources: usize,
        scheduling_complexity: usize,
    ) -> Self {
        let score = (dependency_count * 3)
            + (dependency_depth * 5)
            + (in_conditions * 2)
            + (out_conditions * 2)
            + variables_count
            + (on_conditions * 4)
            + (on_conditions_complexity * 5)
            + if is_cyclic { 15 } else { 0 }
            + (quantitative_resources * 3)
            + (control_resources * 3)
            + (scheduling_complexity * 2);

        Self(score as u32)
    }

    pub fn add(&self, other: u32) -> Self {
        Self(self.0 + other)
    }
}

impl fmt::Display for ComplexityScore {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<u32> for ComplexityScore {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl From<ComplexityScore> for u32 {
    fn from(score: ComplexityScore) -> Self {
        score.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_complexity_score() {
        let score = ComplexityScore::new(50);
        assert_eq!(score.value(), 50);
    }

    #[test]
    fn test_from_metrics_simple() {
        let score = ComplexityScore::from_metrics(
            2, // dependency_count
            1, // dependency_depth
            1, // in_conditions
            1, // out_conditions
            0, // variables_count
            0, // on_conditions
            0, // on_conditions_complexity
            false, // is_cyclic
            0, // quantitative_resources
            0, // control_resources
            0, // scheduling_complexity
        );
        // (2*3) + (1*5) + (1*2) + (1*2) = 6 + 5 + 2 + 2 = 15
        assert_eq!(score.value(), 15);
    }

    #[test]
    fn test_from_metrics_complex() {
        let score = ComplexityScore::from_metrics(
            10, // dependency_count
            4,  // dependency_depth
            5,  // in_conditions
            3,  // out_conditions
            8,  // variables_count
            2,  // on_conditions
            3,  // on_conditions_complexity
            true, // is_cyclic
            1,  // quantitative_resources
            2,  // control_resources
            5,  // scheduling_complexity
        );
        // (10*3) + (4*5) + (5*2) + (3*2) + 8 + (2*4) + (3*5) + 15 + (1*3) + (2*3) + (5*2)
        // = 30 + 20 + 10 + 6 + 8 + 8 + 15 + 15 + 3 + 6 + 10 = 131
        assert_eq!(score.value(), 131);
    }

    #[test]
    fn test_complexity_score_ordering() {
        let score1 = ComplexityScore::new(30);
        let score2 = ComplexityScore::new(60);
        assert!(score1 < score2);
    }
}
