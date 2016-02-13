use std::fmt;
use std::fmt::Display;
use std::rc::Rc;
use std::cell::{RefCell, Cell};
use std::collections::HashMap;
use nomplusplus::IResult;
use ast::structs::{Toml, ArrayType, HashValue, TableType, Value, Array};
use types::{ParseError, ParseResult, TOMLValue, Str};
use std::collections::hash_map::Entry;

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
	pub last_array_tables_index: RefCell<Vec<usize>>,
	pub last_table: Option<Rc<TableType<'a>>>,
	pub last_array_type: RefCell<Vec<ArrayType>>,
	pub last_key: &'a str,
	pub array_error: Cell<bool>,
	pub mixed_array: Cell<bool>,
	pub failure: Cell<bool>,
	pub string: String,
}

// TODO change this to return a parser result
impl<'a> Parser<'a> {
	pub fn new() -> Parser<'a> {
		Parser{ root: RefCell::new(Toml{ exprs: vec![] }), map: HashMap::new(),
						errors: RefCell::new(vec![]), leftover: "",
						line_count: Cell::new(0), last_array_tables: RefCell::new(vec![]),
						last_array_tables_index: RefCell::new(vec![]),
						last_table: None, last_array_type: RefCell::new(vec![]),
						last_key: "", 
						array_error: Cell::new(false), mixed_array: Cell::new(false),
						failure: Cell::new(false), string: String::new()}
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
		// BEGIN DEBUG STATEMENTS
		for (k, v) in self.map.iter() {
			println!("key: {} : value: {}", k, v);
		}
		// END DEBUG STATEMENTS
		self
	}

	pub fn get_result(self: &Parser<'a>) -> ParseResult<'a> {
		if self.failure.get() == true {
			return ParseResult::Failure(0, 0);
		}
		if self.leftover.len() > 0 {
			if self.errors.borrow().len() > 0 {
				return ParseResult::PartialError(Str::Str(self.leftover), self.get_errors());
			} else {
				return ParseResult::Partial(Str::Str(self.leftover))
			}
		} else {
			if self.errors.borrow().len() > 0 {
				return ParseResult::FullError(self.get_errors());
			} else {
				return ParseResult::Full;
			}
		}
	}

	pub fn get_value(self: &mut Parser<'a>, key: String) -> Option<TOMLValue<'a>> {
		if self.map.contains_key(&key) {
			let hashval = self.map.get(&key).unwrap();
			let clone = hashval.clone();
			if let Some(val) = clone.value {
				match &*val {
					&Value::Integer(ref v) => Some(TOMLValue::Integer(v.clone())),
					&Value::Float(ref v) => Some(TOMLValue::Float(v.clone())),
					&Value::Boolean(v) => Some(TOMLValue::Boolean(v)),
					&Value::DateTime(ref v) => Some(TOMLValue::DateTime(v.clone())),
					&Value::Array(ref arr) => Some(Parser::sanitize_array(arr.clone())),
					&Value::String(ref s, t) => Some(TOMLValue::String(s.clone(), t.clone())),
				}
			} else {
				None
			}
		} else {
			None
		}
	}

	pub fn set_value(self: &mut Parser<'a>, key: String, val: TOMLValue<'a>) -> bool {
		if self.map.contains_key(&key) {
			let mut entry = self.map.entry(key);
			if let Entry::Occupied(mut o) = entry {
				let map_val = match val {
					TOMLValue::Integer(ref v) 		=> Value::Integer(v.clone()),
					TOMLValue::Float(ref v)				=> Value::Float(v.clone()),
					TOMLValue::Boolean(v) 		=> Value::Boolean(v),
					TOMLValue::DateTime(v)		=> Value::DateTime(v.clone()),
					TOMLValue::Array(arr)			=> Parser::reconstruct_array(arr),
					TOMLValue::String(ref s, t)		=> Value::String(s.clone(), t),
				};
				o.insert(HashValue::new(Rc::new(map_val)));
				return true;
			}
		}
		false
	}

	fn reconstruct_array(arr: Rc<Vec<TOMLValue<'a>>>) -> Value<'a> {
		// TODO: Implement this
		return Value::Integer(Str::Str("1"));
	}

	fn sanitize_array(arr: Rc<Array<'a>>) -> TOMLValue<'a> {
		let mut result: Vec<TOMLValue> = vec![];
		for av in arr.values.iter() {
			match *av.val {
				Value::Integer(ref v) => result.push(TOMLValue::Integer(v.clone())),
				Value::Float(ref v) => result.push(TOMLValue::Float(v.clone())),
				Value::Boolean(v) => result.push(TOMLValue::Boolean(v)),
				Value::DateTime(ref v) => result.push(TOMLValue::DateTime(v.clone())),
				Value::Array(ref arr) => result.push(Parser::sanitize_array(arr.clone())),
				Value::String(ref s, t) => result.push(TOMLValue::String(s.clone(), t.clone())),
			}
		}
		TOMLValue::Array(Rc::new(result))
	}

	pub fn get_errors(self: &Parser<'a>) -> Vec<ParseError<'a>> {
		unimplemented!{}
	}
}

impl<'a> Display for Parser<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", *self.root.borrow())
	}
}