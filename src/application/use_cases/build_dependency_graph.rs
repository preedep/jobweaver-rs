use crate::domain::entities::Job;
use crate::application::services::DependencyAnalyzer;

pub struct BuildDependencyGraph {
    analyzer: DependencyAnalyzer,
}

impl BuildDependencyGraph {
    pub fn new() -> Self {
        Self {
            analyzer: DependencyAnalyzer::new(),
        }
    }

    pub fn execute(&mut self, jobs: &[&Job]) -> DependencyGraphResult {
        self.analyzer.build_graph(jobs);

        let has_circular = self.analyzer.has_circular_dependencies();
        let topological_order = if !has_circular {
            self.analyzer.topological_sort().ok()
        } else {
            None
        };

        DependencyGraphResult {
            total_jobs: jobs.len(),
            has_circular_dependencies: has_circular,
            topological_order,
        }
    }

    pub fn get_analyzer(&self) -> &DependencyAnalyzer {
        &self.analyzer
    }
}

impl Default for BuildDependencyGraph {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
pub struct DependencyGraphResult {
    pub total_jobs: usize,
    pub has_circular_dependencies: bool,
    pub topological_order: Option<Vec<String>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    

    #[test]
    fn test_execute() {
        let mut use_case = BuildDependencyGraph::new();
        let job1 = Job::new("JOB1".to_string(), "FOLDER".to_string());
        let job2 = Job::new("JOB2".to_string(), "FOLDER".to_string());
        
        let jobs = vec![&job1, &job2];
        let result = use_case.execute(&jobs);
        
        assert_eq!(result.total_jobs, 2);
    }
}
