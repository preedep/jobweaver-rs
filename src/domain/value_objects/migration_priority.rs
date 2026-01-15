//! Migration Priority value object module
//!
//! This module defines the MigrationPriority value object which determines
//! the order in which jobs should be migrated.

use serde::{Deserialize, Serialize};
use super::{ComplexityScore, MigrationDifficulty};

/// Represents the priority level for migrating a job
///
/// Priority is calculated based on complexity (easier jobs get higher priority),
/// criticality (critical jobs get bonus priority), and dependencies (more
/// dependencies reduce priority). Higher values mean higher priority.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct MigrationPriority(u32);

impl MigrationPriority {
    /// Creates a new MigrationPriority with the given value
    ///
    /// # Arguments
    ///
    /// * `priority` - The priority value (higher is more important)
    ///
    /// # Returns
    ///
    /// A new MigrationPriority instance
    pub fn new(priority: u32) -> Self {
        Self(priority)
    }

    /// Returns the numeric value of the priority
    ///
    /// # Returns
    ///
    /// The underlying u32 priority value
    pub fn value(&self) -> u32 {
        self.0
    }

    /// Calculates migration priority from job characteristics
    ///
    /// Priority calculation strategy:
    /// - Base priority: Easy=100, Medium=50, Hard=10
    /// - Critical bonus: +50 for critical jobs
    /// - Dependency penalty: -2 per dependency
    /// - Minimum priority: 1 (never zero)
    ///
    /// This encourages migrating easier jobs first while giving critical
    /// jobs higher priority regardless of complexity.
    ///
    /// # Arguments
    ///
    /// * `complexity_score` - The job's complexity score
    /// * `is_critical` - Whether the job is marked as critical
    /// * `dependency_count` - Number of dependencies the job has
    ///
    /// # Returns
    ///
    /// A calculated MigrationPriority
    pub fn calculate(
        complexity_score: ComplexityScore,
        is_critical: bool,
        dependency_count: usize,
    ) -> Self {
        let difficulty = MigrationDifficulty::from_complexity_score(complexity_score);
        
        // Base priority favors easier jobs
        let base_priority: u32 = match difficulty {
            MigrationDifficulty::Easy => 100,    // Highest base priority
            MigrationDifficulty::Medium => 50,   // Medium base priority
            MigrationDifficulty::Hard => 10,     // Lowest base priority
        };

        // Critical jobs get a significant bonus
        let critical_bonus: u32 = if is_critical { 50 } else { 0 };
        
        // Dependencies reduce priority (migrate independent jobs first)
        let dependency_penalty: u32 = (dependency_count as u32) * 2;

        // Calculate final priority with saturation to prevent overflow/underflow
        let priority = base_priority.saturating_add(critical_bonus).saturating_sub(dependency_penalty);
        
        // Ensure priority is never zero
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
