use crate::{
    value::{Value, ValueKind},
    arg::Arg,
    matches::Matches
};


/// This is the interface that the program uses to define command line arguments that
/// the program expects and accepts.
///
/// You create the parser with `Parser::new`; the param that `new` expects is usually the program name.
/// then give the parser a description, basically what the program does. then you add arguments
/// 
/// Once all arguments are added then `Parser::parse` which takes a list of strings that represent arguments, 
/// validates those arguments against the defined arguments and returns a `Matches` struct.
/// The `Matches` struct contains all the matched arguments which you then use in your code
///
/// # Example
/// ```
/// # use parse_clargs::{Arg, Parser, Value, ValueKind};
/// let arg_1 = Arg::new("name")
///     .long("name")
///     .help("name to include in greeting")
///     .value_kind(ValueKind::Str);
///
///  let arg_2 = Arg::new("uppercase")
///     .long("uppercase").short("u")
///     .help("prints greeting in uppercase")
///     .value_kind(ValueKind::Bool);
///
/// let arg_3 = Arg::positional("age")
///     .help("persons age in numbers")
///     .value_kind(ValueKind::Num);
/// 
/// let args = vec!["--name".to_string(), "Ada".to_string(), "-u".to_string(), "40".to_string()];
///
/// let parser = Parser::new("greeter")
///     .description("prints a greeting message")
///     .arg(arg_1)?
///     .arg(arg_2)?
///     .arg(arg_3)?;
///
/// let matches = parser.parse(&args)?;
/// # Ok::<(), String>(())
/// ```
pub struct Parser {
    name: String,
    description: String,
    args: Vec<Arg>,
    args_index: usize,
    help_on_empty: bool
}

impl Parser {
    /// Creates a new parser. `name` is usually the program's name and appears in the
    /// usage line of the help text. A `--help` argument is registered automatically.
    ///
    /// # Example
    /// ```
    /// # use parse_clargs::Parser;
    /// let parser = Parser::new("greeter");
    /// assert_eq!(parser.get_name(), "greeter");
    /// ```
    pub fn new(name: &str) -> Self {
        let mut demo = Self {
            name: name.to_string(),
            description: String::new(),
            args: vec![],
            args_index: 0,
            help_on_empty: true
        };

        match Arg::new("help").long("help")
            .help("display this help and exit")
            .value_kind(ValueKind::Bool)
            .register()
        {
            Ok(help_arg) => demo.args.insert(0, help_arg),
            Err(err) => {
                eprintln!("error adding help argument in 'new': {err}");
                std::process::exit(1);
            }
        }

        demo
    }

    /// returns the name of the parser
    pub fn get_name(&self) -> &str { &self.name }

    /// returns the description of the parser
    pub fn get_description(&self) -> &str { &self.description }

    /// returns the list of all args that the parser expects, including the automatic `--help`
    pub fn get_args(&self) -> &Vec<Arg> { &self.args }

    /// this returns a boolean value to represent whether the help text is to be
    /// printed when there are no cli arguments passed to the program
    pub fn get_help_on_empty(&self) -> bool { self.help_on_empty }

    /// This adds an argument to the parser. Returns an error if the argument is invalid, for
    /// example  if it has not flags (and isn't positional), has no value kind, or has a
    /// default value of the wrong type
    /// # Example
    /// ```
    /// # use parse_clargs::{Arg, Parser, Value, ValueKind};
    /// Parser::new("My program")
    ///     .arg(Arg::new("name")
    ///         .long("name").help("the name of some random person")
    ///         .value_kind(ValueKind::Str)
    ///     )?
    ///     .arg(Arg::new("uppercase")
    ///         .long("uppercase").help("whether to print in uppercase")
    ///         .value_kind(ValueKind::Bool)
    ///     )?;
    /// # Ok::<(), String>(())
    /// ```
    pub fn arg(mut self, arg: Arg) -> Result<Self, String> {
        let registered = arg.register()?;
        self.args.insert(self.args_index, registered);
        self.args_index += 1;

        Ok(self)
    }

    /// used to set the description message that briefly describes what the
    /// program does
    pub fn description(mut self, description: &str) -> Self {
        self.description = description.to_string();

        self
    }

