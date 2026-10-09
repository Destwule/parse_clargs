
use crate::value::Value;
use std::collections::HashMap;

pub struct Matches {
    values: HashMap<String, Value>
}

impl Matches {
    pub fn new() -> Self {
        Self { values: HashMap::new() }
    }

    pub fn get(&self, name: &str) -> Option<&Value> {
        self.values.get(name)
    }

    pub fn is_present(&self, name: &str) -> bool {
        self.values.contains_key(name)
    }

    pub fn add(&mut self, key: &str, value: Value) {
        self.values.insert(key.to_string(), value);
    }

    pub fn get_values(&self) -> &HashMap<String, Value> {
        &self.values
    }

    pub fn get_num(&self, key: &str) -> Option<u64> {
        match self.values.get(key) {
            Some(Value::Num(value)) => Some(*value),
            _ => None
        }
    }

    pub fn get_str(&self, key: &str) -> Option<&str> {
        match self.values.get(key) {
            Some(Value::Str(value)) => Some(value.as_str()),
            _ => None
        }
    }

    pub fn get_bool(&self, key: &str) -> Option<bool> {
        match self.values.get(key) {
            Some(Value::Bool(value)) => Some(*value),
            _ => None
        }
    }
}