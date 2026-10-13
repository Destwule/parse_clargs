use crate::value::{Value, ValueKind};

/// This represents a single command line argument.
///
///
/// After a command line argument is created, it is then added to the
/// `Parser` using the `Parser::arg` function to represent one of the
/// arguments that the parser supports
///
///
/// # Example
/// ```
/// use parse_clargs::{Arg, Parser, ValueKind};
/// fn main() -> Result<(), String> {
///     let arg_1 = Arg::new("name")        // first arg
///         .long("name").short("n")
///         .help("name of the person to greet")
///         .value_kind(ValueKind::Str);
///
///     let arg_2 = Arg::new("uppercase")       // second argument
///         .long("uppercase").short("u")
///         .help("if provided, the greeting will be in uppercase")
///         .value_kind(ValueKind::Bool);
///
///     let parser = Parser::new("greeter")
///         .description("display greeting message with the 'name' specified;")
///         .arg(arg_1)?
///         .arg(arg_2)?;
///     let args = vec!["--name".to_string(), "Ada".to_string(), "-u".to_string()];
///     let matches = parser.parse(&args)?;
///
///     let name = matches.get_str("name").unwrap();
///     let uppercase = matches.is_present("uppercase");
///
///     let mut greeting = format!("Hello, {name}!!. how are you doing?");
///     if uppercase {
///         greeting = greeting.to_uppercase();
///     }
///     println!("{}", greeting);
///
///     # Ok(())
/// }
/// ```
#[derive(Clone)]
pub struct Arg {
    name: String,
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
    /// This creates a new argument.
    /// It takes a parameter 'name' which is the key used to read the value
    /// from `Matches`. The users don't use it to pass their cli arguments.
    /// This is because at runtime the program does not know what name the users
    /// will use to pass in the cli arguments, so rather than guessing, it is
    /// easier to provide a generic name to reference the argument.
    ///
    /// The `.long()` and `.short()` function are used to define what the users use
    /// to pass cli arguments.
    ///
    /// # Example
    /// ```
    /// use parse_clargs::{Arg, Parser, ValueKind};
    /// # fn main() -> Result<(), String> {
    /// let arg_1 = Arg::new("cols").long("blocks").value_kind(ValueKind::Num);
    ///
    /// let parser = Parser::new("copy").arg(arg_1)?;
    ///
    /// let args = vec!["--blocks".to_string(), "8".to_string()];
    /// let matches = parser.parse(&args)?;
    ///
    /// // cols is used here to get the value of block
    /// assert_eq!(matches.get_num("cols"), Some(8));
    /// # Ok(())
    /// # }
    /// ```
    ///
    ///
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            long_name: String::new(),
            short_name: String::new(),
            help_text: String::new(),
            takes_value: true,
            required: false,
            value_kind: ValueKind::UNSPECIFIED,
            default_value: None,
            positional: false
        }
    }

    /// This creates 'Positional' arguments.
    /// Here positional arguments are not really fixed to a specific position.
    /// they are called without a flag and are matched when the value is not
    /// recognized as a valid parser arguments. they can be put in any position,
    /// NOTE: Since positional arguments are passed without name, they don't use
    ///       or set the long_name or short_name of the argument. The 'name' parameter
    ///       passed in the function def is used to identify the argument
    ///
    /// Positionals are optional unless `.required()`
    /// # Example
    /// ```
    /// use parse_clargs::{Arg, Parser, ValueKind};
    /// # fn main() -> Result<(), String> {
    /// let arg_1 = Arg::new("cols").long("cols").value_kind(ValueKind::Num);
    /// let arg_2 = Arg::positional("file").value_kind(ValueKind::Str);
    ///
    /// let parser = Parser::new("copy").arg(arg_1)?.arg(arg_2)?;
    ///
    /// let args = vec!["--cols".to_string(), "8".to_string(), "data.bin".to_string()];
    /// let matches = parser.parse(&args)?;
    ///
    /// assert_eq!(matches.get_num("cols"), Some(8));
    /// assert_eq!(matches.get_str("file"), Some("data.bin"));
    /// # Ok(())
    /// # }
    /// ```
    pub fn positional(name: &str) -> Self {
        Self {
            positional: true,
            ..Self::new(name)
        }
    }

    /// This sets the long_name of the argument.
    /// The 'value' parameter should be passed without including hyphens
    /// # Example
    /// ```
    /// use parse_clargs::{Arg, Parser, ValueKind};
    /// # fn main() -> Result<(), String> {
    /// let arg = Arg::new("file")
    ///     .long("file")      // used here to set the long name
    ///     .value_kind(ValueKind::Str);
    /// # Ok(())
    /// # }
    ///```
    pub fn long(mut self, value: &str) -> Self {
        self.long_name = value.to_string();
        self
    }

    /// This sets the short_name of the argument
    /// The 'value' parameter should be passed without including hyphens
    /// # Example
    /// ```
    /// use parse_clargs::{Arg, Parser, ValueKind};
    /// # fn main() -> Result<(), String> {
    /// let arg = Arg::new("file")
    ///     .short("f")      // used here to set the short name
    ///     .value_kind(ValueKind::Str);
    /// # Ok(())
    /// # }
    ///```
    pub fn short(mut self, value: &str) -> Self {
        self.short_name = value.to_string();
        self
    }

    /// This sets the help text of the argument. Information on what the argument does
    ///
    /// NOTE: help appends to a string using `String::push_str` internally so calling
    ///       help twice joins the text without space.
    /// # Example
    /// ```
    /// use parse_clargs::{Arg, Parser, ValueKind};
    /// # fn main() -> Result<(), String> {
    /// let arg = Arg::new("file")
    ///     .long("file")
    ///     .help("The file to read from")
    ///     .value_kind(ValueKind::Str);
    /// # Ok(())
    /// # }
    ///```
    pub fn help(mut self, help_text: &str) -> Self {
        self.help_text.push_str(help_text);
        self
    }

    /// this defines whether or not the argument takes a value. if `false` then when the
    /// argument is called, the argument is treated as a boolean value where the name alone
    /// signifies its value. Basically an arguments get forced to `ValueKind::Bool` if its
    /// `take_value(false)`.
    ///
    /// # Example
    /// ```
    /// use parse_clargs::{Arg, Parser, ValueKind};
    /// # fn main() -> Result<(), String> {
    /// let arg = Arg::new("file")
    ///     .long("file")
    ///     .help("The file to read from")
    ///     .value_kind(ValueKind::Str)
    ///     .takes_value(true);
    /// # Ok(())
    /// # }
    ///```
    pub fn takes_value(mut self, value: bool) -> Self {
        self.takes_value = value;
        self
    }

    /// this make the argument a required argument.
    /// required arguments always need to be passed when the program is used
    /// # Example
    /// ```
    /// use parse_clargs::{Arg, Parser, ValueKind};
    /// # fn main() -> Result<(), String> {
    /// let arg = Arg::new("file")
    ///     .long("file")
    ///     .help("The file to read from")
    ///     .value_kind(ValueKind::Str)
    ///     .required();        // used here
    /// # Ok(())
    /// # }
    ///```
    pub fn required(mut self) -> Self {
        self.required = true;
        self
    }
    
    /// specifies what kind of value the argument expects to represent.
    /// The value_kind is used by the corresponding get method of the `Matches`
    /// struct to parse the argument value to its correct type
    /// # Example
    /// ```
    /// use parse_clargs::{Arg, Parser, ValueKind};
    /// # fn main() -> Result<(), String> {
    /// let arg = Arg::new("file")
    ///     .long("file")
    ///     .help("The file to read from")
    ///     .value_kind(ValueKind::Str);        // used here
    /// # Ok(())
    /// # }
    ///```
    pub fn value_kind(mut self, value: ValueKind) -> Self {
        self.value_kind = value;
        self
    }
    
    /// this is used to set the default value of the argument if no value is passed
    /// # Example 
    /// ```
    /// use parse_clargs::{Arg, Parser, ValueKind, Value};
    /// # fn main() -> Result<(), String> {
    /// let arg = Arg::new("file")
    ///     .long("file")
    ///     .help("The file to read from")
    ///     .value_kind(ValueKind::Str)
    ///     .default_value(Value::Str("words.txt".to_string()));        // used here
    /// # Ok(())
    /// # }
    ///```
    pub fn default_value(mut self, value: Value) -> Self {
        self.default_value = Some(value);
        self
    }
    
    /// this defines how the long name of the argument is expected to be passed through
    /// the cli.  This just appends two hyphens to the long_name
    /// # Example 
    /// ```
    /// use parse_clargs::{Arg, Parser, ValueKind};
    /// # fn main() -> Result<(), String> {
    /// let arg = Arg::new("file")
    ///     .long("file")
    ///     .help("The file to read from")
    ///     .value_kind(ValueKind::Str);
    ///
    /// assert_eq!(arg.long_flag(), "--file".to_string());
    /// # Ok(())
    /// # }
    ///```
    pub fn long_flag(&self) -> String {
        if !self.get_long_name().is_empty() {
            return format!("--{}", self.get_long_name());
        }
        
        String::new()
    }
    

    /// this defines how the short name of the argument is expected to be passed through
    /// the cli.  This just appends one hyphen to the short_name
    /// # Example
    /// ```
    /// use parse_clargs::{Arg, Parser, ValueKind};
    /// # fn main() -> Result<(), String> {
    /// let arg = Arg::new("file")
    ///     .short("f")
    ///     .help("The file to read from")
    ///     .value_kind(ValueKind::Str);
    ///
    /// assert_eq!(arg.short_flag(), "-f".to_string());
    /// # Ok(())
    /// # }
    ///```
    pub fn short_flag(&self) -> String {
        if !self.get_short_name().is_empty() {
            return format!("-{}", self.get_short_name());
        }
        
        String::new()
    }
    

    /// returns the generic name used to idnetify the argument
    pub fn get_name(&self) -> &str { self.name.as_str() }

    
    /// returns the long_name of the argument which is used to pass
    /// the argument value to the program
    pub fn get_long_name(&self) -> &String { &self.long_name }


    /// returns the short_name of the argument which is used to pass
    /// the argument value to the program
    pub fn get_short_name(&self) -> &String { &self.short_name }


    /// return true if the argument takes a value and false otherwise.
    /// the default is `true`
    pub fn get_takes_value(&self) -> bool { self.takes_value }


    /// returns the valuekind of the argument
    pub fn get_value_kind(&self) -> &ValueKind { &self.value_kind }


    /// returns the information on what the argument does.
    pub fn get_help_text(&self) -> &String { &self.help_text }


    /// returns true if the argument is required and false otherwise.
    /// the default is false
    pub fn is_required(&self) -> bool { self.required }


    /// returns true if the argument is a positional argument and false otherwise
    /// The default is false.
    /// The value is always set to true for args created with [`Arg::positional`]
    pub fn is_positional(&self) -> bool { self.positional }


    /// returns the default value that the arguments is assigned to when no value
    /// is passed.
    pub fn get_default_value(&self) -> &Option<Value> { &self.default_value }


    /// This valideates that the argument is properly defined.
    /// Usually called by [`Parser::arg`]
    /// NOTE: this is a private method so u can't call it directly
    pub(crate) fn register(mut self) -> Result<Self, String> {
        let arg_name = if self.get_long_name().is_empty() {
            self.get_short_name().clone()
        } else { self.get_long_name().clone() };

        if self.get_name().is_empty() {
            return Err("name cannot be empty".to_string());
        }

        if !self.is_positional() && (self.get_long_name().is_empty() && self.get_short_name().is_empty()) {
            return Err(format!("Both short_name and long_name cannot be empty for arg '{}'", self.get_name()));
        }

        if self.value_kind == ValueKind::UNSPECIFIED {
            if !self.is_positional() {
                return Err(format!("value_kind of '{arg_name}' cannot be 'unspecified'"));
            }
            return Err(format!("value_kind of '{}' cannot be 'unspecified'", self.get_name()));
        }

        if *self.get_value_kind() == ValueKind::Bool {
            self = self.takes_value(false);
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




#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn long_flag_adds_two_hyphens() {
        let arg = Arg::new("file").long("file").value_kind(ValueKind::Str);
        assert_eq!(arg.long_flag(), "--file");
    }

    #[test]
    fn flags_are_empty_when_not_set() {
        let arg = Arg::new("file").value_kind(ValueKind::Str);
        assert_eq!(arg.long_flag(), "");
        assert_eq!(arg.short_flag(), "");
    }

    #[test]
    fn short_flag_adds_one_hyphen() {
        let arg = Arg::new("file").short("f").value_kind(ValueKind::Str);
        assert_eq!(arg.short_flag(), "-f");
    }

    #[test]
    fn register_accepts_a_valid_arg() {
        let result = Arg::new("file").long("file").value_kind(ValueKind::Str).register();
        assert!(result.is_ok());
    }

    #[test]
    fn register_rejects_an_empty_name() {
        let result = Arg::new("").long("file").value_kind(ValueKind::Str).register();
        assert!(result.is_err());
    }

    #[test]
    fn register_rejects_an_arg_with_no_flags() {
        let result = Arg::new("file").value_kind(ValueKind::Str).register();
        assert!(result.is_err());
    }

    #[test]
    fn positional_does_not_need_flags() {
        let result = Arg::positional("file").value_kind(ValueKind::Str).register();
        assert!(result.is_ok());
    }

    #[test]
    fn register_rejects_unspecified_value_kind() {
        let result = Arg::new("file").long("file").register();
        assert!(result.is_err());
    }

    #[test]
    fn bool_args_do_not_take_a_value() {
        let arg = Arg::new("verbose").long("verbose")
            .value_kind(ValueKind::Bool)
            .register()
            .unwrap();
        assert!(!arg.get_takes_value());
    }

    #[test]
    fn register_rejects_a_default_of_the_wrong_type() {
        let result = Arg::new("file").long("file")
            .value_kind(ValueKind::Str)
            .default_value(Value::Num(5))
            .register();
        assert!(result.is_err());
    }
}