    /// chooses what happens when the program is run with no arguments.
    ///
    /// When `true` (the default), the help text is printed and the program quits.
    /// Set it to `false` for programs that do something sensible without arguments,
    /// such as hex viewer that reads from stdin.
    ///
    /// # Example
    /// ```
    /// # use parse_clargs::Parser;
    /// let parser = Parser::new("hexview").help_on_empty(false);
    /// assert!(!parser.get_help_on_empty());
    /// ```
    pub fn help_on_empty(mut self, value: bool) -> Self {
        self.help_on_empty = value;

        self
    }

    fn parse_value(arg: &Arg, value: &str, name: &str) -> Result<Value, String> {
        let error_text: String = format!("Error parsing '{name}'");
        let return_value: Value;
        // NOTE: ValueKind::UNSPECIFIED wont run because the register()
        // function check that an argument cannot be unspecified
        match arg.get_value_kind() {
            ValueKind::Bool => {
                return_value = Value::Bool(value.parse::<bool>()
                    .map_err(|err| format!("{error_text}: {err}"))?);
            }
            ValueKind::Str => {
                return_value = Value::Str(value.to_string());
            }
            ValueKind::Num => {
                return_value = Value::Num(value.parse::<u64>()
                    .map_err(|err| format!("{error_text}: {err}"))?);
            }
            _ => return Err("'value kind' is not part of the supported types".to_string())
        }

        Ok(return_value)
    }

    fn value_of_arg(arg: &Arg, cli_args: &[String], name: &str, index: usize) -> Result<Value, String> {
        let return_value: Value;

        if arg.get_takes_value() {
            let token = cli_args.get(index + 1);
            match token {
                None => {
                    if let Some(value) = arg.get_default_value().as_ref() {
                        return Ok(value.clone())
                    }
                    return Err(format!("no value specified for '{name}'"));
                },
                Some(value) => {
                    if value.starts_with("-") {
                        return Err(format!("no value specified for '{name}'"))
                    }
                    return_value = Self::parse_value(arg, value, name)?;
                }
            }
        } else {
            return_value = Value::Bool(true);
        }

        Ok(return_value)
    }

