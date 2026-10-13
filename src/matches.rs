use crate::value::Value;
use std::collections::HashMap;

/// The result of a successful `Parser::parse` call; the values the user
/// gave, plus the defaults of anything they left out.
///
/// Look a value up by the name you gave its `Arg`, using the getter that
/// matches its value_kind.
///
/// # Example
///
/// ```
/// use parse_clargs::{Arg, Parser, Value, ValueKind};
///
/// # fn main() -> Result<(), String> {
///     let parser = Parser::new("demo")
///         .arg(Arg::new("name").long("name").value_kind(ValueKind::Str))?
///         .arg(Arg::new("count").long("count").value_kind(ValueKind::Num)
///             .default_value(Value::Num(1)))?
///         .arg(Arg::new("verbose").long("verbose").value_kind(ValueKind::Bool))?;
///
///     let args = vec!["--name".to_string(), "Ada".to_string(), "--verbose".to_string()];
///     let matches = parser.parse(&args)?;
///
///     assert_eq!(matches.get_str("name"), Some("Ada"));
///     assert_eq!(matches.get_num("count"), Some(1));  // not given so the default
///     assert!(matches.is_present("verbose"));
///     assert_eq!(matches.get_num("name"), None);  // wwrong getter for string flag
///     # Ok(())
/// # }
/// ```
pub struct Matches {
    values: HashMap<String, Value>,
}

impl Matches {
    /// Creates a new empty `Matches`
    /// # Example
    /// ```
    /// use parse_clargs::Matches;
    ///
    /// let matches = Matches::new();
    /// ```
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
        }
    }

    /// Return the value stored under `name`.
    ///
    /// Returns `None` if the argument is not given and has no default,
    pub fn get(&self, name: &str) -> Option<&Value> {
        self.values.get(name)
    }

    /// Returns `true` if `name` argument was given or has a default value.
    pub fn is_present(&self, name: &str) -> bool {
        self.values.contains_key(name)
    }

    /// Adds a tuple of an arguments name as the key and its value as
    /// to the matches structure
    pub fn add(&mut self, key: &str, value: Value) {
        self.values.insert(key.to_string(), value);
    }

    /// Returns a reference to a Hashmap of all the arguments passed as well as the
    /// arguments with their defaults
    pub fn get_values(&self) -> &HashMap<String, Value> {
        &self.values
    }

    /// Return the number stored under `key`
    ///
    /// Return `None` if the argument wasn't given and has no default, or if
    /// it is not a number argument
    pub fn get_num(&self, key: &str) -> Option<u64> {
        match self.values.get(key) {
            Some(Value::Num(value)) => Some(*value),
            _ => None,
        }
    }

    /// Return the string stored under `key`
    ///
    /// Return `None` if the argument wasn't given and has no default, or if
    /// it is not a string argument
    pub fn get_str(&self, key: &str) -> Option<&str> {
        match self.values.get(key) {
            Some(Value::Str(value)) => Some(value.as_str()),
            _ => None,
        }
    }

    /// Return the bool stored under `key`
    ///
    /// Return `None` if the argument wasn't given and has no default, or if
    /// it is not a bool argument
    pub fn get_bool(&self, key: &str) -> Option<bool> {
        match self.values.get(key) {
            Some(Value::Bool(value)) => Some(*value),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stored_value_can_be_read_back() {
        let mut m = Matches::new();
        m.add("count", Value::Num(8));
        assert!(m.is_present("count"));
        assert_eq!(m.get_num("count"), Some(8));
    }

    #[test]
    fn missing_key_gives_none() {
        let m = Matches::new();
        assert!(!m.is_present("count"));
        assert_eq!(m.get_num("count"), None);
    }

    #[test]
    fn wrong_type_gives_none() {
        let mut m = Matches::new();
        m.add("name", Value::Str("ada".to_string()));
        assert_eq!(m.get_num("name"), None);
        assert_eq!(m.get_str("name"), Some("ada"));
    }
}
