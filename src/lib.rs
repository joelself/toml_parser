#[macro_use]
extern crate nom;
extern crate regex;
#[macro_use]extern crate log;
#[macro_use]
mod macros;
mod ast;
mod toml;
mod util;
mod objects;
mod primitives;
pub mod types;
pub mod parser;