    /// Checks the user's arguments against the ones you defined. Returns the values
    /// as a [`Matches`], or an error message, for example for an unknown flag or a
    /// value that isn't a valid number.
    ///
    /// Pass only the arguments, not the program name. With `std::env::args()`, skip
    /// the first item:
    ///
    /// ```no_run
    /// # use parse_clargs::Parser;
    /// let args: Vec<String> = std::env::args().skip(1).collect();
    /// let result = Parser::new("myprog").parse(&args);
    /// ```
    ///
    /// `--help` anywhere in the arguments prints the help text and exits the program.
    /// So does an empty list, unless [`Parser::help_on_empty`] is set to `false`.
    /// A missing required argument also prints an error and exits.
    ///
    /// # Example
    /// ```
    /// # use parse_clargs::{Arg, Parser, ValueKind};
    /// let parser = Parser::new("greeter")
    ///     .arg(Arg::new("name")
    ///         .long("name")
    ///         .help("name to greet")
    ///         .value_kind(ValueKind::Str))?
    ///     .arg(Arg::new("uppercase")
    ///         .long("uppercase").short("u")
    ///         .help("print the greeting in uppercase")
    ///         .value_kind(ValueKind::Bool))?;
    ///
    /// let args = vec!["--name".to_string(), "Ada".to_string(), "-u".to_string()];
    /// let matches = parser.parse(&args)?;
    ///
    /// assert_eq!(matches.get_str("name"), Some("Ada"));
    /// assert!(matches.is_present("uppercase"));
    ///
    /// // a flag that was never defined is an error, not a panic
    /// assert!(parser.parse(&["--nope".to_string()]).is_err());
    /// # Ok::<(), String>(())
    /// ```
    pub fn parse(&self, cli_args: &[String]) -> Result<Matches, String> {
        let mut matches = Matches::new();
        let mut index = 0;

        if cli_args.iter().any(|a| a.as_str() == "--help")
            || (self.get_help_on_empty() && cli_args.is_empty())
        {
            self.display_help_text();
        }

        let required_flags = self.get_args().iter()
            .filter(|x| x.is_required() && !x.is_positional())
            .collect::<Vec<&Arg>>();
        let positional_args = self.get_args().iter()
            .filter(|x| x.is_positional())
            .collect::<Vec<&Arg>>();
        let mut next_positional = 0usize;

        while index < cli_args.len() {
            let peek = index + 1;
            let after_peek = index + 2;
            let arg_name = &cli_args[index];

            // an empty token never matches a flag (flags without a short name
            // have an empty short_flag(), which would otherwise match "")
            let found = self.args.iter().find(|a| {
                !a.is_positional()
                    && !arg_name.is_empty()
                    && (a.long_flag() == *arg_name || a.short_flag() == *arg_name)
            });

            if let Some(arg) = found {
                matches.add(
                    arg.get_name(),
                    Self::value_of_arg(arg, cli_args, arg_name, index)?
                );

                if arg.get_takes_value() {
                    index = after_peek;
                } else {
                    index = peek;
                }
            }
            else if arg_name.starts_with("-") && arg_name != "-" {
                return Err(format!("Unknown argument '{arg_name}'"));
            }
            else {
                // anything that is not a flag (and was not eaten as a flag's value)
                // is a positional; it goes to the next unfilled positional slot
                let def = positional_args
                    .get(next_positional)
                    .ok_or_else(|| format!("too many arguments: '{arg_name}'"))?;
                matches.add(def.get_name(), Self::parse_value(def, arg_name, def.get_name())?);
                next_positional += 1;
                index = peek;
            }
        }

        // required flags are stored under their neutral name, so look them up by it
        for flag in &required_flags {
            if !matches.is_present(flag.get_name()) {
                let shown = if flag.get_long_name().is_empty() {
                    flag.short_flag()
                } else {
                    flag.long_flag()
                };
                eprintln!("{}: missing {} operand", self.get_name(), shown);
                eprintln!("Try '{} --help' for more information", self.get_name());
                std::process::exit(1);
            }
        }

        // a positional is only mandatory when it was marked .required()
        // and has no default value to fall back on
        for def in &positional_args[next_positional..] {
            if def.is_required() && def.get_default_value().is_none() {
                eprintln!("{}: missing {} operand", self.get_name(), def.get_name().to_uppercase());
                eprintln!("Try '{} --help' for more information", self.get_name());
                std::process::exit(1);
            }
        }

        // Fills in the Default values for those who were not specified
        for arg in self.get_args() {
            if let Some(default) = arg.get_default_value() {
                if !matches.is_present(arg.get_name()) {
                    matches.add(arg.get_name(), default.clone());
                }
            }
        }

        Ok(matches)
    }

