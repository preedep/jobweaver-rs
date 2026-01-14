use crate::domain::entities::Job;
use anyhow::Result;

pub trait JobRepository {
    fn add(&mut self, job: Job) -> Result<()>;
    fn get_by_name(&self, name: &str) -> Option<&Job>;
    fn get_all(&self) -> Vec<&Job>;
    fn find_by_folder(&self, folder_name: &str) -> Vec<&Job>;
    fn count(&self) -> usize;
}
