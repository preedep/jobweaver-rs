//! Complexity Score value object module
//!
//! This module defines the ComplexityScore value object which represents
//! the calculated complexity of a job based on various metrics.

use serde::{Deserialize, Serialize};
use std::fmt;

/// Represents a job's complexity score
///
/// ComplexityScore is calculated from multiple job metrics including dependencies,
/// conditions, resources, and scheduling complexity. Higher scores indicate
/// more complex jobs that are harder to migrate or maintain.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct ComplexityScore(u32);

impl ComplexityScore {
    /// Creates a new ComplexityScore with the given value
    ///
    /// # Arguments
    ///
    /// * `score` - The complexity score value
    ///
    /// # Returns
    ///
    /// A new ComplexityScore instance
    pub fn new(score: u32) -> Self {
        Self(score)
    }

    /// Returns the numeric value of the complexity score
    ///
    /// # Returns
    ///
    /// The underlying u32 score value
    pub fn value(&self) -> u32 {
        self.0
    }

    /// Calculates complexity score from job metrics
    ///
    /// This is the primary method for computing job complexity. It uses a weighted
    /// formula that considers various aspects of job configuration.
    ///
    /// # Arguments
    ///
    /// * `dependency_count` - Number of job dependencies
    /// * `dependency_depth` - Depth of dependency chain
    /// * `in_conditions` - Number of input conditions
    /// * `out_conditions` - Number of output conditions
    /// * `variables_count` - Number of variables and auto-edits
    /// * `on_conditions` - Number of event-based conditions
    /// * `on_conditions_complexity` - Combined complexity of all on-conditions
    /// * `is_cyclic` - Whether the job runs cyclically
    /// * `quantitative_resources` - Number of quantitative resources
    /// * `control_resources` - Number of control resources
    /// * `scheduling_complexity` - Complexity score from scheduling configuration
    ///
    /// # Returns
    ///
    /// A ComplexityScore calculated from the weighted metrics
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
        // Weighted complexity calculation
        let score = (dependency_count * 3)              // Dependencies moderately increase complexity
            + (dependency_depth * 5)                    // Deep dependency chains are highly complex
            + (in_conditions * 2)                       // Input conditions add complexity
            + (out_conditions * 2)                      // Output conditions add complexity
            + variables_count                           // Variables add minor complexity
            + (on_conditions * 4)                       // Event conditions are moderately complex
            + (on_conditions_complexity * 5)            // Complex event actions are highly complex
            + if is_cyclic { 15 } else { 0 }           // Cyclic jobs are significantly more complex
            + (quantitative_resources * 3)              // Resource management adds complexity
            + (control_resources * 3)                   // Mutex management adds complexity
            + (scheduling_complexity * 2);              // Complex scheduling adds complexity

        Self(score as u32)
    }

    /// Adds a value to this complexity score
    ///
    /// # Arguments
    ///
    /// * `other` - Value to add to the score
    ///
    /// # Returns
    ///
    /// A new ComplexityScore with the added value
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
