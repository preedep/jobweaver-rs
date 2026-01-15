//! Determine Migration Waves use case module
//!
//! This module provides the use case for determining migration waves based on
//! job complexity and dependencies. It groups jobs into waves for phased migration.

use std::collections::HashMap;
use crate::application::use_cases::calculate_complexity::JobComplexityResult;

/// Use case for determining migration waves
///
/// This use case analyzes job complexity results and assigns jobs to migration
/// waves based on complexity, dependencies, and criticality. Lower waves contain
/// easier jobs that should be migrated first.
pub struct DetermineMigrationWaves;

impl DetermineMigrationWaves {
    /// Creates a new DetermineMigrationWaves use case
    ///
    /// # Returns
    ///
    /// A new DetermineMigrationWaves instance
    pub fn new() -> Self {
        Self
    }

    /// Executes migration wave determination for job complexity results
    ///
    /// # Arguments
    ///
    /// * `results` - Slice of job complexity results to analyze
    ///
    /// # Returns
    ///
    /// Vector of MigrationWave objects, sorted by wave number
    pub fn execute(&self, results: &[JobComplexityResult]) -> Vec<MigrationWave> {
        let mut waves: HashMap<usize, Vec<String>> = HashMap::new();

        for result in results {
            let wave = self.determine_wave(result);
            waves.entry(wave).or_insert_with(Vec::new).push(result.job_name.clone());
        }

        let mut wave_list: Vec<MigrationWave> = waves
            .into_iter()
            .map(|(wave_number, jobs)| {
                let reason = self.get_wave_reason(wave_number);
                MigrationWave {
                    wave: wave_number,
                    jobs,
                    reason: reason.to_string(),
                }
            })
            .collect();

        wave_list.sort_by_key(|w| w.wave);
        wave_list
    }

    /// Determines the migration wave for a single job
    ///
    /// Wave assignment strategy:
    /// - Wave 1-2: Low complexity + minimal/no dependencies (quick wins)
    /// - Wave 3: Medium complexity or critical jobs
    /// - Wave 4: Medium-high complexity with dependencies
    /// - Wave 5: High complexity - requires careful planning
    ///
    /// # Arguments
    ///
    /// * `result` - The job complexity result to analyze
    ///
    /// # Returns
    ///
    /// The assigned wave number (1-5)
    fn determine_wave(&self, result: &JobComplexityResult) -> usize {
        // Get the complexity score and dependency count for the current job.
        let score = result.complexity_score.value();
        let deps = result.dependency_count;

        // Determine the migration wave based on the complexity score and
        // dependency count.
        match score {
            // Very low complexity (0-15)
            0..=15 => {
                // If the job has no dependencies, assign it to wave 1.
                if deps == 0 {
                    1  // Perfect quick win
                } else if deps <= 1 {
                    // If the job has one or fewer dependencies, assign it to wave 1.
                    1  // Still very simple
                } else {
                    // If the job has more than one dependency, assign it to wave 2.
                    2  // Low complexity but has dependencies
                }
            }
            // Low complexity (16-30)
            16..=30 => {
                // If the job has no dependencies, assign it to wave 1.
                if deps == 0 {
                    1  // Easy job, no dependencies
                } else if deps <= 1 {
                    // If the job has one or fewer dependencies, assign it to wave 2.
                    2  // Easy job, minimal dependencies
                } else {
                    // If the job has more than one dependency, assign it to wave 3.
                    3  // Easy job but too many dependencies
                }
            }
            // Medium-low complexity (31-45)
            31..=45 => {
                // If the job has no dependencies, assign it to wave 2.
                if deps == 0 {
                    2  // Medium complexity but independent
                } else if deps <= 1 {
                    // If the job has one or fewer dependencies, assign it to wave 3.
                    3  // Medium complexity, minimal dependencies
                } else {
                    // If the job has more than one dependency, assign it to wave 3.
                    3  // Medium complexity with dependencies
                }
            }
            // Medium-high complexity (46-60)
            46..=60 => {
                // If the job is critical, assign it to wave 3.
                if result.is_critical {
                    3  // Critical jobs get priority
                } else if deps <= 2 {
                    // If the job has two or fewer dependencies, assign it to wave 3.
                    3  // Medium-high, manageable dependencies
                } else {
                    // If the job has more than two dependencies, assign it to wave 4.
                    4  // Medium-high with many dependencies
                }
            }
            // High complexity (61+)
            _ => {
                // If the job is critical, assign it to wave 4.
                if result.is_critical {
                    4  // Critical hard jobs
                } else {
                    // If the job is not critical, assign it to wave 5.
                    5  // Non-critical hard jobs - last wave
                }
            }
        }
    }

    /// Gets the description/reason for a migration wave
    ///
    /// This method returns a string describing the purpose of the given wave.
    ///
    /// # Arguments
    ///
    /// * `wave` - The wave number
    ///
    /// # Returns
    ///
    /// A string describing the wave's purpose
    fn get_wave_reason(&self, wave: usize) -> &str {
        match wave {
            1 => "Low complexity, no dependencies - Quick wins",
            2 => "Low to medium complexity, minimal dependencies",
            3 => "Medium complexity or critical jobs",
            4 => "Medium complexity with dependencies",
            5 => "High complexity - Requires careful planning",
            _ => "Unknown",
        }
    }
}

impl Default for DetermineMigrationWaves {
    fn default() -> Self {
        Self::new()
    }
}

/// Represents a migration wave containing a group of jobs
///
/// A migration wave groups jobs that should be migrated together
/// in the same phase of the migration project.
#[derive(Debug, Clone)]
pub struct MigrationWave {
    /// Wave number (1 is first/easiest, higher numbers are later/harder)
    pub wave: usize,
    /// List of job names in this wave
    pub jobs: Vec<String>,
    /// Description of why jobs are in this wave
    pub reason: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::value_objects::{ComplexityScore, MigrationDifficulty, MigrationPriority};

    #[test]
    fn test_determine_wave_easy_no_deps() {
        let use_case = DetermineMigrationWaves::new();
        let result = JobComplexityResult {
            job_name: "EASY_JOB".to_string(),
            folder_name: "FOLDER".to_string(),
            complexity_score: ComplexityScore::new(10),
            migration_difficulty: MigrationDifficulty::Easy,
            migration_priority: MigrationPriority::new(100),
            migration_wave: 0,
            dependency_count: 0,
            is_critical: false,
            is_cyclic: false,
        };

        let wave = use_case.determine_wave(&result);
        assert_eq!(wave, 1);
    }

    #[test]
    fn test_execute() {
        let use_case = DetermineMigrationWaves::new();
        let results = vec![
            JobComplexityResult {
                job_name: "JOB1".to_string(),
                folder_name: "FOLDER".to_string(),
                complexity_score: ComplexityScore::new(10),
                migration_difficulty: MigrationDifficulty::Easy,
                migration_priority: MigrationPriority::new(100),
                migration_wave: 0,
                dependency_count: 0,
                is_critical: false,
                is_cyclic: false,
            },
            JobComplexityResult {
                job_name: "JOB2".to_string(),
                folder_name: "FOLDER".to_string(),
                complexity_score: ComplexityScore::new(75),
                migration_difficulty: MigrationDifficulty::Hard,
                migration_priority: MigrationPriority::new(10),
                migration_wave: 0,
                dependency_count: 5,
                is_critical: false,
                is_cyclic: true,
            },
        ];

        let waves = use_case.execute(&results);
        assert!(waves.len() >= 1);
    }
}
