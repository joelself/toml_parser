use std::fmt;
use std::fmt::Display;
use std::rc::Rc;
use std::cell::{RefCell, Cell};
use std::collections::HashMap;
use nomplusplus::IResult;
use ast::structs::{Toml, ArrayType, HashValue, TableType};
use types::{ParseError, ParseResult};

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
	pub map: HashMap<String, HashValue<'a>>,
	pub errors: RefCell<Vec<ParseError<'a>>>,
	pub leftover: &'a str,
	pub line_count: Cell<u64>,
	pub last_array_tables: RefCell<Vec<Rc<TableType<'a>>>>,
	pub last_table: Option<Rc<TableType<'a>>>,
	pub last_array_type: RefCell<Vec<ArrayType>>,
	pub array_error: Cell<bool>,
	pub mixed_array: Cell<bool>,
	pub failure: Cell<bool>,
}

impl<'a> Default for Parser<'a> {
  fn default () -> Parser<'a> {
    Parser{
    	root: RefCell::new(Toml{exprs: vec![]}),
    	map: HashMap::new(),
    	errors: RefCell::new(vec![]),
    	leftover: "",
    	line_count: Cell::new(0u64),
    	last_array_tables: RefCell::new(vec![]),
    	last_table: None,
    	last_array_type: RefCell::new(vec![]),
    	array_error: Cell::new(false),
    	mixed_array: Cell::new(false),
    	failure: Cell::new(false),
    }
  }
}

// TODO change this to return a parser result
impl<'a> Parser<'a> {
	pub fn new<'b>() -> Parser<'a> {
		Parser{ root: RefCell::new(Toml{ exprs: vec![] }), map: HashMap::new(),
						errors: RefCell::new(vec![]), leftover: "",
						line_count: Cell::new(0), last_array_tables: RefCell::new(vec![]),
						last_table: None,
						last_array_type: RefCell::new(vec![]), array_error: Cell::new(false),
						mixed_array: Cell::new(false), failure: Cell::new(false)}
	}

	pub fn parse(mut self: Parser<'a>, input: &'a str) -> Parser<'a> {
		let (tmp, res) = self.toml(input);
		self = tmp;
		//let mut res = self.result;
		match res {
			IResult::Done(i, o) => {
				*self.root.borrow_mut() = o;
				self.leftover = i;
			},
			_ => self.failure.set(true),
		};
		self
	}

	pub fn get_result(self: &Parser<'a>) -> ParseResult<'a> {
		if self.failure.get() == true {
			return ParseResult::Failure(0, 0);
		}
		if self.leftover.len() > 0 {
			if self.errors.borrow().len() > 0 {
				return ParseResult::PartialError(self.leftover, self.get_errors());
			} else {
				return ParseResult::Partial(self.leftover)
			}
		} else {
			if self.errors.borrow().len() > 0 {
				return ParseResult::FullError(self.get_errors());
			} else {
				return ParseResult::Full;
			}
		}
	}

	fn get_errors(self: &Parser<'a>) -> Vec<ParseError<'a>> {
		unimplemented!{}
	}
}

impl<'a> Display for Parser<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", *self.root.borrow())
	}
}