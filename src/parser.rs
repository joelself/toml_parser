use std::fmt;
use std::fmt::Display;
use std::collections::HashMap;
use std::cell::{RefCell, Cell, RefMut};
use std::thread;
use nom::IResult;
use ast::structs::{Toml};
use types::HashValue;

named!(full_line<&str, &str>, re_find!("^(.*?)(\n|(\r\n))"));
named!(all_lines<&str, Vec<&str> >, many0!(full_line));

pub fn count_lines(s: &str) -> u64 {
	let r = all_lines(s);
	match &r {
    &IResult::Done(_, ref o) 	=> o.len() as u64,
    _													=> 0 as u64,
	}
}

pub struct ParseData<'a> {
	pub root: RefCell<Toml<'a>>,
	pub map: RefCell<HashMap<&'a str, HashValue<'a>>>,
	pub leftover: RefCell<&'a str>,
	pub line_count: Cell<u64>,
	pub last_table: RefCell<&'a str>,
}

impl<'a> Default for ParseData<'a> {
  fn default () -> ParseData<'a> {
    ParseData{
    	root: RefCell::new(Toml{exprs: vec![]}),
    	map: RefCell::new(HashMap::new()),
    	leftover: RefCell::new(""),
    	line_count: Cell::new(0u64),
    	last_table: RefCell::new(""),
    }
  }
}

pub struct Parser {
	pub foo: u64,
}

// TODO change this to return a parser result
impl<'a> Parser {
	pub fn parse(input: &'a str, data: &mut ParseData<'a>) {
		let mut res = Parser::toml(input, data);
		match res {
			IResult::Done(i, o) => {
				*data.root.borrow_mut() = o;
				*data.leftover.borrow_mut() = i;
			},
			_ => (),
		};
	}
}

impl<'a> Display for ParseData<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", *self.root.borrow())
	}
}