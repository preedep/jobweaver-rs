//! Dependency Analyzer service module
//!
//! This service builds and analyzes dependency graphs for jobs,
//! detecting circular dependencies and computing topological ordering.

use std::collections::{HashMap, HashSet};
use petgraph::graph::{DiGraph, NodeIndex};
use petgraph::algo::toposort;
use petgraph::visit::EdgeRef;
use crate::domain::entities::{Job, Dependency, DependencyType};

/// Service for analyzing job dependencies
///
/// DependencyAnalyzer builds a directed graph of job dependencies and provides
/// methods to analyze the graph, including cycle detection and topological sorting.
pub struct DependencyAnalyzer {
    /// Directed graph representing job dependencies
    graph: DiGraph<String, String>,
    /// Map from job names to their graph node indices
    job_indices: HashMap<String, NodeIndex>,
}

impl DependencyAnalyzer {
    /// Creates a new DependencyAnalyzer instance
    ///
    /// # Returns
    ///
    /// A new DependencyAnalyzer with an empty graph
    pub fn new() -> Self {
        Self {
            graph: DiGraph::new(),
            job_indices: HashMap::new(),
        }
    }

    /// Builds the dependency graph from a collection of jobs
    ///
    /// This method adds all jobs as nodes and creates edges based on
    /// input conditions and control resources.
    ///
    /// # Arguments
    ///
    /// * `jobs` - Slice of job references to build the graph from
    pub fn build_graph(&mut self, jobs: &[&Job]) {
        // First pass: add all jobs as nodes
        for job in jobs {
            self.add_job(&job.job_name);
        }

        // Second pass: add dependencies as edges
        for job in jobs {
            // Add edges for input conditions
            for in_cond in &job.in_conditions {
                self.add_dependency(&in_cond.name, &job.job_name, "INCOND");
            }

            // Add edges for control resources
            for ctrl_res in &job.control_resources {
                self.add_dependency(&ctrl_res.name, &job.job_name, "CONTROL");
            }
        }
    }

    /// Adds a job to the dependency graph
    ///
    /// If the job already exists, this is a no-op.
    ///
    /// # Arguments
    ///
    /// * `job_name` - Name of the job to add
    pub fn add_job(&mut self, job_name: &str) {
        if !self.job_indices.contains_key(job_name) {
            let idx = self.graph.add_node(job_name.to_string());
            self.job_indices.insert(job_name.to_string(), idx);
        }
    }

    /// Adds a dependency edge between two jobs
    ///
    /// Creates both jobs if they don't exist, then adds an edge from
    /// the first job to the second.
    ///
    /// # Arguments
    ///
    /// * `from` - Name of the predecessor job
    /// * `to` - Name of the successor job
    /// * `dep_type` - Type of dependency (e.g., "INCOND", "CONTROL")
    pub fn add_dependency(&mut self, from: &str, to: &str, dep_type: &str) {
        self.add_job(from);
        self.add_job(to);

        let from_idx = self.job_indices[from];
        let to_idx = self.job_indices[to];
        
        self.graph.add_edge(from_idx, to_idx, dep_type.to_string());
    }

    /// Gets all dependencies for a specific job
    ///
    /// Returns a list of Dependency objects representing all incoming
    /// dependencies (jobs that this job depends on).
    ///
    /// # Arguments
    ///
    /// * `job_name` - Name of the job to get dependencies for
    ///
    /// # Returns
    ///
    /// Vector of Dependency objects
    pub fn get_dependencies(&self, job_name: &str) -> Vec<Dependency> {
        let mut dependencies = Vec::new();

        if let Some(&job_idx) = self.job_indices.get(job_name) {
            for edge in self.graph.edges_directed(job_idx, petgraph::Direction::Incoming) {
                let from_job = &self.graph[edge.source()];
                let dep_type = match edge.weight().as_str() {
                    "INCOND" => DependencyType::InCondition,
                    "OUTCOND" => DependencyType::OutCondition,
                    "CONTROL" => DependencyType::ControlResource,
                    "QUANTITATIVE" => DependencyType::QuantitativeResource,
                    _ => DependencyType::InCondition,
                };

                dependencies.push(Dependency::new(
                    from_job.clone(),
                    job_name.to_string(),
                    dep_type,
                ));
            }
        }

        dependencies
    }

    /// Calculates the dependency depth for a job
    ///
    /// Depth is the length of the longest path from any root node to this job.
    /// Jobs with no dependencies have depth 0.
    ///
    /// # Arguments
    ///
    /// * `job_name` - Name of the job to calculate depth for
    ///
    /// # Returns
    ///
    /// The dependency depth
    pub fn get_dependency_depth(&self, job_name: &str) -> usize {
        if let Some(&job_idx) = self.job_indices.get(job_name) {
            self.calculate_depth(job_idx, &mut HashSet::new())
        } else {
            0
        }
    }

