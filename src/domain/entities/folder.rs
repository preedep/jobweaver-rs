use serde::{Deserialize, Serialize};
use super::Job;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum FolderType {
    Simple,
    Smart,
    Table,
    SmartTable,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Folder {
    pub folder_name: String,
    pub datacenter: Option<String>,
    pub folder_type: FolderType,
    pub application: Option<String>,
    pub description: Option<String>,
    pub owner: Option<String>,
    pub jobs: Vec<Job>,
    pub sub_folders: Vec<Folder>,
}

impl Folder {
    pub fn new(folder_name: String, folder_type: FolderType) -> Self {
        Self {
            folder_name,
            datacenter: None,
            folder_type,
            application: None,
            description: None,
            owner: None,
            jobs: Vec::new(),
            sub_folders: Vec::new(),
        }
    }

    pub fn add_job(&mut self, job: Job) {
        self.jobs.push(job);
    }

    pub fn add_sub_folder(&mut self, folder: Folder) {
        self.sub_folders.push(folder);
    }

    pub fn total_jobs(&self) -> usize {
        let mut count = self.jobs.len();
        for sub_folder in &self.sub_folders {
            count += sub_folder.total_jobs();
        }
        count
    }

    pub fn all_jobs(&self) -> Vec<&Job> {
        let mut jobs = Vec::new();
        for job in &self.jobs {
            jobs.push(job);
        }
        for sub_folder in &self.sub_folders {
            jobs.extend(sub_folder.all_jobs());
        }
        jobs
    }

    pub fn depth(&self) -> usize {
        if self.sub_folders.is_empty() {
            1
        } else {
            1 + self.sub_folders.iter().map(|f| f.depth()).max().unwrap_or(0)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_folder() {
        let folder = Folder::new("TEST_FOLDER".to_string(), FolderType::Simple);
        assert_eq!(folder.folder_name, "TEST_FOLDER");
        assert_eq!(folder.total_jobs(), 0);
    }

    #[test]
    fn test_add_job() {
        let mut folder = Folder::new("TEST_FOLDER".to_string(), FolderType::Simple);
        let job = Job::new("JOB1".to_string(), "TEST_FOLDER".to_string());
        folder.add_job(job);
        assert_eq!(folder.total_jobs(), 1);
    }

    #[test]
    fn test_folder_depth() {
        let mut folder = Folder::new("ROOT".to_string(), FolderType::Smart);
        let mut sub_folder = Folder::new("SUB1".to_string(), FolderType::Smart);
        let sub_sub_folder = Folder::new("SUB2".to_string(), FolderType::Smart);
        
        sub_folder.add_sub_folder(sub_sub_folder);
        folder.add_sub_folder(sub_folder);
        
        assert_eq!(folder.depth(), 3);
    }
}
