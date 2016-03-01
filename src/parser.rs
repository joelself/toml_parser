use std::fmt;
use std::fmt::Display;
use std::rc::Rc;
use std::cell::{RefCell, Cell};
use std::collections::{HashMap, BTreeMap};
use std::collections::hash_map::Entry;
use nom::IResult;
use ast::structs::{Toml, ArrayType, HashValue, TableType, Value, Array, InlineTable};
use types::{ParseError, ParseResult, TOMLValue, Str};

named!(full_line<&str, &str>, re_find!("^(.*?)(\n|(\r\n))"));
named!(all_lines<&str, Vec<&str> >, many0!(full_line));

pub fn count_lines(s: &str) -> u64 {
	let r = all_lines(s);
	match &r {
    &IResult::Done(_, ref o) 	=> o.len() as u64,
    _													=> 0 as u64,
	}
}

pub enum Key<'a> {
	Str(Str<'a>),
	Index(Cell<usize>),
}

impl<'a> Key<'a> {
	pub fn inc(&mut self) {
		if let &mut Key::Index(ref mut i) = self {
			i.set(i.get() + 1);
		}
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
	pub keychain: RefCell<Vec<Key<'a>>>,
	pub last_table: Option<Rc<TableType<'a>>>,
	pub last_array_type: RefCell<Vec<ArrayType>>,
	pub array_error: Cell<bool>,
	pub mixed_array: Cell<bool>,
	pub failure: Cell<bool>,
	pub string: String,
}

// TODO change this to return a parser result
impl<'a> Parser<'a> {
	pub fn new() -> Parser<'a> {
		let mut map = HashMap::new();
		map.insert("$TableRoot$".to_string(), HashValue::none_keys());
		Parser{ root: RefCell::new(Toml{ exprs: vec![] }), map: map,
						errors: RefCell::new(vec![]), leftover: "",
						line_count: Cell::new(0), last_array_tables: RefCell::new(vec![]),
						last_array_tables_index: RefCell::new(vec![]),
						last_table: None, last_array_type: RefCell::new(vec![]),
						keychain: RefCell::new(vec![]), 
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
		self
	}

