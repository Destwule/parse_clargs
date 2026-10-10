//! A small command-line argument parser with typed values and auto-generated help text.
//!
//! fn main() -> Result<(), String> {
//!     let cli_args: Vec<String> = std::env::args().skip(1).collect();
//!     let parser = Parser::new("My program")
//!         .description("My description")
//!         .help_on_empty(true)
//!         .arg
//!
//!
//!
//!
//!

mod value;
mod arg;
mod parser;
mod matches;

pub use value::{Value, ValueKind};
pub use arg::Arg;
pub use parser::Parser;
pub use matches::Matches;