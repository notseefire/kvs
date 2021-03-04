#[macro_use] extern crate failure;
#[macro_use] extern crate failure_derive;

use serde::{Serialize, Deserialize};
use failure::Error;
use std::{collections::HashMap, fs::OpenOptions, u32, u64};
use std::path::PathBuf;
use std::fs::{File, metadata};
use serde_json;
use serde_json::{from_str};
use std::io::prelude::*;
use std::io::{BufReader, SeekFrom};

#[derive(Debug, Fail)]
pub enum CommandError {
    #[fail(display = "Command {} is not valid", _0)]
    CommandInvalidError(String),
    #[fail(display = "You need input {} arguments. It is far enough", _0)]
    ArgumenInvalidErorr(u32),
    #[fail(display = "GetError")]
    GetCommandError(String),
    #[fail(display = "SetError")]
    SetCommandError(String),
    #[fail(display = "RmError")]
    RmCommandError(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Command {
    Set {
        key: String,
        value: String,
    },
    Get {
        key: String,
    },
    Remove {
        key: String
    },
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Log {
    tstamp: u32,
    key: String,
    value: Command,
}

#[derive(Debug, Serialize, Deserialize)]
struct Hash {
    file_id: String,
    value_pos: u64,
    tstamp: u32,
}

pub type CommandResult<T> = Result<T, CommandError>;

/// A struct wrapper HashMap
pub struct KvStore {
    hash_map: HashMap<String, Hash>,
    id: String,
    path_buf: PathBuf,
}

impl KvStore {

    pub fn open (path: impl Into<PathBuf>) -> Result<KvStore, Error> {
        let mut hash_map = HashMap::new();
        let mut path_buf: PathBuf = path.into();
        // check if path exist
        metadata(path_buf.clone()).unwrap();
        // ./head is id file
        path_buf = path_buf.join("head");

        let mut head = match File::open(path_buf.clone()) {
            Ok(head) => {head},
            Err(_) => {
                let mut file = File::create(path_buf.clone()).unwrap();
                file.write(b"0").unwrap();
                File::open(path_buf.clone()).unwrap()
            },
        };
        let mut id = String::new();
        head.read_to_string(&mut id).unwrap();
        // pop to make ./
        path_buf.pop();
        let index = id.parse::<u32>().unwrap();
        for i in 0..=index {
            let index_i = i.clone().to_string();
            let path_data = path_buf.join(index_i.clone());
            let file_data = File::open(path_data).unwrap();
            let mut buf_reader = BufReader::with_capacity(1024, file_data);
            let mut pos = 0;
            loop {
                let mut buf_line = Vec::new();
                match buf_reader.read_until(b' ', &mut buf_line) {
                    Ok(_) => {
                        let line = String::from_utf8(buf_line).unwrap();
                        if line.len() == 0 {
                            break;
                        }
                        let log: Log = from_str(&line).unwrap();
                        let key = log.key;
                        let value = Hash {
                            file_id: index_i.clone(),
                            value_pos: pos,
                            tstamp: 0,
                        };
                        hash_map.insert(key, value);
                        pos += (line.len()) as u64;
                    },
                    Err(_) => {
                        break;
                    }
                }
            }
        }
        Ok(
            KvStore {
                hash_map,
                id,
                path_buf: path_buf.clone(),
            }
        )
    }

    fn write_log(&mut self, key:String, log: Log) -> Result<(), Error> {
        let mut path_buf = self.path_buf.clone();
        path_buf = path_buf.join(self.id.clone());
        let mut file = OpenOptions::new().read(true)
            .write(true)
            .create(true)
            .open(path_buf).unwrap();
        let mut log = serde_json::to_string(&log).unwrap();
        log.push(' ');
        let mut log = log.into_bytes();
        let pos = file.seek(SeekFrom::End(0)).unwrap();
        file.write(&mut log).unwrap();
        let hash = Hash {
            file_id: self.id.clone(),
            value_pos: pos,
            tstamp: 0,
        };
        self.hash_map.insert(key, hash);
        Ok(())
    }

    /// create a new instance of KvStore
    /// # Example
    /// ```rust
    /// use kvs::KvStore;
    /// let db = KvStore::new()
    /// ```
    pub fn new() -> Self{
        KvStore{
            hash_map: HashMap::new(),
            id: "0".to_string(),
            path_buf: PathBuf::from("./")
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
    pub fn set(&mut self, key: String, value: String) -> CommandResult<()> {
        let value = Command::Set{
            key: key.clone(),
            value,
        };

        let log = Log {
            tstamp: 0,
            key: key.clone(),
            value,
        };
        self.write_log(key, log).unwrap();
        Ok(())
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
    pub fn get(&self, key: String) -> CommandResult<Option<String>> {
        match self.hash_map.get(&key) {
            Some(hash) => {
                let id = hash.file_id.clone();
                let path_buf = self.path_buf.clone();
                let file = File::open(path_buf.join(id)).unwrap();
                let mut bufreader = BufReader::new(file);
                bufreader.seek(SeekFrom::Start(hash.value_pos)).unwrap();
                let mut log = Vec::new();
                bufreader.read_until(b' ', &mut log).unwrap();
                let log = String::from_utf8(log).unwrap();
                let log: Log = from_str(&log).unwrap();
                match log.value {
                    Command::Set { key: _, value } => {
                        Ok(Some(value))
                    },
                    Command::Remove { key: _ } => {
                        Ok(None)
                    }
                    Command::Get { key: _ } => {Ok(None)}
                }
            },
            None => {Ok(None)}
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
    pub fn remove(&mut self, key: String) -> CommandResult<()> {
        let value = Command::Remove{
            key: key.clone(),
        };

        let log = Log {
            tstamp: 0,
            key: key.clone(),
            value,
        };
        self.write_log(key, log).unwrap();
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