	pub fn print_keys_and_values(self: &Parser<'a>) {
    let mut btree = BTreeMap::new();
		for (k, v) in self.map.iter() {
      btree.insert(k, v);
		}
    for (k, v) in btree.iter() {
      println!("key: {} - {}", k, v);
    }
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
				Some(to_tval!(&*val.borrow()))
			} else {
				None
			}
		} else {
			None
		}
	}


	pub fn set_value(self: &mut Parser<'a>, key: String, tval: TOMLValue<'a>) -> bool {
		let rf_map = RefCell::new(&mut self.map);
			let mut map_borrow = rf_map.borrow_mut();
			let val = match map_borrow.entry(key) {
				Entry::Occupied(entry) => entry.into_mut(),
				_ => return false,
			};
			let opt_value: &mut Option<Rc<RefCell<Value<'a>>>> = &mut val.value;
			let value_rf = match opt_value {
				&mut Some(ref mut v) => v,
				&mut None => return false,
			};
			let value = match tval {
				TOMLValue::Integer(ref v) 	=> Value::Integer(v.clone()),
				TOMLValue::Float(ref v)			=> Value::Float(v.clone()),
				TOMLValue::Boolean(v) 			=> Value::Boolean(v),
				TOMLValue::DateTime(v)			=> Value::DateTime(v.clone()),
				TOMLValue::Array(arr)				=> return Parser::reconstruct_array(value_rf, arr),
				TOMLValue::String(ref s, t)	=> Value::String(s.clone(), t),
				TOMLValue::InlineTable(it)	=> return Parser::reconstruct_inline_table(value_rf, it),
			};
			*value_rf.borrow_mut() = value;
			true
	}

	// TODO: BIG TODO: Need to rehash keys values when reconstituting inline tables and arrays if their keys
	//                 or their structure has changed.
	// 1. Array is replaced with scalar => remove keys for replaced array values
	// 2. Scalar is replaced with array => add new keys for new array values
	// 3. Array is truncated => remove keys for values that were removed
	// 4. Array is lengthened => add keys for new values that were added
	// 5. Inline table is replaced with a scalar => remove keys for replaced inline table key/values
	// 6. Scalar is replaced with inline table => add new keys for new inline table key/values
	// 7. Inline table is truncated => remove keys for values that were removed
	// 8. inline table is lengthened => add keys for new values that were added

	fn reconstruct_array(val_rf: &mut Rc<RefCell<Value<'a>>>,
		tval: Rc<Vec<TOMLValue<'a>>>) -> bool {
		match *val_rf.borrow_mut() {
			Value::Array(ref mut arr_rf) 	=> {
				let len = tval.len();
				if arr_rf.borrow().values.len() != len {
					return false; // TODO: implement default formatting for replacement arrays that are
												//       longer than the arrays they are replacing and simple array
												//       truncation for arrays that are shorter than what they are
												//       replacing
				}
				for i in 0..len {
					let value = match tval[i] {
						TOMLValue::Integer(ref v) 			=> Value::Integer(v.clone()),
						TOMLValue::Float(ref v)					=> Value::Float(v.clone()),
						TOMLValue::Boolean(v) 					=> Value::Boolean(v),
						TOMLValue::DateTime(ref v)			=> Value::DateTime(v.clone()),
						TOMLValue::Array(ref arr)				=> 
							return Parser::reconstruct_array(&mut arr_rf.borrow_mut().values[i].val, arr.clone()),
						TOMLValue::String(ref s, t)			=> Value::String(s.clone(), t),
						TOMLValue::InlineTable(ref it)	=>
							return Parser::reconstruct_inline_table(&mut arr_rf.borrow_mut().values[i].val, it.clone()),
					};
					let mut array_borrow = arr_rf.borrow_mut();
					let array_val_rc = &mut array_borrow.values[i].val;
					*array_val_rc.borrow_mut() = value;
				}
				return true;
			},
			_ => return false, // TODO: implement Parser::construct_array with some default formatting
		}
	}

	fn sanitize_array(arr: Rc<RefCell<Array<'a>>>) -> TOMLValue<'a> {
		let mut result: Vec<TOMLValue> = vec![];
		for av in arr.borrow().values.iter() {
			result.push(to_tval!(&*av.val.borrow()));
		}
		TOMLValue::Array(Rc::new(result))
	}

	// TODO: Implement reconstruct_inline_table, remembering to rehash values
	fn reconstruct_inline_table(_val_rf: &mut Rc<RefCell<Value<'a>>>,
		_tit: Rc<Vec<(Str, TOMLValue<'a>)>>) -> bool {
		// match *val_rf.borrow_mut() {
		// 	Value::InlineTable(ref mut it_rf) 	=> {
		// 		let len = tit.len();
		// 		if it_rf.borrow().keyvals.len() != len {
		// 			return false; // TODO: implement default formatting for replacement tables that are
		// 										//       longer than the tables they are replacing and simple table
		// 										//       truncation for tables that are shorter than what they are
		// 										//       replacing
		// 		}
		// 		for i in 0..len {
		// 			let (key, value) = match tval[i] {
		// 				(k, TOMLValue::Integer(ref v)) 			=> (k, Value::Integer(v.clone())),
		// 				(k, TOMLValue::Float(ref v))				=> (k, Value::Float(v.clone())),
		// 				(k, TOMLValue::Boolean(v)) 					=> (k, Value::Boolean(v)),
		// 				(k, TOMLValue::DateTime(ref v))			=> (k, Value::DateTime(v.clone())),
		// 				(k, TOMLValue::Array(ref arr))			=> 
		// 					return Parser::reconstruct_array(&mut arr_rf.borrow_mut().values[i].val, arr.clone()),
		// 				(k, TOMLValue::String(ref s, t))		=> Value::String(s.clone(), t),
		// 				(k, TOMLValue::InlineTable(ref it))	=>
		// 					return Parser::reconstruct_inline_table(&mut arr_rf.borrow_mut().values[i].val, it.clone()),
		// 			};
		// 			let mut it_borrow = it_rf.borrow_mut();
		// 			let mut array_val_rc = &mut it_borrow.keyvalues[i].keyval;
		// 			*array_val_rc.borrow_mut() = value;
		// 		}
		// 		return true;
		// 	},
		// 	_ => return false, // TODO: implement Parser::construct_table with some default formatting
		// }
		return false;
	}
	
	fn sanitize_inline_table(it: Rc<RefCell<InlineTable<'a>>>) -> TOMLValue<'a> {
		let mut result: Vec<(Str<'a>, TOMLValue)> = vec![];
		for kv in it.borrow().keyvals.iter() {
			result.push((kv.keyval.key.clone(), to_tval!(&*kv.keyval.val.borrow())));
		}
		return TOMLValue::InlineTable(Rc::new(result));
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