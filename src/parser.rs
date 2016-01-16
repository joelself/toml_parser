use std::fmt;
use std::fmt::Display;
use std::collections::HashMap;
use std::cell::RefCell;
use std::thread;
use nom::IResult;
use ast::structs::{Toml};
use toml::toml;
use types::HashValue;

thread_local!(pub static LINE_COUNT: RefCell<u64> = RefCell::new(0));
thread_local!(pub static KEY_VALUE_MAP: RefCell<HashMap<String, HashValue<'static>>> = 
	RefCell::new(HashMap::new()));
thread_local!(pub static LAST_TABLE: RefCell<&'a str> = RefCell::new(""));

named!(full_line<&str, &str>, re_find!("^(.*?)(\n|(\r\n))"));
named!(all_lines<&str, Vec<&str> >, many0!(full_line));

pub fn count_lines(s: &str) -> u64 {
	let r = all_lines(s);
	match &r {
    &IResult::Done(_, ref o) 	=> o.len() as u64,
    _													=> 0 as u64,
	}
}

pub struct Parser<'a> {
	root: Toml<'a>,
	map: HashMap<&'a str, HashValue<'a>>,
	leftover: &'a str,
	line_count: u64,
	last_table: &' str,
}

impl<'a> Default for Parser<'a> {
  fn default () -> Parser<'a> {
    Parser{
    	root: Toml{exprs: vec![]},
    	map: HashMap::new(),
    	leftover: "",
    	line_count: 0,
    	last_table: "",
    }
  }
}

// TODO change this to return a parser result
impl<'a> Parser<'a> {
	pub fn new<'b>() -> Parser<'a> {
		Parser{ root: Toml{ exprs: vec![] }, map: HashMap::new() }
	}

	pub fn parse(&mut self, input: &'a str) {
		let r = self.toml(input);
		match r {
			IResult::Done(i, o) => {
				self.root = o;
				self.leftover = i;
			},
			_ => (),
		};
	}
}

impl<'a> Display for Parser<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", self.root)
	}
}