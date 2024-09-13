use std::collections::HashMap;
use tokio::time::{sleep, Duration};
use std::sync::{Arc, Mutex};
use crate::resp::Value;

pub struct Store {
    pub map:Arc<Mutex<HashMap<String, String>>>,
}

impl Store {
    pub fn new() -> Self {
        Store {
            map: Arc::new(Mutex::new(HashMap::new())),
        }
    }
    
    // set a key value
    pub fn set(&mut self, key: String, value: String, ttl: Option<Duration>) {
        let mut store = self.map.lock().unwrap();
        store.insert(key.clone(), value);

        if let Some(ttl) = ttl {
            let store_clone = Arc::clone(&self.map);
            tokio::spawn(async move {
                sleep(ttl).await;
                let mut store = store_clone.lock().unwrap();
                store.remove(&key);
            });
        }
    }

    // get a value
    pub fn get(&mut self, key: &Value) -> Value {
        let mut value = "";
        let store = self.map.lock().unwrap();
        if !store.contains_key(key.to_string().as_str()) {
            return Value::BulkString(value.to_string());
        }

        value = store.get(key.clone().to_string().as_str()).unwrap();
        return Value::BulkString(value.to_string());
    }
}