use std::fmt;
use std::fmt::Display;
use std::rc::Rc;
use std::cell::{RefCell, Cell};
use std::collections::{HashMap, BTreeMap};
use std::collections::hash_map::Entry;
use nom::IResult;
use ast::structs::{Toml, ArrayType, HashValue, TableType, Value, Array, InlineTable};
use types::{ParseError, ParseResult, TOMLValue, Str, Children};

named!(full_line<&str, &str>, re_find!("^(.*?)(\n|(\r\n))"));
named!(all_lines<&str, Vec<&str> >, many0!(full_line));

pub fn count_lines(s: &str) -> u64 {
	let r = all_lines(s);
	match &r {
    &IResult::Done(_, ref o) => o.len() as u64,
    _						 => 0 as u64,
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
						failure: Cell::new(false)}
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

	pub fn get_value(self: &Parser<'a>, key: String) -> Option<TOMLValue<'a>> {
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

  pub fn get_children(self: &Parser<'a>, key: String) -> Option<&Children> {
    if self.map.contains_key(&key) {
      let hashval = self.map.get(&key).unwrap();
      return Some(&hashval.subkeys);
    } else {
      None
    }
  }

  // TODO: ********* Need to figure out a way to borrow self or self.map and use it multiple
  //       times.
  // Move the borrow to the bottom where it is used, make reconstruct_vector a method again
  pub fn set_value(self: &mut Parser<'a>, key: String, tval: TOMLValue<'a>) -> bool {
    let self_rc = RefCell::new(self);
    let mut borrow = self_rc.borrow_mut();
    let val = match borrow.map.entry(key.clone()) {
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
      TOMLValue::Array(_)				=> return Parser::reconstruct_vector(&self_rc, key, value_rf, &tval),
      TOMLValue::String(ref s, t)	=> Value::String(s.clone(), t),
      TOMLValue::InlineTable(_)	=> return Parser::reconstruct_vector(&self_rc, key, value_rf, &tval),
    };
    *value_rf.borrow_mut() = value;
    true
  }

	// TODO: BIG TODO: Need to rehash keys values when reconstituting inline tables and arrays if their keys
	//                 or their structure has changed.
  // This is is a simplified version of my previous plan:
  // 1. Check the structure of the inline table/array
  //   1. If the structure is the same, just go through it and replace values
  //   2. If the structure is different, wipe out the whole array/inline table from the map
  //      and then rebuild it, using default whitespace scheme
  
  fn same_structure(val_rf: &Rc<RefCell<Value<'a>>>, tval: &TOMLValue<'a>) -> bool {
    match (&*val_rf.borrow(), tval) {
      (&Value::Array(ref arr), &TOMLValue::Array(ref t_arr)) => {
        let borrow = arr.borrow();
        if borrow.values.len() != t_arr.len() {
          return false;
        }
        let len = borrow.values.len();
        for i in 0..len {
          if !Parser::same_structure(&borrow.values[i].val, &t_arr[i]) {
            return false;
          }
        }
        return true;
      },
      (&Value::InlineTable(ref it), &TOMLValue::InlineTable(ref t_it)) => {
        let borrow = it.borrow();
        if borrow.keyvals.len() != t_it.len() {
          return false;
        }
        let len = borrow.keyvals.len();
        for i in 0..len {
          if borrow.keyvals[i].keyval.key != t_it[i].0 ||
            !Parser::same_structure(&borrow.keyvals[i].keyval.val, &t_it[i].1) {
            return false;
          }
        }
        return true;
      },
      (&Value::Array(_), _)           => return false, // Array replaced with scalar
      (&Value::InlineTable(_), _)     => return false, // InlineTable replaced with scalar
      (_, &TOMLValue::Array(_))       => return false, // scalar replaced with an Array
      (_, &TOMLValue::InlineTable(_)) => return false, // scalar replaced with an InlineTable
      (_,_)                           => return true,  // scalar replaced with any other scalar
    }
  }

	fn reconstruct_vector(self_rc: &RefCell<&mut Parser<'a>>, key: String,
    val_rf: &mut Rc<RefCell<Value<'a>>>, tval: &TOMLValue<'a>) -> bool {
    if Parser::same_structure(val_rf, tval) {
      Parser::replace_values(val_rf, tval);
      true
    } else {
      Parser::wipe_out_key(self_rc, key);
      true
		}
	}
  
  fn wipe_out_key(self_rc: &RefCell<&mut Parser<'a>>, key: String) {
    let borrow = self_rc.borrow();
    let hv_opt = borrow.map.get(&key);
    if let Some(hv) = hv_opt {
      let children = &hv.subkeys;
      match children {
        &Children::Count(ref cell) => {
          for i in 0..cell.get() {
            let subkey = format!("{}[{}]", key, i);
            Parser::wipe_out_key(self_rc, subkey.clone());
            self_rc.borrow_mut().map.remove(&subkey);
          }
        },
        &Children::Keys(ref rc_hs) => {
          for childkey in rc_hs.borrow().iter(){
            let subkey = format!("{}.{}", key, childkey);
            Parser::wipe_out_key(self_rc, subkey.clone());
            self_rc.borrow_mut().map.remove(&subkey);
          }
        },
      }
    }
  }
  
  fn replace_values(val_rf: &Rc<RefCell<Value<'a>>>, tval: &TOMLValue<'a>) {
    let value = match (&*val_rf.borrow(), tval) {
      (&Value::Array(ref arr), &TOMLValue::Array(ref t_arr)) => {
        let borrow = arr.borrow();
        let len = borrow.values.len();
        for i in 0..len {
          Parser::replace_values(&borrow.values[i].val, &t_arr[i]);
        }
        return;
      },
      (&Value::InlineTable(ref it), &TOMLValue::InlineTable(ref t_it)) => {
        let borrow = it.borrow();
        let len = borrow.keyvals.len();
        for i in 0..len {
          Parser::replace_values(&borrow.keyvals[i].keyval.val, &t_it[i].1);
        }
        return;
      },
      (_, &TOMLValue::Integer(ref v)) 	=> Value::Integer(v.clone()),
      (_, &TOMLValue::Float(ref v))			=> Value::Float(v.clone()),
      (_, &TOMLValue::Boolean(v)) 			=> Value::Boolean(v),
      (_, &TOMLValue::DateTime(ref v))	=> Value::DateTime(v.clone()),
      (_, &TOMLValue::String(ref s, t))	=> Value::String(s.clone(), t),
      (_,_)                             => panic!("This code should be unreachable."),
    };
    *val_rf.borrow_mut() = value;
  }

	fn sanitize_array(arr: Rc<RefCell<Array<'a>>>) -> TOMLValue<'a> {
		let mut result: Vec<TOMLValue> = vec![];
		for av in arr.borrow().values.iter() {
			result.push(to_tval!(&*av.val.borrow()));
		}
		TOMLValue::Array(Rc::new(result))
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