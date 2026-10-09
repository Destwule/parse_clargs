use crate::value::{Value, ValueKind};

#[derive(Clone)]
pub struct Arg {
    short_name: String,
    long_name: String,
    help_text: String,
    takes_value: bool,
    required: bool,
    value_kind: ValueKind,
    default_value: Option<Value>,
    positional: bool
}

impl Arg {
    pub fn new(long_name: &str) -> Self {
        Self {
            long_name: long_name.to_string(),
            short_name: String::new(),
            help_text: String::new(),
            takes_value: true,
            required: false,
            value_kind: ValueKind::UNSPECIFIED,
            default_value: None,
            positional: false
        }
    }

    /// Positional arguments usually appear at the end of the
    /// cli args list, so any positional arguments created will be
    /// checked for at the end of the cli args
    pub fn positional(long_name: &str) -> Self {
        Self {
            positional: true,
            ..Self::new(long_name)
        }
    }

    pub fn long(mut self, value: &str) -> Self {
        self.long_name = value.to_string();
        self
    }

    pub fn short(mut self, value: &str) -> Self {
        self.short_name = value.to_string();
        self
    }

    pub fn help(mut self, help_text: &str) -> Self {
        self.help_text.push_str(help_text);
        self
    }

    pub fn takes_value(mut self, value: bool) -> Self {
        self.takes_value = bool;
        self
    }

    pub fn required(mut self) -> Self {
        self.required = true;
        self
    }

    pub fn value_kind(mut self, value: ValueKind) -> Self {
        self.value_kind = value;
        self
    }

    pub fn default_value(mut self, value: Value) -> Self {
        self.default_value = Some(value);
        self
    }

    pub fn long_flag(&self) -> String {
        if !self.get_long_name().is_empty() {
            return format!("--{}", self.get_long_name());
        }

        String::new()
    }
    
    pub fn short_flag(&self) -> String {
        if !self.get_short_name().is_empty() {
            return format!("-{}", self.get_short_name());
        }

        String::new()
    }

    pub fn get_long_name(&self) -> &String { &self.long_name }
    pub fn get_short_name(&self) -> &String { &self.short_name }
    pub fn get_takes_value(&self) -> bool { self.takes_value }
    pub fn get_value_kind(&self) -> &ValueKind { &self.value_kind }
    pub fn get_help_text(&self) -> &String { &self.help_text }
    pub fn is_required(&self) -> bool { self.required }
    pub fn is_positional(&self) -> bool { self.positional }
    pub fn get_default_value(&self) -> &Option<Value> { &self.default_value }


    pub(crate) fn register(self) -> Result<Self, String> {
        let arg_name = if self.get_long_name().is_empty() {
            self.get_short_name()
        } else { self.get_long_name() };

        if self.get_long_name().is_empty() && self.get_short_name().is_empty() {
            return Err("short_name and long_name cannot be empty".to_string());
        }

        if self.value_kind == ValueKind::UNSPECIFIED {
            return Err(format!("value_kind of '{arg_name}' cannot be 'unspecified'"));
        }

        match self.default_value {
            Some(Value::Str(_)) => {
                if self.value_kind != ValueKind::Str {
                    return Err(format!("DefaultValue of '{arg_name}' must be the same type as value_kind"));
                }
            }
            Some(Value::Num(_)) => {
                if self.value_kind != ValueKind::Num {
                    return Err(format!("DefaultValue of '{arg_name}' must be the same type as value_kind"));
                }
            }
            Some(Value::Bool(_)) => {
                if self.value_kind != ValueKind::Bool {
                    return Err(format!("DefaultValue of '{arg_name}' must be the same type as value_kind"));
                }
            }
            None => ()
        }
        Ok(self)
    }
}