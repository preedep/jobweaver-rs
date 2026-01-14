use serde::{Deserialize, Serialize};
use chrono::Utc;
use crate::application::use_cases::{
    analyze_jobs::AnalysisResult,
    calculate_complexity::JobComplexityResult,
    determine_migration_waves::MigrationWave,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisOutput {
    pub summary: SummaryOutput,
    pub jobs: Vec<JobOutput>,
    pub migration_waves: Vec<WaveOutput>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SummaryOutput {
    pub total_jobs: usize,
    pub total_folders: usize,
    pub analysis_date: String,
    pub average_complexity_score: f64,
    pub has_circular_dependencies: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobOutput {
    pub job_name: String,
    pub folder: String,
    pub folder_name: String,
    pub complexity_score: u32,
    pub migration_difficulty: String,
    pub migration_priority: u32,
    pub migration_wave: usize,
    pub is_critical: bool,
    pub dependency_count: usize,
    pub estimated_effort_hours: u32,
    pub metrics: MetricsOutput,
    pub risks: Vec<String>,
    pub airflow_mapping: AirflowMappingOutput,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricsOutput {
    pub dependency_count: usize,
    pub is_critical: bool,
    pub is_cyclic: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AirflowMappingOutput {
    pub suggested_dag_name: String,
    pub operator_type: String,
    pub estimated_effort_hours: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WaveOutput {
    pub wave: usize,
    pub wave_number: usize,
    pub jobs: Vec<String>,
    pub reason: String,
}

impl AnalysisOutput {
    pub fn from_analysis_result(result: AnalysisResult) -> Self {
        let summary = SummaryOutput {
            total_jobs: result.total_jobs,
            total_folders: result.total_folders,
            analysis_date: Utc::now().format("%Y-%m-%d").to_string(),
            average_complexity_score: result.average_complexity,
            has_circular_dependencies: result.has_circular_dependencies,
        };

        let jobs: Vec<JobOutput> = result.complexity_results
            .into_iter()
            .map(JobOutput::from_complexity_result)
            .collect();

        let migration_waves: Vec<WaveOutput> = result.migration_waves
            .into_iter()
            .map(WaveOutput::from_migration_wave)
            .collect();

        Self {
            summary,
            jobs,
            migration_waves,
        }
    }
}

impl JobOutput {
    fn from_complexity_result(result: JobComplexityResult) -> Self {
        let risks = Self::generate_risks(&result);
        let airflow_mapping = AirflowMappingOutput {
            suggested_dag_name: Self::generate_dag_name(&result.job_name),
            operator_type: if result.is_cyclic {
                "PythonOperator".to_string()
            } else {
                "BashOperator".to_string()
            },
            estimated_effort_hours: result.migration_difficulty.estimated_effort_hours(),
        };

        Self {
            job_name: result.job_name.clone(),
            folder: result.folder_name.clone(),
            folder_name: result.folder_name,
            complexity_score: result.complexity_score.value(),
            migration_difficulty: result.migration_difficulty.to_string(),
            migration_priority: result.migration_priority.value(),
            migration_wave: result.migration_wave,
            is_critical: result.is_critical,
            dependency_count: result.dependency_count,
            estimated_effort_hours: result.migration_difficulty.estimated_effort_hours(),
            metrics: MetricsOutput {
                dependency_count: result.dependency_count,
                is_critical: result.is_critical,
                is_cyclic: result.is_cyclic,
            },
            risks,
            airflow_mapping,
        }
    }

    fn generate_risks(result: &JobComplexityResult) -> Vec<String> {
        let mut risks = Vec::new();

        if result.is_cyclic {
            risks.push("Cyclic execution pattern - requires special handling in Airflow".to_string());
        }

        if result.dependency_count > 5 {
            risks.push("High number of dependencies - complex dependency chain".to_string());
        }

        if result.is_critical {
            risks.push("Critical job - requires careful testing and validation".to_string());
        }

        if result.complexity_score.value() > 80 {
            risks.push("Very high complexity - consider breaking into smaller DAGs".to_string());
        }

        if risks.is_empty() {
            risks.push("Low risk migration".to_string());
        }

        risks
    }

    fn generate_dag_name(job_name: &str) -> String {
        job_name.to_lowercase().replace('_', "_")
    }
}

impl WaveOutput {
    fn from_migration_wave(wave: MigrationWave) -> Self {
        Self {
            wave: wave.wave,
            wave_number: wave.wave,
            jobs: wave.jobs,
            reason: wave.reason,
        }
    }
}
