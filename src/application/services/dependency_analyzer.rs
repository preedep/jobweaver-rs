use std::collections::{HashMap, HashSet};
use petgraph::graph::{DiGraph, NodeIndex};
use petgraph::algo::toposort;
use petgraph::visit::EdgeRef;
use crate::domain::entities::{Job, Dependency, DependencyType};

pub struct DependencyAnalyzer {
    graph: DiGraph<String, String>,
    job_indices: HashMap<String, NodeIndex>,
}

impl DependencyAnalyzer {
    pub fn new() -> Self {
        Self {
            graph: DiGraph::new(),
            job_indices: HashMap::new(),
        }
    }

    pub fn build_graph(&mut self, jobs: &[&Job]) {
        for job in jobs {
            self.add_job(&job.job_name);
        }

        for job in jobs {
            for in_cond in &job.in_conditions {
                self.add_dependency(&in_cond.name, &job.job_name, "INCOND");
            }

            for ctrl_res in &job.control_resources {
                self.add_dependency(&ctrl_res.name, &job.job_name, "CONTROL");
            }
        }
    }

    pub fn add_job(&mut self, job_name: &str) {
        if !self.job_indices.contains_key(job_name) {
            let idx = self.graph.add_node(job_name.to_string());
            self.job_indices.insert(job_name.to_string(), idx);
        }
    }

    pub fn add_dependency(&mut self, from: &str, to: &str, dep_type: &str) {
        self.add_job(from);
        self.add_job(to);

        let from_idx = self.job_indices[from];
        let to_idx = self.job_indices[to];
        
        self.graph.add_edge(from_idx, to_idx, dep_type.to_string());
    }

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

    pub fn get_dependency_depth(&self, job_name: &str) -> usize {
        if let Some(&job_idx) = self.job_indices.get(job_name) {
            self.calculate_depth(job_idx, &mut HashSet::new())
        } else {
            0
        }
    }

    fn calculate_depth(&self, node: NodeIndex, visited: &mut HashSet<NodeIndex>) -> usize {
        if visited.contains(&node) {
            return 0;
        }

        visited.insert(node);

        let mut max_depth = 0;
        for edge in self.graph.edges_directed(node, petgraph::Direction::Incoming) {
            let depth = self.calculate_depth(edge.source(), visited);
            max_depth = max_depth.max(depth);
        }

        visited.remove(&node);
        max_depth + 1
    }

    pub fn topological_sort(&self) -> Result<Vec<String>, String> {
        match toposort(&self.graph, None) {
            Ok(sorted) => {
                Ok(sorted.iter().map(|&idx| self.graph[idx].clone()).collect())
            }
            Err(_) => Err("Circular dependency detected".to_string()),
        }
    }

    pub fn has_circular_dependencies(&self) -> bool {
        self.topological_sort().is_err()
    }

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
