use serde::{Deserialize, Serialize};
use super::{ComplexityScore, MigrationDifficulty};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct MigrationPriority(u32);

impl MigrationPriority {
    pub fn new(priority: u32) -> Self {
        Self(priority)
    }

    pub fn value(&self) -> u32 {
        self.0
    }

    pub fn calculate(
        complexity_score: ComplexityScore,
        is_critical: bool,
        dependency_count: usize,
    ) -> Self {
        let difficulty = MigrationDifficulty::from_complexity_score(complexity_score);
        
        let base_priority = match difficulty {
            MigrationDifficulty::Easy => 100,
            MigrationDifficulty::Medium => 50,
            MigrationDifficulty::Hard => 10,
        };

        let critical_bonus = if is_critical { 50 } else { 0 };
        
        let dependency_penalty = (dependency_count as u32) * 2;

        let priority = base_priority + critical_bonus - dependency_penalty;
        
        Self(priority.max(1))
    }
}

impl From<u32> for MigrationPriority {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl From<MigrationPriority> for u32 {
    fn from(priority: MigrationPriority) -> Self {
        priority.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_migration_priority() {
        let priority = MigrationPriority::new(50);
        assert_eq!(priority.value(), 50);
    }

    #[test]
    fn test_calculate_easy_non_critical() {
        let score = ComplexityScore::new(20);
        let priority = MigrationPriority::calculate(score, false, 2);
        // base: 100, critical: 0, dependency_penalty: 4
        // 100 + 0 - 4 = 96
        assert_eq!(priority.value(), 96);
    }

    #[test]
    fn test_calculate_easy_critical() {
        let score = ComplexityScore::new(20);
        let priority = MigrationPriority::calculate(score, true, 2);
        // base: 100, critical: 50, dependency_penalty: 4
        // 100 + 50 - 4 = 146
        assert_eq!(priority.value(), 146);
    }

    #[test]
    fn test_calculate_hard_non_critical() {
        let score = ComplexityScore::new(75);
        let priority = MigrationPriority::calculate(score, false, 5);
        // base: 10, critical: 0, dependency_penalty: 10
        // 10 + 0 - 10 = 0, but min is 1
        assert_eq!(priority.value(), 1);
    }

    #[test]
    fn test_priority_ordering() {
        let priority1 = MigrationPriority::new(100);
        let priority2 = MigrationPriority::new(50);
        assert!(priority1 > priority2);
    }
}
