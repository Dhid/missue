use crate::*;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub struct MissueClient<E: MissueEngine> {
    pub engine: E,
}

impl<E: MissueEngine> MissueClient<E> {
    pub fn new(engine: E) -> Self {
        MissueClient { engine }
    }

    pub fn write_or_update(&mut self, issue: Issue) -> Result<()> {
        let key = get_hashed_key(&issue.name);
        let mut issue_to_write = serde_json::to_string(&issue)?;
        if self.engine.has(&key) {
            let mut old_issue = self.get(issue.name.clone())?;
            if let Some(description) = issue.description {
                old_issue.with_description(Some(description));
            }
            old_issue.with_status(Some(issue.status));
            issue_to_write = serde_json::to_string(&old_issue)?;
        }
        self.engine.set(key, issue_to_write)?;
        Ok(())
    }

    pub fn get(&mut self, key: String) -> Result<Issue> {
        let val = self.engine.get(get_hashed_key(&key))?;
        if let Some(val) = val {
            let res = serde_json::from_str(&*val)?;
            Ok(res)
        } else {
            Err(MissueError::KeyNotFound)
        }
    }

    pub fn remove(&mut self, key: String) -> Result<()> {
        self.engine.remove(get_hashed_key(&key))
    }

    pub fn all(&mut self) -> Result<Vec<String>> {
        self.engine.all()
    }
    
    pub fn open(&mut self) -> Result<Vec<String>> {
        self.engine.open()
    }
}

fn get_hashed_key(name: &String) -> String {
    let mut hasher = DefaultHasher::new();
    let issue = Issue::new(name.clone());
    issue.name.hash(&mut hasher);
    hasher.finish().to_string()
}
