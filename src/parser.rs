use std::fmt;
use std::fmt::Display;
use std::rc::Rc;
use std::cell::{RefCell, Cell};
use std::collections::{HashMap, BTreeMap};
use std::collections::hash_map::Entry;
use std::borrow::Cow;
use nom::IResult;
use ast::structs::{Toml, ArrayType, HashValue, TableType, Value, Array, InlineTable,
                   ArrayValue, WSSep, TableKeyVal};
use types::{ParseError, ParseResult, TOMLValue, Children};

named!(full_line<&str, &str>, re_find!("^(.*?)(\n|(\r\n))"));
named!(all_lines<&str, Vec<&str> >, many0!(full_line));

pub fn count_lines(s: &str) -> usize {
	let r = all_lines(s);
	match &r {
    &IResult::Done(_, ref o) => o.len() as usize,
    _						 => 0 as usize,
	}
}

pub enum Key<'a> {
	Str(Cow<'a, str>),
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
	pub line_count: Cell<usize>,
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
		map.insert("$Root$".to_string(), HashValue::none_keys());
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

	pub fn print_keys_and_values_debug(self: &Parser<'a>) {
    let mut btree = BTreeMap::new();
		for (k, v) in self.map.iter() {
      btree.insert(k, v);
		}
    for (k, v) in btree.iter() {
      debug!("key: {} - {}", k, v);
    }
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

	pub fn get_result(self: &'a Parser<'a>) -> ParseResult<'a> {
		if self.failure.get() == true {
			return ParseResult::Failure(self.line_count.get(), 0);
		}
		if self.leftover.len() > 0 {
			if self.errors.borrow().len() > 0 {
				return ParseResult::PartialError(self.leftover.into(), &self.errors);
			} else {
				return ParseResult::Partial(self.leftover.into())
			}
		} else {
			if self.errors.borrow().len() > 0 {
				return ParseResult::FullError(&self.errors);
			} else {
				return ParseResult::Full;
			}
		}
	}

	pub fn get_value<S>(self: &Parser<'a>, key: S) -> Option<TOMLValue<'a>> where S: Into<String> {
		let s_key = key.into();
    if self.map.contains_key(&s_key) {
			let hashval = self.map.get(&s_key).unwrap();
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
    let k;
    if key == "" {
      k = "$Root$".to_string();
    } else {
      k = key;
    }
    if self.map.contains_key(&k) {
      let hashval = self.map.get(&k).unwrap();
      return Some(&hashval.subkeys);
    } else {
      None
    }
  }
  
  pub fn set_value<S>(self: &mut Parser<'a>, key: S, tval: TOMLValue<'a>) -> bool where S: Into<String> {
    let s_key = key.into();
    {
      let val = match self.map.entry(s_key.clone()) {
        Entry::Occupied(entry) => entry.into_mut(),
        _ => return false,
      };
      let opt_value: &mut Option<Rc<RefCell<Value<'a>>>> = &mut val.value;
      let val_rf = match opt_value {
        &mut Some(ref mut v) => v,
        &mut None => return false,
      };
      // if the inline table/array has the same structure the just replace the values
      if Parser::same_structure(val_rf, &tval) {
        return Parser::replace_values(val_rf, &tval);
      }
    }
    // if the inline table/array has a different structure, delete the existing
    // array/inline table from the map and rebuild it from the new value
    let all_keys = self.get_all_subkeys(&s_key);
    for key in all_keys.iter() {
      self.map.remove(key);
    }
    let new_value_opt = Parser::convert_vector(&tval);
    if new_value_opt.is_none() {
      return false;
    }
    let new_value = new_value_opt.unwrap();
    let new_value_clone = match new_value {
      Value::Array(ref rc_rc) => {
        Value::Array(rc_rc.clone())
      },
      Value::InlineTable(ref rc_rc) => {
        Value::InlineTable(rc_rc.clone())
      },
      unknown => panic!("In Parser.set_value, new_value should only be an Array or InlineTable. Instead it's a {:?}", unknown),
    };
    if self.map.contains_key(&s_key) {
      let existing_value = match self.map.entry(s_key.clone()) {
        Entry::Occupied(entry) => entry.into_mut(),
        _ => panic!("Map contains key, but map.entry returned a Vacant entry is set_value."),
      };
      let opt_value: &mut Option<Rc<RefCell<Value<'a>>>> = &mut existing_value.value;
      let val_rf = match opt_value {
        &mut Some(ref mut v) => v,
        &mut None => panic!("existing_value's value is None, when this should've returned false earlier in the function."),
      };
      *val_rf.borrow_mut() = new_value;
    }
    let new_value_rc = Rc::new(RefCell::new(new_value_clone));
    self.rebuild_vector(s_key.clone(), new_value_rc.clone(), true);
    true
  }
  
  fn convert_vector(tval: &TOMLValue<'a>) -> Option<Value<'a>> {
    if !tval.validate() {
      return None;
    }
     match tval {
      &TOMLValue::Array(ref arr) => {
        let mut values = vec![];
        for i in 0..arr.len() {
          let subval = &arr[i];
          let value_opt = Parser::convert_vector(subval);
          if value_opt.is_none() {
            return None;
          }
          let value = value_opt.unwrap();
          let array_value;
          if i < arr.len() - 1 {
            array_value = ArrayValue::default(Rc::new(RefCell::new(value)));
          } else {
            array_value = ArrayValue::last(Rc::new(RefCell::new(value)));
          }
          values.push(array_value);
        }
        return Some(Value::Array(Rc::new(RefCell::new(
          Array::new(values, vec![], vec![])
        ))));
      },
      &TOMLValue::InlineTable(ref it) => {
        let mut key_values = vec![];
        for i in 0..it.len() {
          let subval = &it[i].1;
          let value_opt = Parser::convert_vector(subval);
          let value = value_opt.unwrap();
          let key_value;
          if i < it.len() - 1 {
            key_value = TableKeyVal::default(it[i].0.clone().into_owned(), Rc::new(RefCell::new(value)));
          } else {
            key_value = TableKeyVal::last(it[i].0.clone().into_owned(), Rc::new(RefCell::new(value)));
          }
          key_values.push(key_value);
        }
        return Some(Value::InlineTable(Rc::new(RefCell::new(
          InlineTable::new(key_values, WSSep::new_str(" ", " "))
        ))));
      },
      &TOMLValue::Integer(ref s) => {
        if tval.validate() {
          return Some(Value::Integer(s.clone()))
        } else {
          return None;
        }
      },
      &TOMLValue::Float(ref s) => {
        if tval.validate() {
          return Some(Value::Float(s.clone()))
        } else {
          return None;
        }
      },
      &TOMLValue::Boolean(b) => return Some(Value::Boolean(b)),
      &TOMLValue::DateTime(ref dt) => {
        if tval.validate() {
          return Some(Value::DateTime(dt.clone()))
        } else {
          return None;
        }
      },
      &TOMLValue::String(ref s, st) => {
        if tval.validate() {
          return Some(Value::String(s.clone(), st))
        } else {
          return None;
        }
      },
    }
  }
  
  fn same_structure(val_rf: &Rc<RefCell<Value<'a>>>, tval: &TOMLValue<'a>) -> bool {
    if !tval.validate() {
      return false;
    }
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
  
  fn rebuild_vector(self: &mut Parser<'a>, key: String, val: Rc<RefCell<Value<'a>>>, skip: bool) {
    match *val.borrow() {
      Value::Array(ref arr) => {
        {
          let value = match self.map.entry(key.clone()) {
            Entry::Occupied(entry) => entry.into_mut(),
            _ => panic!("Map contains array key, but map.entry returned a Vacant entry in set_value."),
          };
          if !skip {
            value.value = Some(val.clone());
          }
          value.subkeys = Children::Count(Cell::new(arr.borrow().values.len()));
        }
        for i in 0..arr.borrow().values.len() {
          let subkey = format!("{}[{}]", key, i);
          self.rebuild_vector(subkey, arr.borrow().values[i].val.clone(), false);
        }
      },
      Value::InlineTable(ref it) => {
        {
          let value = match self.map.entry(key.clone()) {
            Entry::Occupied(entry) => entry.into_mut(),
            _ => panic!("Map contains inline table key, but map.entry returned a Vacant entry is set_value."),
          };
          if !skip {
            value.value = Some(val.clone());
          }
          if let Children::Keys(ref child_keys) = value.subkeys {
            for i in 0..it.borrow().keyvals.len() {
              Parser::insert(child_keys, it.borrow().keyvals[i].keyval.key.clone().into_owned());
            }
          }
        }
        for i in 0..it.borrow().keyvals.len() {
          let subkey = format!("{}.{}", key, &it.borrow().keyvals[i].keyval.key);
          self.rebuild_vector(subkey, it.borrow().keyvals[i].keyval.val.clone(), false);
        }
      },
      _ => {
        self.map.entry(key.clone()).or_insert(
          HashValue::new_count(val.clone())
        );
      },
    }
  }
  
  fn get_all_subkeys(self: &Parser<'a>, key: &str) -> Vec<String>{
    let hv_opt = self.map.get(key);
    let mut all_keys = vec![];
    if let Some(hv) = hv_opt {
      let children = &hv.subkeys;
      match children {
        &Children::Count(ref cell) => {
          for i in 0..cell.get() {
            let subkey = format!("{}[{}]", key, i);
            all_keys.push(subkey.clone());
            all_keys.append(&mut self.get_all_subkeys(&subkey));
          }
        },
        &Children::Keys(ref rc_hs) => {
          for childkey in rc_hs.borrow().iter(){
            let subkey = format!("{}.{}", key, childkey);
            all_keys.push(subkey.clone());
            all_keys.append(&mut self.get_all_subkeys(&subkey));
          }
        },
      }
    }
    all_keys
  }
  
  fn replace_values(val_rf: &Rc<RefCell<Value<'a>>>, tval: &TOMLValue<'a>) -> bool {
    let value = match (&*val_rf.borrow(), tval) {
      (&Value::Array(ref arr), &TOMLValue::Array(ref t_arr)) => {
        let borrow = arr.borrow();
        let len = borrow.values.len();
        for i in 0..len {
          if !Parser::replace_values(&borrow.values[i].val, &t_arr[i]) {
            return false;
          }
        }
        return true;
      },
      (&Value::InlineTable(ref it), &TOMLValue::InlineTable(ref t_it)) => {
        let borrow = it.borrow();
        let len = borrow.keyvals.len();
        for i in 0..len {
          if !Parser::replace_values(&borrow.keyvals[i].keyval.val, &t_it[i].1) {
            return false;
          }
        }
        return true;
      },
      (_, &TOMLValue::Integer(ref v)) 	=> Value::Integer(v.clone()),
      (_, &TOMLValue::Float(ref v))			=> Value::Float(v.clone()),
      (_, &TOMLValue::Boolean(v)) 			=> Value::Boolean(v),
      (_, &TOMLValue::DateTime(ref v))	=> Value::DateTime(v.clone()),
      (_, &TOMLValue::String(ref s, t))	=> Value::String(s.clone(), t),
      (v, tv)                             => panic!("Check for the same structure should have eliminated the possibility of replacing {} with {}", v, tv),
    };
    *val_rf.borrow_mut() = value;
    return true;
  }

  pub fn sanitize_array(arr: Rc<RefCell<Array<'a>>>) -> TOMLValue<'a> {
		let mut result: Vec<TOMLValue> = vec![];
		for av in arr.borrow().values.iter() {
			result.push(to_tval!(&*av.val.borrow()));
		}
		TOMLValue::Array(Rc::new(result))
	}
	
	pub fn sanitize_inline_table(it: Rc<RefCell<InlineTable<'a>>>) -> TOMLValue<'a> {
		let mut result: Vec<(Cow<'a, str>, TOMLValue)> = vec![];
		for kv in it.borrow().keyvals.iter() {
			result.push((kv.keyval.key.clone(), to_tval!(&*kv.keyval.val.borrow())));
		}
		return TOMLValue::InlineTable(Rc::new(result));
	}
}

impl<'a> Display for Parser<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", *self.root.borrow())
	}
}

#[cfg(test)]
mod test {
  use std::cell::{Cell, RefCell};
  use std::rc::Rc;
  use parser::Parser;
  use types::{TOMLValue, Children, StrType, Str, Date, Time, DateTime, Bool};
  struct TT;
  impl TT {
    fn get<'a>() -> &'a str {
      return r#"animal = "bear"

[car]
model = "Civic"
"ωλèèℓƨ" = 4
"ƭôƥ ƨƥèèδ" = 124.56
"Date of Manufacture" = 2007-05-16T10:12:13.2324+04:00
drivers = ["Bob", "Jane", "John", "Michael", { disallowed = "Chris", banned="Sally"}]
properties = { color = "red", "plate number" = "ABC 345",
               accident_dates = [2008-09-29, 2011-01-16, 2014-11-30T03:13:54]}

[car.interior.seats]
type = '''fabric'''
count = 5

[[car.owners]]
Name = """Bob Jones"""
Age = 25
[[car.owners]]
Name = 'Jane Doe'
Age = 44"#;
    }
  }
  

  #[test]
  fn test_bare_key() {
    let mut p = Parser::new();
    p = p.parse(TT::get());
    assert_eq!(p.get_value("animal".to_string()), res2opt!(TOMLValue::basic_string("bear")));
  }

  #[test]
  fn test_key_val() {
    let mut p = Parser::new();
    p = p.parse(TT::get());
    assert_eq!(p.get_value("car.model".to_string()), res2opt!(TOMLValue::basic_string("Civic")));
  }

  #[test]
  fn test_quoted_key_val_int() {
    let mut p = Parser::new();
    p = p.parse(TT::get());
    assert_eq!(p.get_value("car.\"ωλèèℓƨ\"".to_string()), res2opt!(TOMLValue::int_str("4")));
  }

  #[test]
  fn test_quoted_key_val_float() {
    let mut p = Parser::new();
    p = p.parse(TT::get());
    assert_eq!(p.get_value("car.\"ƭôƥ ƨƥèèδ\"".to_string()), res2opt!(TOMLValue::float_str("124.56")));
  }

  #[test]
  fn test_key_array() {
    let mut p = Parser::new();
    p = p.parse(TT::get());
    assert_eq!(p.get_value("car.drivers[0]".to_string()), res2opt!(TOMLValue::basic_string("Bob")));
    assert_eq!(p.get_value("car.drivers[1]".to_string()), res2opt!(TOMLValue::basic_string("Jane")));
    assert_eq!(p.get_value("car.drivers[2]".to_string()), res2opt!(TOMLValue::basic_string("John")));
    assert_eq!(p.get_value("car.drivers[3]".to_string()), res2opt!(TOMLValue::basic_string("Michael")));
    assert_eq!(p.get_value("car.drivers[4].disallowed".to_string()), res2opt!(TOMLValue::basic_string("Chris")));
    assert_eq!(p.get_value("car.drivers[4].banned".to_string()), res2opt!(TOMLValue::basic_string("Sally")));
  }

  #[test]
  fn test_key_inline_table() {
    let mut p = Parser::new();
    p = p.parse(TT::get());
    assert_eq!(p.get_value("car.properties.color".to_string()), res2opt!(TOMLValue::basic_string("red")));
    assert_eq!(p.get_value("car.properties.\"plate number\"".to_string()), res2opt!(TOMLValue::basic_string("ABC 345")));
    assert_eq!(p.get_value("car.properties.accident_dates[0]".to_string()), res2opt!(TOMLValue::date_str("2008", "09", "29")));
    assert_eq!(p.get_value("car.properties.accident_dates[1]".to_string()), res2opt!(TOMLValue::date_int(2011, 1, 16)));
    assert_eq!(p.get_value("car.properties.accident_dates[2]".to_string()), res2opt!(TOMLValue::datetime_str("2014", "11", "30", "03", "13", "54")));
  }

  #[test]
  fn test_implicit_table() {
    
    let mut p = Parser::new();
    p = p.parse(TT::get());
    assert_eq!(p.get_value("car.interior.seats.type".to_string()), res2opt!(TOMLValue::ml_literal_string("fabric")));
    assert_eq!(p.get_value("car.interior.seats.count".to_string()), res2opt!(TOMLValue::int_str("5")));
  }

  #[test]
  fn test_array_table() {
    let mut p = Parser::new();
    p = p.parse(TT::get());
    assert_eq!(p.get_value("car.owners[0].Name".to_string()), res2opt!(TOMLValue::ml_basic_string("Bob Jones")));
    assert_eq!(p.get_value("car.owners[0].Age".to_string()), Some(TOMLValue::int(25)));
    assert_eq!(p.get_value("car.owners[1].Name".to_string()), res2opt!(TOMLValue::literal_string("Jane Doe")));
    assert_eq!(p.get_value("car.owners[1].Age".to_string()), Some(TOMLValue::int(44)));
  }
  
  #[test]
  fn test_get_root_children() {
    let mut p = Parser::new();
    p = p.parse(TT::get());
    assert_eq!(p.get_children("".to_string()), Some(&Children::Keys(RefCell::new(vec!["animal".to_string(), "car".to_string()]))));
  }
  
  #[test]
  fn test_get_table_children() {
    let mut p = Parser::new();
    p = p.parse(TT::get());
    assert_eq!(p.get_children("car".to_string()),
      Some(&Children::Keys(RefCell::new(vec!["model".to_string(), "\"ωλèèℓƨ\"".to_string(), "\"ƭôƥ ƨƥèèδ\"".to_string(),
        "\"Date of Manufacture\"".to_string(), "drivers".to_string(), "properties".to_string(), "interior".to_string(),
        "owners".to_string()]))));
  }
  
  #[test]
  fn test_get_array_children() {
    let mut p = Parser::new();
    p = p.parse(TT::get());
    assert_eq!(p.get_children("car.drivers".to_string()), Some(&Children::Count(Cell::new(5))));
  }
  
  #[test]
  fn test_get_inline_table_children() {
    let mut p = Parser::new();
    p = p.parse(TT::get());
    assert_eq!(p.get_children("car.properties".to_string()),
      Some(&Children::Keys(RefCell::new(vec!["color".to_string(), "\"plate number\"".to_string(), "accident_dates".to_string()]))));
  }
  
  #[test]
  fn test_get_nested_inline_table_children() {
    let mut p = Parser::new();
    p = p.parse(TT::get());
    assert_eq!(p.get_children("car.drivers[4]".to_string()),
      Some(&Children::Keys(RefCell::new(vec!["disallowed".to_string(), "banned".to_string()]))));
  }
  
  #[test]
  fn test_get_nested_array_children() {
    let mut p = Parser::new();
    p = p.parse(TT::get());
    assert_eq!(p.get_children("car.properties.accident_dates".to_string()), Some(&Children::Count(Cell::new(3))));
  }
  
  #[test]
  fn test_implicit_table_children() {
    let mut p = Parser::new();
    p = p.parse(TT::get());
    assert_eq!(p.get_children("car.interior.seats".to_string()),
      Some(&Children::Keys(RefCell::new(vec!["type".to_string(), "count".to_string()]))));
  }
  
  #[test]
  fn test_get_array_of_table_children() {
    let mut p = Parser::new();
    p = p.parse(TT::get());
    assert_eq!(p.get_children("car.owners".to_string()), Some(&Children::Count(Cell::new(2))));
  }
  
  #[test]
  fn test_get_array_of_table0_children() {
    let mut p = Parser::new();
    p = p.parse(TT::get());
    assert_eq!(p.get_children("car.owners[0]".to_string()),
      Some(&Children::Keys(RefCell::new(vec!["Name".to_string(), "Age".to_string()]))));
  }
  
  #[test]
  fn test_get_array_of_table1_children() {
    let mut p = Parser::new();
    p = p.parse(TT::get());
    assert_eq!(p.get_children("car.owners[1]".to_string()),
      Some(&Children::Keys(RefCell::new(vec!["Name".to_string(), "Age".to_string()]))));
  }
  
  #[test]
  fn test_set_bare_key() {
    let mut p = Parser::new();
    p = p.parse(TT::get());
    p.set_value("animal".to_string(), TOMLValue::ml_basic_string("shark").unwrap());
    assert_eq!(p.get_value("animal".to_string()),
      Some(TOMLValue::String(Str::String("shark".to_string()), StrType::MLBasic)));
  }
  
  #[test]
  fn test_set_table_key() {
    let mut p = Parser::new();
    p = p.parse(TT::get());
    p.set_value("car.model".to_string(), TOMLValue::literal_string("Accord").unwrap());
    assert_eq!(p.get_value("car.model".to_string()),
      Some(TOMLValue::String(Str::String("Accord".to_string()), StrType::Literal)));
  }
  
  #[test]
  fn test_set_array_element_key() {
    let mut p = Parser::new();
    p = p.parse(TT::get());
    p.set_value("car.drivers[1]".to_string(), TOMLValue::ml_literal_string("Mark").unwrap());
    assert_eq!(p.get_value("car.drivers[1]".to_string()),
      Some(TOMLValue::String(Str::String("Mark".to_string()), StrType::MLLiteral)));
  }
  
  #[test]
  fn test_set_nested_aray_element_key() {
    let mut p = Parser::new();
    p = p.parse(TT::get());
    p.set_value("car.properties.accident_dates[2]".to_string(), TOMLValue::float(3443.34));
    assert_eq!(p.get_value("car.properties.accident_dates[2]".to_string()),
      Some(TOMLValue::Float(Str::String("3443.34".to_string()))));
  }
  
  #[test]
  fn test_set_inline_table_element_key() {
    let mut p = Parser::new();
    p = p.parse(TT::get());
    p.set_value("car.properties.color".to_string(), TOMLValue::int(19));
    assert_eq!(p.get_value("car.properties.color".to_string()),
      Some(TOMLValue::Integer(Str::String("19".to_string()))));
  }
  
  #[test]
  fn test_set_nested_inline_table_element_key() {
    let mut p = Parser::new();
    p = p.parse(TT::get());
    p.set_value("car.drivers[4].banned".to_string(), TOMLValue::datetime_int(2013, 9, 23, 17, 34, 2).unwrap());
    assert_eq!(p.get_value("car.drivers[4].banned".to_string()),
      Some(TOMLValue::DateTime(DateTime::new(Date::new_str("2013", "09", "23"),
        Some(Time::new_str("17", "34", "02", None, None))))));
  }
  
  #[test]
  fn test_truncate_array() {
    let mut p = Parser::new();
    p = p.parse(TT::get());
    p.set_value("car.drivers".to_string(), TOMLValue::Array(Rc::new(
      vec![TOMLValue::basic_string("Phil").unwrap(), TOMLValue::basic_string("Mary").unwrap()]
    )));
    assert_eq!(p.get_value("car.drivers".to_string()),
      Some(TOMLValue::Array(Rc::new(
        vec![TOMLValue::String(Str::String("Phil".to_string()), StrType::Basic),
             TOMLValue::String(Str::String("Mary".to_string()), StrType::Basic)
        ]
      ))));
  }
  
  #[test]
  fn test_truncate_inline_table() {
    let mut p = Parser::new();
    p = p.parse(TT::get());
    p.set_value("car.properties".to_string(), TOMLValue::InlineTable(Rc::new(
      vec![(Str::String("make".to_string()), TOMLValue::literal_string("Honda").unwrap()),
           (Str::String("transmission".to_string()), TOMLValue::bool(true))]
    )));
    assert_eq!(p.get_value("car.properties".to_string()),
      Some(TOMLValue::InlineTable(Rc::new(
        vec![(Str::String("make".to_string()), TOMLValue::String(Str::String("Honda".to_string()), StrType::Literal)),
             (Str::String("transmission".to_string()), TOMLValue::Boolean(Bool::True))]
      ))));
  }
  
  #[test]
  fn test_extend_array() {
    let mut p = Parser::new();
    p = p.parse(TT::get());
    p.set_value("car.drivers".to_string(), TOMLValue::Array(Rc::new(
      vec![TOMLValue::int(1), TOMLValue::int(2), TOMLValue::int(3), TOMLValue::int(4),
      TOMLValue::int(5), TOMLValue::int(6), TOMLValue::int(7), TOMLValue::int(8)]
    )));
    assert_eq!(p.get_value("car.drivers".to_string()),
      Some(TOMLValue::Array(Rc::new(
        vec![TOMLValue::Integer(Str::String("1".to_string())),
             TOMLValue::Integer(Str::String("2".to_string())),
             TOMLValue::Integer(Str::String("3".to_string())),
             TOMLValue::Integer(Str::String("4".to_string())),
             TOMLValue::Integer(Str::String("5".to_string())),
             TOMLValue::Integer(Str::String("6".to_string())),
             TOMLValue::Integer(Str::String("7".to_string())),
             TOMLValue::Integer(Str::String("8".to_string())),
        ]
      ))));
  }
  
  #[test]
  fn test_extend_inline_table() {
    let mut p = Parser::new();
    p = p.parse(TT::get());
    p.set_value("car.properties".to_string(), TOMLValue::InlineTable(Rc::new(
      vec![(Str::String("prop1".to_string()), TOMLValue::bool_str("TrUe").unwrap()),
           (Str::String("prop2".to_string()), TOMLValue::bool_str("FALSE").unwrap()),
           (Str::String("prop3".to_string()), TOMLValue::bool_str("truE").unwrap()),
           (Str::String("prop4".to_string()), TOMLValue::bool_str("false").unwrap())]
    )));
    assert_eq!(p.get_value("car.properties".to_string()),
      Some(TOMLValue::InlineTable(Rc::new(
        vec![(Str::String("prop1".to_string()), TOMLValue::Boolean(Bool::True)),
             (Str::String("prop2".to_string()), TOMLValue::Boolean(Bool::False)),
             (Str::String("prop3".to_string()), TOMLValue::Boolean(Bool::True)),
             (Str::String("prop4".to_string()), TOMLValue::Boolean(Bool::False))]
      ))));
  }
  
  #[test]
  fn test_set_implicit_table_key() {
    let mut p = Parser::new();
    p = p.parse(TT::get());
    p.set_value("car.interior.seats.type".to_string(), TOMLValue::basic_string("leather").unwrap());
    assert_eq!(p.get_value("car.interior.seats.type".to_string()),
      Some(TOMLValue::String(Str::String("leather".to_string()), StrType::Basic)));
  }
  
  #[test]
  fn test_set_array_of_table0_key() {
    let mut p = Parser::new();
    p = p.parse(TT::get());
    p.set_value("car.owners[0].Age".to_string(), TOMLValue::float_str("19.5").unwrap());
    assert_eq!(p.get_value("car.owners[0].Age".to_string()),
      Some(TOMLValue::Float(Str::String("19.5".to_string()))));
  }
  
  #[test]
  fn test_set_array_of_table1_key() {
    let mut p = Parser::new();
    p = p.parse(TT::get());
    p.set_value("car.owners[1].Name".to_string(), TOMLValue::ml_basic_string("Steve Parker").unwrap());
    assert_eq!(p.get_value("car.owners[1].Name".to_string()),
      Some(TOMLValue::String(Str::String("Steve Parker".to_string()), StrType::MLBasic)));
  }
  
  #[test]
  fn test_output_after_set() {
    let mut p = Parser::new();
    p = p.parse(TT::get());
     p.set_value("car.interior.seats.type".to_string(), TOMLValue::basic_string("leather").unwrap());
     p.set_value("car.interior.seats.type".to_string(), TOMLValue::basic_string("vinyl").unwrap());
    p.set_value("car.owners[0].Age".to_string(), TOMLValue::float_str("19.5").unwrap());
    p.set_value("car.owners[1].Name".to_string(), TOMLValue::ml_basic_string("Steve Parker").unwrap());
    p.set_value("car.drivers[4].banned".to_string(), TOMLValue::datetime_int(2013, 9, 23, 17, 34, 2).unwrap());
    p.set_value("car.properties.color".to_string(), TOMLValue::int(19));
    p.set_value("car.properties.accident_dates[2]".to_string(), TOMLValue::float(3443.34));
    p.set_value("car.drivers[1]".to_string(), TOMLValue::ml_literal_string("Mark").unwrap());
    p.set_value("car.properties".to_string(), TOMLValue::InlineTable(Rc::new(
      vec![(Str::String("make".to_string()), TOMLValue::literal_string("Honda").unwrap()),
           (Str::String("transmission".to_string()), TOMLValue::bool(true))]
    )));
    p.set_value("car.drivers".to_string(), TOMLValue::Array(Rc::new(
      vec![TOMLValue::basic_string("Phil").unwrap(), TOMLValue::basic_string("Mary").unwrap()]
    )));
    p.set_value("car.properties".to_string(), TOMLValue::InlineTable(Rc::new(
      vec![(Str::String("prop1".to_string()), TOMLValue::bool_str("TrUe").unwrap()),
           (Str::String("prop2".to_string()), TOMLValue::bool_str("FALSE").unwrap()),
           (Str::String("prop3".to_string()), TOMLValue::bool_str("truE").unwrap()),
           (Str::String("prop4".to_string()), TOMLValue::bool_str("false").unwrap())]
    )));
    p.set_value("car.drivers".to_string(), TOMLValue::Array(Rc::new(
      vec![TOMLValue::int(1), TOMLValue::int(2), TOMLValue::int(3), TOMLValue::int(4),
      TOMLValue::int(5), TOMLValue::int(6), TOMLValue::int(7), TOMLValue::int(8)]
    )));
    p.set_value("car.model".to_string(), TOMLValue::literal_string("Accord").unwrap());
    p.set_value("animal".to_string(), TOMLValue::ml_basic_string("shark").unwrap());
    assert_eq!(r#"animal = """shark"""

[car]
model = 'Accord'
"ωλèèℓƨ" = 4
"ƭôƥ ƨƥèèδ" = 124.56
"Date of Manufacture" = 2007-05-16T10:12:13.2324+04:00
drivers = [1, 2, 3, 4, 5, 6, 7, 8]
properties = { prop1 = true, prop2 = false, prop3 = true, prop4 = false }

[car.interior.seats]
type = "vinyl"
count = 5

[[car.owners]]
Name = """Bob Jones"""
Age = 19.5
[[car.owners]]
Name = """Steve Parker"""
Age = 44"#, format!("{}", p));
  }
}