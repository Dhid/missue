use crate::*;
use serde::{Deserialize, Serialize};
use chrono::prelude::{DateTime, Local};
use std::fmt;
use std::path::PathBuf;

/// Issue implements the builder pattern for build an issue with optional parameters
#[derive(Debug, Serialize, Deserialize, Hash)]
pub struct Issue {
    pub name: String,
    pub description: Option<String>,
    pub status: Status,
    pub creation_time: DateTime<Local>,
    pub due_time: Option<DateTime<Local>>,
    pub file_path: Option<PathBuf>
}

impl Issue {
    /// Creates a basic issue with the given name and status OPEN
    pub fn new(name: String) -> Self {
        Self {
            name: name,
            description: None,
            status: Status::OPEN,
            creation_time: Local::now(),
            due_time: None,
            file_path: None,
        }
    }

    pub fn with_due_time(&mut self, due: DateTime<Local>) -> &mut Issue {
        self.due_time = Some(due);
        self
    }
    
    pub fn with_file_path(&mut self, path: Option<PathBuf>) -> &mut Issue {
        self.file_path = path;
        self
    }

    pub fn with_description(&mut self, descr: Option<String>) -> &mut Issue {
        self.description = descr;
        self
    }
    
    pub fn with_status(&mut self, status: Option<Status>) -> &mut Issue {
        if let Some(status) = status {
            self.status = status;
        }
        self
    }

}

impl From<String> for Issue {
    fn from(item: String) -> Self {
        let issue = serde_json::from_str(&*item);
        match issue {
            Ok(issue) => issue,
            _ => panic!("Cannot convert issue")
        }
    }
}
impl fmt::Display for Issue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let write = serde_json::to_string(&self);
        match write {
            Ok(value) => write!(f, "{}", value),
            Err(e) => write!(f, "Error while printing Issue: {}", e)
        }
        
    }
}