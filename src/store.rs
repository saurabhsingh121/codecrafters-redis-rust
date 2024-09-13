use std::collections::HashMap;

use crate::resp::Value;

pub struct Store {
    pub map:HashMap<String, String>,
}

impl Store {
    pub fn new() -> Self {
        Store {
            map: HashMap::new(),
        }
    }
    
    // set a key value
    pub fn set(&mut self, key: String, value: String) -> Value {
        self.map.insert(key, value);
        Value::SimpleString("OK".to_string())
    }

    // get a value
    pub fn get(&mut self, key: &Value) -> Value {
        let mut value = "";
        if !self.map.contains_key(key.to_string().as_str()) {
            return Value::BulkString(value.to_string());
        }

        value = self.map.get(key.clone().to_string().as_str()).unwrap();
        return Value::BulkString(value.to_string());
    }
}