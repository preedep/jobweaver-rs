use serde::{Deserialize, Serialize};
use std::fmt;
use super::ComplexityScore;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum MigrationDifficulty {
    Easy,
    Medium,
    Hard,
}

impl MigrationDifficulty {
    pub fn from_complexity_score(score: ComplexityScore) -> Self {
        match score.value() {
            0..=30 => MigrationDifficulty::Easy,
            31..=60 => MigrationDifficulty::Medium,
            _ => MigrationDifficulty::Hard,
        }
    }

    pub fn estimated_effort_hours(&self) -> u32 {
        match self {
            MigrationDifficulty::Easy => 4,
            MigrationDifficulty::Medium => 8,
            MigrationDifficulty::Hard => 16,
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            MigrationDifficulty::Easy => "Easy",
            MigrationDifficulty::Medium => "Medium",
            MigrationDifficulty::Hard => "Hard",
        }
    }
}

impl fmt::Display for MigrationDifficulty {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_complexity_score_easy() {
        let score = ComplexityScore::new(20);
        let difficulty = MigrationDifficulty::from_complexity_score(score);
        assert_eq!(difficulty, MigrationDifficulty::Easy);
        assert_eq!(difficulty.estimated_effort_hours(), 4);
    }

    #[test]
    fn test_from_complexity_score_medium() {
        let score = ComplexityScore::new(45);
        let difficulty = MigrationDifficulty::from_complexity_score(score);
        assert_eq!(difficulty, MigrationDifficulty::Medium);
        assert_eq!(difficulty.estimated_effort_hours(), 8);
    }

    #[test]
    fn test_from_complexity_score_hard() {
        let score = ComplexityScore::new(75);
        let difficulty = MigrationDifficulty::from_complexity_score(score);
        assert_eq!(difficulty, MigrationDifficulty::Hard);
        assert_eq!(difficulty.estimated_effort_hours(), 16);
    }

    #[test]
    fn test_boundary_values() {
        assert_eq!(
            MigrationDifficulty::from_complexity_score(ComplexityScore::new(30)),
            MigrationDifficulty::Easy
        );
        assert_eq!(
            MigrationDifficulty::from_complexity_score(ComplexityScore::new(31)),
            MigrationDifficulty::Medium
        );
        assert_eq!(
            MigrationDifficulty::from_complexity_score(ComplexityScore::new(60)),
            MigrationDifficulty::Medium
        );
        assert_eq!(
            MigrationDifficulty::from_complexity_score(ComplexityScore::new(61)),
            MigrationDifficulty::Hard
        );
    }
}