    /// generates and returns the help text of the parser from the arguments passed
    pub fn generate_help_text(&self) -> String {
        struct HelpText {
            first_part: String,
            middle_part: String,
            last_part: String
        }

        impl HelpText {
            fn new(first_part: String, middle_part: String, last_part: String) -> Self {
                Self { first_part, last_part, middle_part }
            }
        }

        let required_flags = self.args.clone().into_iter()
            .filter(|x| x.is_required() && !x.is_positional())
            .collect::<Vec<Arg>>();

        let positional_flags = self.args.clone().into_iter()
            .filter(|x| x.is_positional())
            .collect::<Vec<Arg>>();

        let text = format!("Usage: {} [OPTIONS]", self.get_name());
        let mut help_text = String::from(text);

        if required_flags.len() > 0 { help_text.push_str("...") }

        for flag in &required_flags {
            let name = if flag.get_long_name().is_empty() {
                flag.short_flag()
            } else { flag.long_flag() };
            help_text.push_str(format!(" [{}]", name).as_str());
        }

        // required positionals are shown plain, optional ones in brackets
        for flag in &positional_flags {
            let name = flag.get_name().to_uppercase();
            if flag.is_required() {
                help_text.push_str(&format!(" {name}"));
            } else {
                help_text.push_str(&format!(" [{name}]"));
            }
        }

        help_text.push_str(format!("\n{}\n", self.get_description()).as_str());
        if !self.get_description().is_empty() {
            help_text.push_str("\n");
        }

        if !&self.args.is_empty() {
            help_text.push_str("OPTIONS:\n");
            let mut help_text_list: Vec<HelpText> = vec![];
            let mut max_len_first = 0;
            let mut max_len_middle = 0;
            let mut max_len_last = 0;

            for arg in &self.args {
                if !arg.is_positional() {
                    let arg_help_text = arg.get_help_text();
                    let long_name = arg.long_flag();
                    let short_name = if arg.long_flag().is_empty() {
                        format!("{}", arg.short_flag())
                    } else {
                        if arg.short_flag().is_empty() {
                            format!("{}", arg.short_flag())
                        } else { format!("{},", arg.short_flag()) }
                    };

                    help_text_list.push(HelpText::new(short_name.clone(), long_name.clone(), arg_help_text.to_string()));

                    max_len_first = std::cmp::max(max_len_first, short_name.len());
                    max_len_middle = std::cmp::max(max_len_middle, long_name.len());
                    max_len_last = std::cmp::max(max_len_last, arg_help_text.len());
                }
            }

            for item in help_text_list {
                help_text.push_str(
                    format!("  {:<max_len_first$} {:<max_len_middle$}  {:<max_len_last$}\n",
                    item.first_part, item.middle_part, item.last_part).as_str()
                );
            }

            // POSITIONAL ARGUMENTS SECTION
            if !positional_flags.is_empty() {
                help_text.push_str("\nPOSITIONAL:\n");
                let width = positional_flags.iter().map(|a| a.get_name().len()).max().unwrap_or(0);

                for arg in &positional_flags {
                    help_text.push_str(&format!(
                        "  {:<width$}  {}\n",
                        arg.get_name().to_uppercase(),
                        arg.get_help_text()
                    ));
                }
            }
        }
        help_text
    }

    /// prints the help text to stdout and exits with status 0
    pub fn display_help_text(&self) {
        println!("{}", self.generate_help_text());
        std::process::exit(0);
    }
}




#[cfg(test)]
mod tests {
    use super::*;

    // turns &["a", "b"] into Vec<String>, so tests stay short
    fn args(list: &[&str]) -> Vec<String> {
        list.iter().map(|s| s.to_string()).collect()
    }

    fn parser() -> Parser {
        Parser::new("test")
            .arg(Arg::new("count").long("count").short("c").value_kind(ValueKind::Num)).unwrap()
            .arg(Arg::positional("file").value_kind(ValueKind::Str)).unwrap()
    }

    #[test]
    fn long_and_short_flags_give_the_same_result() {
        let long = parser().parse(&args(&["--count", "8"])).unwrap();
        let short = parser().parse(&args(&["-c", "8"])).unwrap();
        assert_eq!(long.get_num("count"), Some(8));
        assert_eq!(short.get_num("count"), Some(8));
    }

    #[test]
    fn positional_works_before_or_after_a_flag() {
        let before = parser().parse(&args(&["file.bin", "-c", "8"])).unwrap();
        let after = parser().parse(&args(&["-c", "8", "file.bin"])).unwrap();
        for m in [before, after] {
            assert_eq!(m.get_str("file"), Some("file.bin"));
            assert_eq!(m.get_num("count"), Some(8));
        }
    }

    #[test]
    fn optional_positional_can_be_missing() {
        let m = parser().parse(&args(&["-c", "8"])).unwrap();
        assert_eq!(m.get_str("file"), None);
    }

    #[test]
    fn too_many_positionals_is_an_error() {
        assert!(parser().parse(&args(&["a.bin", "b.bin"])).is_err());
    }

    #[test]
    fn unknown_flag_is_an_error() {
        assert!(parser().parse(&args(&["--nope"])).is_err());
    }

    #[test]
    fn bad_number_is_an_error() {
        assert!(parser().parse(&args(&["--count", "abc"])).is_err());
    }

    #[test]
    fn default_value_fills_in_when_flag_is_absent() {
        let p = Parser::new("test")
            .help_on_empty(false)
            .arg(Arg::new("cols").long("cols")
                .value_kind(ValueKind::Num)
                .default_value(Value::Num(16))).unwrap();
        let m = p.parse(&args(&[])).unwrap();
        assert_eq!(m.get_num("cols"), Some(16));
    }
}