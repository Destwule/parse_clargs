use crate::{
    value::{Value, ValueKind},
    arg::Arg,
    matches::Matches
};

pub struct Parser {
    name: String,
    description: String,
    args: Vec<Arg>,
    args_index: usize,
    help_on_empty: bool
}

impl Parser {
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

    pub fn get_name(&self) -> &str { &self.name }
    pub fn get_description(&self) -> &str { &self.description }
    pub fn get_args(&self) -> &Vec<Arg> { &self.args }
    pub fn get_help_on_empty(&self) -> bool { self.help_on_empty }
    
    pub fn add(mut self, arg: Arg) -> Result<Self, String> {
        let registered = arg.register()?;
        self.args.insert(self.args_index, registered);
        self.args_index += 1;

        Ok(self)
    }

    pub fn description(mut self, description: &str) -> Self {
        self.description = description.to_string();

        self
    }

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


    pub fn parse(&self, cli_args: &[String]) -> Result<Matches, String> {
        let mut matches = Matches::new();
        let mut index = 0;
        
        if cli_args.iter().find(|a| *a == "--help").is_some() || (self.get_help_on_empty() && cli_args.is_empty()) {
            self.display_help_text();
        }

        let required_flags = self.get_args().iter().filter(|x| x.is_required() && !x.is_positional()).collect::<Vec<&Arg>>();
        let positional_args = self.get_args().iter().filter(|x| x.is_positional()).collect::<Vec<&Arg>>();
        let mut next_positional = 0usize;

        while index < cli_args.len() {
            let peek = index + 1;
            let after_peek = index + 2;
            let arg_name = &cli_args[index];

            let found = self.args.iter().find(|a| {
                !a.is_positional() && (a.long_flag() == *arg_name || a.short_flag() == *arg_name)
            });
            
            if let Some(arg) = found {
                matches.add(
                    arg.get_name(),
                    Self::value_of_arg(&arg, &cli_args, arg_name, index)?
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
                // for positional arguments: comeback later and try to understand it
                let def = positional_args
                    .get(next_positional)
                    .ok_or_else(|| format!("too many arguments: '{arg_name}'"))?;
                let key = def.get_long_name();
                matches.add(def.get_name(), Self::parse_value(def, arg_name, key)?);
                next_positional += 1;
                index = peek;
            }
        }

        if required_flags.len() != 0 {
            for flag in &required_flags {
                let name = if flag.get_long_name().is_empty() {
                    flag.get_short_name()
                } else { flag.get_long_name() }.as_str();
                
                if !matches.is_present(name) {
                    eprintln!("{}: missing {} operand", self.get_name(), name);
                    eprintln!("Try '{} --help' for more information", self.get_name());
                    std::process::exit(1);
                }
            }
        }

        // for positional arguments: comeback later and try to understand it
        if self.get_help_on_empty() {
            for def in &positional_args[next_positional..] {
                if def.get_default_value().is_none() {
                    eprintln!("{}: missing {} operand", self.get_name(), def.get_long_name().to_uppercase());
                    eprintln!("Try '{} --help' for more information", self.get_name());
                    std::process::exit(1);
                }
            }
        }
                
        // Fills in the Default values for those who were not specified but were not given a value
        for arg in self.get_args() {
            if let Some(default) = arg.get_default_value() {
                let checking_value = arg.get_name();

                if !matches.is_present(checking_value) {
                    matches.add(arg.get_name(), default.clone());
                }
            }
            
        }
        

        Ok(matches)
    }

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

        let required_flags = self.args.clone().into_iter().filter(|x| x.is_required() && !x.is_positional()).collect::<Vec<Arg>>();
        
        let positional_flags = self.args.clone().into_iter().filter(|x| x.is_positional()).collect::<Vec<Arg>>();
        
        let text = format!("Usage: {} [OPTIONS]", self.get_name());
        let mut help_text = String::from(text);

        if required_flags.len() > 0 { help_text.push_str("...") }
        
        for flag in &required_flags {
            let name = if flag.get_long_name().is_empty() {
                flag.short_flag()
            } else { flag.long_flag() };
            help_text.push_str(format!(" [{}]", name).as_str());
        }

        for flag in positional_flags {
            let name = flag.get_long_name();
            
            help_text.push_str(format!(" {}", name.to_uppercase()).as_str());
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
            
            // Always remember to clear 'text' variable at the end of each loop iteration
            let mut text = String::new();
            
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
                
                text.clear();
            }

            for item in help_text_list {
                help_text.push_str(
                    format!("  {:<max_len_first$} {:<max_len_middle$}  {:<max_len_last$}\n",
                    item.first_part, item.middle_part, item.last_part).as_str()
                );
            }
        }
        help_text
    }

    pub fn display_help_text(&self) {
        println!("{}", self.generate_help_text());
        std::process::exit(0);
    }
}