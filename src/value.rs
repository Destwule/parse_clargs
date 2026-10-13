/// A value an argument can hold: text, a whole number, or true/false.
///
/// This is what `Matches` stores for each argument. Most of the time you won't build
/// one yourself and will use the typed getters like `Matches::get_num`. You need
/// `Value` when giving an argument a default value, or when matching on the result
/// of `Matches::get`.
///
/// `Num` holds a `u64`, so negative numbers aren't supported.
///
/// # Example
/// ```
/// # use parse_clargs::{Arg, Parser, Value, ValueKind};
/// // as a default value
/// let arg = Arg::new("cols")
///     .long("cols")
///     .value_kind(ValueKind::Num)
///     .default_value(Value::Num(16));
///
/// // as the result of `get`
/// let parser = Parser::new("hexview")
///     .help_on_empty(false)
///     .arg(arg)?;
/// let matches = parser.parse(&["--cols".to_string(), "8".to_string()])?;
/// assert_eq!(matches.get("cols"), Some(&Value::Num(8)));
/// # Ok::<(), String>(())
/// ```
#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Str(String),
    Num(u64),
    Bool(bool),
}

/// The type of value an argument expects. The parser uses it to convert what the
/// user typed.
///
/// - `Str`: text, kept as typed.
/// - `Num`: a whole number from 0 up. Anything else, such as `abc`, is an error.
/// - `Bool`: `true` or `false`. A `Bool` argument takes no value: writing the flag
///   alone means `true`.
/// - `UNSPECIFIED`: the default for a new argument. It isn't a usable kind, and
///   `Parser::arg` rejects any argument that still has it, so always set a kind
///   with `Arg::value_kind`.
///
/// # Example
/// ```
/// # use parse_clargs::{Arg, Parser, ValueKind};
/// let arg = Arg::new("name").long("name").value_kind(ValueKind::Str);
/// assert!(Parser::new("greeter").arg(arg).is_ok());
///
/// // forgetting the value kind is an error
/// let missing = Arg::new("name").long("name");
/// assert!(Parser::new("greeter").arg(missing).is_err());
/// ```
#[derive(PartialEq, Clone, Debug)]
pub enum ValueKind {
    Str,
    Num,
    Bool,
    UNSPECIFIED,
}
