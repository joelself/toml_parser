use std::fmt;
use std::fmt::Display;
use nom::IResult;
use ast::structs::{Toml};
use toml::toml;
pub struct Parser<'a> {
	root: Toml<'a>,
}

// TODO change this to return a parser result
impl<'a> Parser<'a> {
	pub fn new<'b>() -> Parser<'a> {
		Parser{ root: Toml{ exprs: vec![] } }
	}

	pub fn parse(&mut self, input: &'a str) {
		let r = toml(input);
		println!("{:?}", r);
		match r {
			IResult::Done(i, o) => self.root = o,
			_ => (),
		};
	}
}

impl<'a> Display for Parser<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", self.root)
	}
}