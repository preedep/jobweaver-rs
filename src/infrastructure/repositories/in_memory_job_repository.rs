//! In-Memory Job Repository implementation
//!
//! This module provides an in-memory implementation of the JobRepository trait
//! using a HashMap for storage. Suitable for testing and small-scale use cases.

use std::collections::HashMap;
use anyhow::Result;
use crate::domain::entities::Job;
use crate::domain::repositories::JobRepository;

/// In-memory implementation of the JobRepository trait
///
/// Stores jobs in a HashMap with job names as keys. This implementation
/// is fast for lookups but does not persist data between application runs.
/// Ideal for testing, prototyping, or scenarios where persistence is not required.
pub struct InMemoryJobRepository {
    /// Internal storage mapping job names to Job entities
    jobs: HashMap<String, Job>,
}

impl InMemoryJobRepository {
    /// Creates a new empty InMemoryJobRepository
    ///
    /// # Returns
    ///
    /// A new InMemoryJobRepository instance with no jobs
    pub fn new() -> Self {
        Self {
            jobs: HashMap::new(),
        }
    }
}

impl Default for InMemoryJobRepository {
    fn default() -> Self {
        Self::new()
    }
}

impl JobRepository for InMemoryJobRepository {
    /// Adds a job to the repository
    ///
    /// If a job with the same name already exists, it will be replaced.
    ///
    /// # Arguments
    ///
    /// * `job` - The job to add
    ///
    /// # Returns
    ///
    /// Ok(()) on success
    fn add(&mut self, job: Job) -> Result<()> {
        self.jobs.insert(job.job_name.clone(), job);
        Ok(())
    }

    /// Retrieves a job by its name
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the job to retrieve
    ///
    /// # Returns
    ///
    /// Some(&Job) if found, None otherwise
    fn get_by_name(&self, name: &str) -> Option<&Job> {
        self.jobs.get(name)
    }

    /// Retrieves all jobs in the repository
    ///
    /// # Returns
    ///
    /// Vector of references to all jobs
    fn get_all(&self) -> Vec<&Job> {
        self.jobs.values().collect()
    }

    /// Finds all jobs belonging to a specific folder
    ///
    /// # Arguments
    ///
    /// * `folder_name` - The name of the folder to search for
    ///
    /// # Returns
    ///
    /// Vector of references to jobs in the specified folder
    fn find_by_folder(&self, folder_name: &str) -> Vec<&Job> {
        self.jobs
            .values()
            .filter(|job| job.folder_name == folder_name)
            .collect()
    }

    /// Returns the total number of jobs in the repository
    ///
    /// # Returns
    ///
    /// The count of jobs
    fn count(&self) -> usize {
        self.jobs.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Tests adding a job and retrieving it by name
    #[test]
    fn test_add_and_get_job() {
        let mut repo = InMemoryJobRepository::new();
        let job = Job::new("TEST_JOB".to_string(), "TEST_FOLDER".to_string());
        
        repo.add(job).unwrap();
        assert_eq!(repo.count(), 1);
        
        let retrieved = repo.get_by_name("TEST_JOB");
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().job_name, "TEST_JOB");
    }

    /// Tests finding jobs by folder name
    #[test]
    fn test_find_by_folder() {
        let mut repo = InMemoryJobRepository::new();
        
        let job1 = Job::new("JOB1".to_string(), "FOLDER_A".to_string());
        let job2 = Job::new("JOB2".to_string(), "FOLDER_A".to_string());
        let job3 = Job::new("JOB3".to_string(), "FOLDER_B".to_string());
        
        repo.add(job1).unwrap();
        repo.add(job2).unwrap();
        repo.add(job3).unwrap();
        
        let folder_a_jobs = repo.find_by_folder("FOLDER_A");
        assert_eq!(folder_a_jobs.len(), 2);
    }
}
