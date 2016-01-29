use std::fmt;
use std::fmt::Display;
use std::collections::HashMap;
use std::cell::{RefCell, Cell, RefMut};
use std::thread;
use nom::IResult;
use ast::structs::{Toml};
use types::HashValue;
use nom;
use std::any::Any;

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
	pub root: RefCell<Toml<'a>>,
	pub map: RefCell<HashMap<&'a str, HashValue<'a>>>,
	pub leftover: RefCell<&'a str>,
	pub line_count: Cell<u64>,
	pub last_table: RefCell<&'a str>,
	pub input: &'a str,
}

impl<'a> Default for Parser<'a> {
  fn default () -> Parser<'a> {
    Parser{
    	root: RefCell::new(Toml{exprs: vec![]}),
    	map: RefCell::new(HashMap::new()),
    	leftover: RefCell::new(""),
    	line_count: Cell::new(0u64),
    	last_table: RefCell::new(""),
    	input: "",
    }
  }
}

// TODO change this to return a parser result
impl<'a> Parser<'a> {
	pub fn new<'b>() -> Parser<'a> {
		Parser{ root: RefCell::new(Toml{ exprs: vec![] }), map: RefCell::new(HashMap::new()),
						leftover: RefCell::new(""), line_count: Cell::new(0),
						last_table: RefCell::new(""), input: ""}
	}

	pub fn parse(self: &'a mut Parser<'a>, input: &'a str) {
		self.input = input;
		self.toml();
		let mut res = self.result;
		match res {
			IResult::Done(i, o) => {
				*self.root.borrow_mut() = o;
				*self.leftover.borrow_mut() = i;
			},
			_ => (),
		};
	}
}

impl<'a> Display for Parser<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", *self.root.borrow())
	}
}