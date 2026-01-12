use std::collections::HashMap;
use crate::application::use_cases::calculate_complexity::JobComplexityResult;
use crate::domain::value_objects::MigrationDifficulty;

pub struct DetermineMigrationWaves;

impl DetermineMigrationWaves {
    pub fn new() -> Self {
        Self
    }

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

    fn determine_wave(&self, result: &JobComplexityResult) -> usize {
        match result.migration_difficulty {
            MigrationDifficulty::Easy => {
                if result.dependency_count == 0 {
                    1
                } else if result.dependency_count <= 2 {
                    2
                } else {
                    3
                }
            }
            MigrationDifficulty::Medium => {
                if result.is_critical {
                    2
                } else if result.dependency_count <= 3 {
                    3
                } else {
                    4
                }
            }
            MigrationDifficulty::Hard => {
                if result.is_critical {
                    3
                } else {
                    5
                }
            }
        }
    }

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

#[derive(Debug, Clone)]
pub struct MigrationWave {
    pub wave: usize,
    pub jobs: Vec<String>,
    pub reason: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::value_objects::{ComplexityScore, MigrationPriority};

    #[test]
    fn test_determine_wave_easy_no_deps() {
        let use_case = DetermineMigrationWaves::new();
        let result = JobComplexityResult {
            job_name: "EASY_JOB".to_string(),
            folder_name: "FOLDER".to_string(),
            complexity_score: ComplexityScore::new(10),
            migration_difficulty: MigrationDifficulty::Easy,
            migration_priority: MigrationPriority::new(100),
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
                dependency_count: 5,
                is_critical: false,
                is_cyclic: true,
            },
        ];

        let waves = use_case.execute(&results);
        assert!(waves.len() >= 1);
    }
}
