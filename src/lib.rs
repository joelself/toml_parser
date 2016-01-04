#![feature(plugin)]
#![plugin(regex_macros)]
#[macro_use]
extern crate nom;
extern crate regex;
pub mod ast;
mod toml;
mod util;
mod objects;
mod primitives;
#[test]
fn it_works() {
	println!("{}", ast::structs::PartialTime{hour: "09", minute: "30", second: "22", fraction: "733"});
}
