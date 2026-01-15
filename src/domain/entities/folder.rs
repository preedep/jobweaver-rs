//! Folder entity module
//!
//! This module defines the folder structure for organizing jobs.
//! Folders can contain jobs and sub-folders in a hierarchical structure.

use serde::{Deserialize, Serialize};
use super::Job;

/// Types of folders in Control-M
///
/// Different folder types have different capabilities and behaviors.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum FolderType {
    /// Simple folder - basic job container
    Simple,
    /// Smart folder - with advanced features
    Smart,
    /// Table folder - for table-based scheduling
    Table,
    /// Smart table folder - combination of smart and table features
    SmartTable,
}

/// Represents a folder containing jobs and sub-folders
///
/// Folders provide hierarchical organization of jobs and can contain
/// both jobs and other folders, forming a tree structure.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Folder {
    /// Name of the folder
    pub folder_name: String,
    /// Datacenter where this folder is located
    pub datacenter: Option<String>,
    /// Type of folder
    pub folder_type: FolderType,
    /// Application this folder belongs to
    pub application: Option<String>,
    /// Description of the folder's purpose
    pub description: Option<String>,
    /// Owner of the folder
    pub owner: Option<String>,
    /// Jobs directly contained in this folder
    pub jobs: Vec<Job>,
    /// Sub-folders contained in this folder
    pub sub_folders: Vec<Folder>,
}

impl Folder {
    /// Creates a new folder
    ///
    /// # Arguments
    ///
    /// * `folder_name` - Name of the folder
    /// * `folder_type` - Type of folder
    ///
    /// # Returns
    ///
    /// A new Folder instance with empty job and sub-folder lists
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

    /// Adds a job to this folder
    ///
    /// # Arguments
    ///
    /// * `job` - The job to add
    pub fn add_job(&mut self, job: Job) {
        self.jobs.push(job);
    }

    /// Adds a sub-folder to this folder
    ///
    /// # Arguments
    ///
    /// * `folder` - The sub-folder to add
    pub fn add_sub_folder(&mut self, folder: Folder) {
        self.sub_folders.push(folder);
    }

    /// Counts the total number of jobs in this folder and all sub-folders
    ///
    /// # Returns
    ///
    /// Total count of jobs (recursive)
    pub fn total_jobs(&self) -> usize {
        let mut count = self.jobs.len();
        for sub_folder in &self.sub_folders {
            count += sub_folder.total_jobs();
        }
        count
    }

    /// Collects all jobs from this folder and all sub-folders
    ///
    /// # Returns
    ///
    /// A vector of references to all jobs (recursive)
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

    /// Calculates the depth of the folder hierarchy
    ///
    /// # Returns
    ///
    /// The maximum depth of the folder tree (1 for leaf folders)
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