    /// Recursively calculates depth for a node
    ///
    /// Uses a visited set to detect and handle cycles.
    ///
    /// # Arguments
    ///
    /// * `node` - The node to calculate depth for
    /// * `visited` - Set of already visited nodes (for cycle detection)
    ///
    /// # Returns
    ///
    /// The depth of this node
    fn calculate_depth(&self, node: NodeIndex, visited: &mut HashSet<NodeIndex>) -> usize {
        // Cycle detection: if already visited, return 0
        if visited.contains(&node) {
            return 0;
        }

        visited.insert(node);

        // Find maximum depth among all predecessors
        let mut max_depth = 0;
        for edge in self.graph.edges_directed(node, petgraph::Direction::Incoming) {
            let depth = self.calculate_depth(edge.source(), visited);
            max_depth = max_depth.max(depth);
        }

        // Remove from visited to allow other paths to visit this node
        visited.remove(&node);
        max_depth + 1
    }

    /// Performs topological sort on the dependency graph
    ///
    /// Returns jobs in an order where all dependencies come before
    /// their dependents. Fails if the graph contains cycles.
    ///
    /// # Returns
    ///
    /// Result containing either:
    /// - Ok: Vector of job names in topological order
    /// - Err: Error message if circular dependencies exist
    pub fn topological_sort(&self) -> Result<Vec<String>, String> {
        match toposort(&self.graph, None) {
            Ok(sorted) => {
                Ok(sorted.iter().map(|&idx| self.graph[idx].clone()).collect())
            }
            Err(_) => Err("Circular dependency detected".to_string()),
        }
    }

    /// Checks if the dependency graph contains circular dependencies
    ///
    /// # Returns
    ///
    /// `true` if circular dependencies exist, `false` otherwise
    pub fn has_circular_dependencies(&self) -> bool {
        self.topological_sort().is_err()
    }

    /// Gets all upstream jobs (direct predecessors) for a job
    ///
    /// # Arguments
    ///
    /// * `job_name` - Name of the job
    ///
    /// # Returns
    ///
    /// Vector of job names that this job depends on
    pub fn get_upstream_jobs(&self, job_name: &str) -> Vec<String> {
        if let Some(&job_idx) = self.job_indices.get(job_name) {
            self.graph
                .edges_directed(job_idx, petgraph::Direction::Incoming)
                .map(|edge| self.graph[edge.source()].clone())
                .collect()
        } else {
            Vec::new()
        }
    }

    /// Gets all downstream jobs (direct successors) for a job
    ///
    /// # Arguments
    ///
    /// * `job_name` - Name of the job
    ///
    /// # Returns
    ///
    /// Vector of job names that depend on this job
    pub fn get_downstream_jobs(&self, job_name: &str) -> Vec<String> {
        if let Some(&job_idx) = self.job_indices.get(job_name) {
            self.graph
                .edges_directed(job_idx, petgraph::Direction::Outgoing)
                .map(|edge| self.graph[edge.target()].clone())
                .collect()
        } else {
            Vec::new()
        }
    }
}

impl Default for DependencyAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_simple_graph() {
        let mut analyzer = DependencyAnalyzer::new();
        analyzer.add_job("JOB_A");
        analyzer.add_job("JOB_B");
        analyzer.add_dependency("JOB_A", "JOB_B", "INCOND");

        let deps = analyzer.get_dependencies("JOB_B");
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].from_job, "JOB_A");
    }

    #[test]
    fn test_dependency_depth() {
        let mut analyzer = DependencyAnalyzer::new();
        analyzer.add_job("JOB_A");
        analyzer.add_job("JOB_B");
        analyzer.add_job("JOB_C");
        analyzer.add_dependency("JOB_A", "JOB_B", "INCOND");
        analyzer.add_dependency("JOB_B", "JOB_C", "INCOND");

        assert_eq!(analyzer.get_dependency_depth("JOB_A"), 1);
        assert_eq!(analyzer.get_dependency_depth("JOB_B"), 2);
        assert_eq!(analyzer.get_dependency_depth("JOB_C"), 3);
    }

    #[test]
    fn test_topological_sort() {
        let mut analyzer = DependencyAnalyzer::new();
        analyzer.add_job("JOB_A");
        analyzer.add_job("JOB_B");
        analyzer.add_job("JOB_C");
        analyzer.add_dependency("JOB_A", "JOB_B", "INCOND");
        analyzer.add_dependency("JOB_B", "JOB_C", "INCOND");

        let sorted = analyzer.topological_sort().unwrap();
        assert_eq!(sorted.len(), 3);
    }
}
