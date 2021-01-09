use std::collections::HashMap;

/// A struct wrapper HashMap
pub struct KvStore {
    db: HashMap<String, String>
}

impl KvStore {

    /// create a new instance of KvStore
    /// # Example
    /// ```rust
    /// use kvs::KvStore;
    /// let db = KvStore::new()
    /// ```
    pub fn new() -> Self{
        let db = HashMap::new();
        KvStore{
            db
        }
    }

    /// set a key with value in KvStore <br/>
    /// 连这都看不懂只能说明你不适合干这行兄弟
    /// # Example
    /// over write value as this way
    /// ```rust
    /// use kvs::KvStore;
    /// let mut store = KvStore::new();
    /// 
    /// store.set("key1".to_owned(), "value1".to_owned());
    /// assert_eq!(store.get("key1".to_owned()), Some("value1".to_owned()));
    /// 
    /// store.set("key1".to_owned(), "value2".to_owned());
    /// assert_eq!(store.get("key1".to_owned()), Some("value2".to_owned()));
    /// ```
    pub fn set(&mut self, key: String, value: String) {
        self.db.insert(key, value);
    }

    /// get a value by a key in KvStore
    /// # Example
    /// before get a value, you need to set key with value
    /// ```rust
    /// use kvs::KvStore;
    /// let mut store = KvStore::new();
    ///
    /// store.set("key1".to_owned(), "value1".to_owned());
    /// store.set("key2".to_owned(), "value2".to_owned());
    /// 
    /// assert_eq!(store.get("key1".to_owned()), Some("value1".to_owned()));
    /// assert_eq!(store.get("key2".to_owned()), Some("value2".to_owned()));
    /// ```
    /// This is a example of non-set key
    /// ```rust
    /// use kvs::KvStore;
    /// let mut store = KvStore::new();
    /// 
    /// store.set("key1".to_owned(), "value1".to_owned());
    /// assert_eq!(store.get("key2".to_owned()), None);
    /// ```
    pub fn get(&self, key: String) -> Option<String> {
        match self.db.get(&key) {
            Some(value) => Some(value.to_owned()),
            None => {None}
        }
    }

    /// remove a key from KvStore
    /// # Example
    /// remove a existing key by this way.
    /// it will do nothing for non-existing key
    /// ```rust
    /// use kvs::KvStore;
    /// let mut store = KvStore::new();
    /// 
    /// store.set("key1".to_owned(), "value1".to_owned());
    /// store.remove("key1".to_owned());
    /// assert_eq!(store.get("key1".to_owned()), None);
    /// ```
    pub fn remove(&mut self, key: String) {
        self.db.remove(&key);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
