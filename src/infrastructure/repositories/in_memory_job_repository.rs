use std::collections::HashMap;
use anyhow::Result;
use crate::domain::entities::Job;
use crate::domain::repositories::JobRepository;

pub struct InMemoryJobRepository {
    jobs: HashMap<String, Job>,
}

impl InMemoryJobRepository {
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
    fn add(&mut self, job: Job) -> Result<()> {
        self.jobs.insert(job.job_name.clone(), job);
        Ok(())
    }

    fn get_by_name(&self, name: &str) -> Option<&Job> {
        self.jobs.get(name)
    }

    fn get_all(&self) -> Vec<&Job> {
        self.jobs.values().collect()
    }

    fn find_by_folder(&self, folder_name: &str) -> Vec<&Job> {
        self.jobs
            .values()
            .filter(|job| job.folder_name == folder_name)
            .collect()
    }

    fn count(&self) -> usize {
        self.jobs.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
