//! A small command-line argument parser with typed values and auto-generated help text.
//!
//! # Example
//!
//! ```
//! use parse_clargs::{
//!     Parser, Arg, Value, ValueKind
//! };
//!
//! fn main() -> Result<(), String> {
//!     let cli_args = vec!["--name".to_string(), "Ada".to_string()];
//!     let parser = Parser::new("greeter")
//!         .description("Prints a greeting")
//!         .arg(Arg::new("name")
//!             .long("name")
//!             .help("who to greet")
//!             .value_kind(ValueKind::Str))?;
//!
//!     let matches = parser.parse(&cli_args)?;
//!     assert_eq!(matches.get_str("name"), Some("Ada"));
//!     Ok(())
//!  }
//! ```

mod value;
mod arg;
mod parser;
mod matches;

pub use value::{Value, ValueKind};
pub use arg::Arg;
pub use parser::Parser;
pub use matches::Matches;