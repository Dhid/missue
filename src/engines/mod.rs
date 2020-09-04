use crate::Result;

pub trait MissueEngine {
    /// Sets the value of a string key to a string.
    ///
    /// If the key already exists, the previous value will be overwritten.
    fn set(&mut self, key: String, value: String) -> Result<()>;

    /// Gets the string value of a given string key.
    ///
    /// Returns `None` if the given key does not exist.
    fn get(&mut self, key: String) -> Result<Option<String>>;

    /// Removes a given key.
    fn remove(&mut self, key: String) -> Result<()>;

    /// Returns the list of all the issues
    fn all(&mut self) -> Result<Vec<String>>;
    
    /// Returns the list of all open issues
    fn open(&mut self) -> Result<Vec<String>>;

    /// Checks if a key is in the database
    fn has(&self, key: &String) -> bool;
}

mod kvs;

pub use self::kvs::KvStore;
