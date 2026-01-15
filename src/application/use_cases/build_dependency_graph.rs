//! Build Dependency Graph use case module
//!
//! This module provides the use case for building and analyzing job dependency graphs.
//! It orchestrates the dependency analyzer service to create graph structures.

use crate::domain::entities::Job;
use crate::application::services::DependencyAnalyzer;

/// Use case for building job dependency graphs
///
/// This use case encapsulates the business logic for constructing dependency
/// graphs from job collections and analyzing their properties.
pub struct BuildDependencyGraph {
    /// The dependency analyzer service
    analyzer: DependencyAnalyzer,
}

impl BuildDependencyGraph {
    /// Creates a new BuildDependencyGraph use case
    ///
    /// # Returns
    ///
    /// A new BuildDependencyGraph instance with an initialized analyzer
    pub fn new() -> Self {
        Self {
            analyzer: DependencyAnalyzer::new(),
        }
    }

    /// Executes dependency graph building for a collection of jobs
    ///
    /// # Arguments
    ///
    /// * `jobs` - Slice of job references to build the graph from
    ///
    /// # Returns
    ///
    /// A DependencyGraphResult containing graph analysis results
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

    /// Gets a reference to the internal dependency analyzer
    ///
    /// # Returns
    ///
    /// Reference to the DependencyAnalyzer
    pub fn get_analyzer(&self) -> &DependencyAnalyzer {
        &self.analyzer
    }
}

impl Default for BuildDependencyGraph {
    fn default() -> Self {
        Self::new()
    }
}

/// Result of dependency graph building
///
/// Contains information about the constructed graph including
/// total jobs, circular dependency detection, and topological ordering.
#[derive(Debug)]
pub struct DependencyGraphResult {
    /// Total number of jobs in the graph
    pub total_jobs: usize,
    /// Whether circular dependencies were detected
    pub has_circular_dependencies: bool,
    /// Topological order of jobs (None if circular dependencies exist)
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
