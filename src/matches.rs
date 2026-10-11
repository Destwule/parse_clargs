use crate::value::Value;
use std::collections::HashMap;

/// The result of a successful ['Parser::parse'] call; the values the user
/// gave, plus the defaults of anything they left out/
///
/// Look a value up by the name you gave its ['Arg'], using the getter that
/// matches its ['ValueKind'].
///
/// # Example
///
/// ```
/// use parse_clargs::{Arg, Parser, Value, ValueKind};
///
/// # fn main() -> Result<<(), String> {
///     let parser = Parser::new("demo")
///         .arg(Arg::name("name").long("name").value_kind(ValueKind::Str))?
///         .arg(Arg::name("count").long("count").value_kind(ValueKind::Num)
///             .default_value(Value::Num(1))?
///         .arg(Arg::name("verbose").long("verbose").value_kind(ValueKind::Bool))?;
/// 
///     let args = vec!["--name".to_string(), "Ada".to_string(), "--verbose".to_string()]
///     let matches = parser.parse(&args)?;
///
///     assert_eq!(matches.get_str("name"), Some("Ada"));
///     assert_eq!(matches.get_num("count"), Some(1"));  // not given so the default
///     assert!(matches.is_present("verbose"));
///     assert_eq!(matches.get_num("name"), None);  // wwrong getter for string flag
///     # Ok(())
/// # }
/// ```
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