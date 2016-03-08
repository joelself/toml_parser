use std::fmt;
use std::fmt::Display;
use std::rc::Rc;
use std::cell::{RefCell, Cell};
use std::collections::HashSet;
use std::option::Option;
use ::types::{DateTime, StrType, Str, Bool, Children};


/// Compares two Options that contain comparable structs
pub fn comp_opt<T: Eq>(left: &Option<T>, right: &Option<T>) -> bool {
	match (left, right) {
		(&Some(ref i), &Some(ref j)) if i == j => true,
		(&None, &None) => true,
		_ => false
	}
}

#[allow(dead_code)]
pub enum ErrorCode {
	BasicString = 0,
	MLBasicString = 1,
	LiteralString = 2,
	MLLiteralString = 3,
}

#[derive(Debug, Eq)]
pub struct Toml<'a> {
	pub exprs: Vec<NLExpression<'a>>,
}

impl<'a> PartialEq for Toml<'a> {
	fn eq(&self, other: &Toml<'a>) -> bool {
		self.exprs == other.exprs
	}
}

impl<'a> Display for Toml<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    	for i in 0..self.exprs.len()-1 {
    		try!(write!(f, "{}", self.exprs[i]));
    	}
		write!(f, "{}", self.exprs[self.exprs.len()-1])
   }
}

#[allow(dead_code)]
impl<'a> Toml<'a> {
	pub fn new(exprs: Vec<NLExpression<'a>>) -> Toml<'a> {
		Toml{exprs: exprs}
	}
}

#[derive(Debug, Eq)]
pub struct NLExpression<'a> {
	pub nl: Str<'a>,
	pub expr: Expression<'a>,
}

impl<'a> PartialEq for NLExpression<'a> {
	fn eq(&self, other: &NLExpression<'a>) -> bool {
		self.nl == other.nl &&
		self.expr == other.expr
	}
}

impl<'a> Display for NLExpression<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    	write!(f, "{}{}", self.nl, self.expr)
    }
}

#[allow(dead_code)]
impl<'a> NLExpression<'a> {
	pub fn new_str(nl: &'a str, expr: Expression<'a>) -> NLExpression<'a> {
		NLExpression{nl: Str::Str(nl), expr: expr}
	}
	pub fn new_string(nl: String, expr: Expression<'a>) -> NLExpression<'a> {
		NLExpression{nl: Str::String(nl), expr: expr}
	}
}

// <ws.ws1>
// <ws.ws1><comment>
// <ws.ws1><keyval><ws.ws2><comment?>
// <ws.ws1><table><ws.ws2><comment?>
#[derive(Debug, Eq)]
pub struct Expression<'a> {
	pub ws: WSSep<'a>,
	pub keyval: Option<KeyVal<'a>>,
	pub table: Option<Rc<TableType<'a>>>,
	pub comment: Option<Comment<'a>>,
}

impl<'a> PartialEq for Expression<'a> {
	fn eq(&self, other: &Expression<'a>) -> bool {
		self.ws == other.ws &&
		comp_opt(&self.keyval, &other.keyval) &&
		comp_opt(&self.table, &other.table) &&
		comp_opt(&self.comment, &other.comment)
	}
}

impl<'a> Display for Expression<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    	match (&self.ws, &self.keyval, &self.table, &self.comment) {
    		(ws, &None, &None, &None) => write!(f, "{}", ws.ws1),
    		(ws, &None, &None, &Some(ref c)) => write!(f, "{}{}", ws.ws1, c),
    		(ws, &Some(ref k), &None, &Some(ref c)) => write!(f, "{}{}{}{}", ws.ws1, k, ws.ws2, c),
    		(ws, &Some(ref k), &None, &None) => write!(f, "{}{}{}", ws.ws1, k, ws.ws2),
    		(ws, &None, &Some(ref t), &Some(ref c)) => write!(f, "{}{}{}{}", ws.ws1, t, ws.ws2, c),
    		(ws, &None, &Some(ref t), &None) => write!(f, "{}{}{}", ws.ws1, t, ws.ws2),
    		_ => panic!("Invalid expression: ws1: \"{}\", ws2: \"{}\", keyval: {:?}, table: {:?}, comment: {:?}",
    			self.ws.ws1, self.ws.ws2, self.keyval, self.table, self.comment),
    	}
    }
}

impl<'a> Expression<'a> {
	pub fn new(ws: WSSep<'a>, keyval: Option<KeyVal<'a>>, table: Option<Rc<TableType<'a>>>,
		comment: Option<Comment<'a>>) -> Expression<'a> {
		Expression{ws: ws, keyval: keyval, table: table, comment: comment}
	}
}

#[derive(Debug, Eq, Clone)]
pub enum Value<'a> {
	Integer(Str<'a>),
	Float(Str<'a>),
	Boolean(Bool),
	DateTime(DateTime<'a>),
	Array(Rc<RefCell<Array<'a>>>),
	String(Str<'a>, StrType),
	InlineTable(Rc<RefCell<InlineTable<'a>>>),
}

// impl<'a> Value<'a> {
//   pub fn validate_string(&self) -> bool {
//     match self {
//       &Value::Integer(ref s) => {return true},
//       &Value::Float(ref s) => {return true},
//       &Value::DateTime(ref dt) => {dt.validate(true)},
//       &Value::String(ref s, st) => {return true},
//       _ => return true,
//     }
//   }
// }

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum ArrayType {
	Integer,
	Float,
	Boolean,
	DateTime,
	Array,
	String,
	InlineTable,
	None,
}

#[derive(Debug, Eq, Clone)]
pub struct HashValue<'a> {
	pub value: Option<Rc<RefCell<Value<'a>>>>, 
	pub subkeys: Children,
}

impl<'a> Display for HashValue<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self.subkeys {
      Children::Count(ref c) => try!(write!(f, "Subkey Count: {}, ", c.get())),
      Children::Keys(ref keys) => {
        try!(write!(f, "Subkey Set: "));
        for key in keys.borrow().iter() {
          try!(write!(f, "{}, ", key));
        }
      },
    }
		if let Some(ref v) = self.value {
			write!(f, "Value: {}", *v.borrow())
		} else {
			write!(f, "No Value")
		}
	}
}

#[allow(dead_code)]
impl<'a> HashValue<'a> {
	pub fn new_keys(value: Rc<RefCell<Value<'a>>>) -> HashValue<'a> {
		HashValue {
			value: Some(value),
			subkeys: Children::Keys(RefCell::new(HashSet::new())),
		}
	}
	pub fn none_keys() -> HashValue<'a> {
		HashValue {
			value: None,
			subkeys: Children::Keys(RefCell::new(HashSet::new())),
		}
	}
  pub fn one_keys(key: String) -> HashValue<'a> {
    let mut set = HashSet::new();
    set.insert(key);
    HashValue {
      value: None,
      subkeys: Children::Keys(RefCell::new(set)),
    }
  }
	pub fn new_count(value: Rc<RefCell<Value<'a>>>) -> HashValue<'a> {
		HashValue {
			value: Some(value),
			subkeys: Children::Count(Cell::new(0)),
		}
	}
  pub fn none_count() -> HashValue<'a> {
    HashValue {
      value: None,
      subkeys: Children::Count(Cell::new(0)),
    }
  }
  pub fn one_count() -> HashValue<'a> {
    HashValue {
      value: None,
      subkeys: Children::Count(Cell::new(1)),
    }
  }
}

impl<'a> PartialEq for HashValue<'a> {
	fn eq(&self, other: &HashValue<'a>) -> bool {
		self.value == other.value
	}
}

impl<'a> PartialEq for Value<'a> {
	fn eq(&self, other: &Value<'a>) -> bool {
		match (self, other) {
			(&Value::Integer(ref i), &Value::Integer(ref j)) if i == j => true,
			(&Value::Float(ref i), &Value::Float(ref j)) if i == j => true,
			(&Value::Boolean(ref i), &Value::Boolean(ref j)) if i == j => true,
			(&Value::DateTime(ref i), &Value::DateTime(ref j)) if i == j => true,
			(&Value::Array(ref i), &Value::Array(ref j)) if i == j => true,
			(&Value::String(ref i, _), &Value::String(ref j, _)) if i == j => true,
			(&Value::InlineTable(ref i), &Value::InlineTable(ref j)) if i == j => true,
			_ => false
		}
	}
}

impl<'a> Display for Value<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			&Value::Integer(ref i) => write!(f, "{}", i),
			&Value::Float(ref i) => write!(f, "{}", i),
			&Value::Boolean(ref i) => write!(f, "{}", i),
			&Value::DateTime(ref i) => write!(f, "{}", i),
			&Value::Array(ref i) => write!(f, "{}", *i.borrow()),
			&Value::InlineTable(ref i) => write!(f, "{}", *i.borrow()),
			&Value::String(ref i, ref t) =>  {
				match t {
					&StrType::Basic => write!(f, "\"{}\"", i),
					&StrType::MLBasic => write!(f, "\"\"\"{}\"\"\"", i),
					&StrType::Literal => write!(f, "'{}'", i),
					&StrType::MLLiteral => write!(f, "'''{}'''", i),
				}
			},
		}
   }
}

#[derive(Debug, Eq)]
pub enum TableType<'a>{
	Standard(Table<'a>),
	Array(Table<'a>),
}

impl<'a> PartialEq for TableType<'a> {
	fn eq(&self, other: &TableType<'a>) -> bool {
		match (self, other) {
			(&TableType::Standard(ref i), &TableType::Standard(ref j)) if i == j => true,
			(&TableType::Array(ref i), &TableType::Array(ref j)) if i == j => true,
			_ => false
		}
	}
}

impl<'a> Display for TableType<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    	match self {
    		&TableType::Standard(ref t) => write!(f, "[{}]", t),
    		&TableType::Array(ref t) => write!(f, "[[{}]]", t),
    	}
    }
}

// #<text>
#[derive(Debug, Eq)]
pub struct Comment<'a> {
	pub text: Str<'a>,
}

impl<'a> PartialEq for Comment<'a> {
	fn eq(&self, other: &Comment<'a>) -> bool {
		self.text == other.text
	}
}

impl<'a> Display for Comment<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    	write!(f, "#{}", self.text)
    }
}

#[allow(dead_code)]
impl<'a> Comment<'a> {
	pub fn new_str(text: &'a str) -> Comment<'a> {
		Comment{text: Str::Str(text)}
	}
	pub fn new_string(text: String) -> Comment<'a> {
		Comment{text: Str::String(text)}
	}
}

#[derive(Debug, Eq)]
pub struct WSSep<'a> {
	pub ws1: Str<'a>,
	pub ws2: Str<'a>,
}

impl<'a> PartialEq for WSSep<'a> {
	fn eq(&self, other: &WSSep<'a>) -> bool {
		self.ws1 == other.ws1 &&
		self.ws2 == other.ws2
	}
}

#[allow(dead_code)]
impl<'a> WSSep<'a> {
	pub fn new_str(ws1: &'a str, ws2: &'a str) -> WSSep<'a> {
		WSSep{ws1: Str::Str(ws1), ws2: Str::Str(ws2)}
	}
	pub fn new_string(ws1: String, ws2: String) -> WSSep<'a> {
		WSSep{ws1: Str::String(ws1), ws2: Str::String(ws2)}
	}
}

// <key><keyval_sep.ws1>=<keyval_sep.ws2><val>
#[derive(Debug, Eq)]
pub struct KeyVal<'a> {
	pub key: Str<'a>,
	pub keyval_sep: WSSep<'a>,
	pub val: Rc<RefCell<Value<'a>>>,
}

impl<'a> PartialEq for KeyVal<'a> {
	fn eq(&self, other: &KeyVal<'a>) -> bool {
		self.key == other.key &&
		self.keyval_sep == other.keyval_sep &&
		self.val == other.val
	}
}

impl<'a> Display for KeyVal<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    	write!(f, "{}{}={}{}", self.key, self.keyval_sep.ws1, self.keyval_sep.ws2, *self.val.borrow())
    }
}

#[allow(dead_code)]
impl<'a> KeyVal<'a> {
    pub fn new_str(key: &'a str, keyval_sep: WSSep<'a>, val: Rc<RefCell<Value<'a>>>) -> KeyVal<'a> {
    	KeyVal{key: Str::Str(key), keyval_sep: keyval_sep, val: val}
    }
    pub fn new_string(key: String, keyval_sep: WSSep<'a>, val: Rc<RefCell<Value<'a>>>) -> KeyVal<'a> {
    	KeyVal{key: Str::String(key), keyval_sep: keyval_sep, val: val}
    }
}

// <ws.ws1>.<ws.ws2><key>
#[derive(Debug, Eq)]
pub struct WSKeySep<'a> {
	pub ws: WSSep<'a>,
	pub key: Str<'a>,
}

impl<'a> PartialEq for WSKeySep<'a> {
	fn eq(&self, other: &WSKeySep<'a>) -> bool {
		self.ws == other.ws &&
		self.key == other.key
	}
}

impl<'a> Display for WSKeySep<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    	write!(f, "{}.{}{}", self.ws.ws1, self.ws.ws2, self.key)
    }
}

#[allow(dead_code)]
impl<'a> WSKeySep<'a> {
    pub fn new_str(ws: WSSep<'a>, key: &'a str) -> WSKeySep<'a> {
    	WSKeySep{ws: ws, key: Str::Str(key)}
    }
    pub fn new_string(ws: WSSep<'a>, key: String) -> WSKeySep<'a> {
    	WSKeySep{ws: ws, key: Str::String(key)}
    }
}

pub fn get_last_keys(last_table: Option<&Table>, t: &Table) -> Vec<String> {
	match last_table {
		None => {
			let mut last_keys = vec!["$Root$".to_string()];
			for i in 0..t.keys.len() {
				last_keys.push(string!(t.keys[i].key));
			}
			last_keys
		},
		Some(lt) => {
			let len_last = lt.keys.len();
			let len = t.keys.len();
			let mut last_keys = vec![];
			for i in len_last..len {
				last_keys.push(string!(t.keys[i].key));
			}
			last_keys
		}
	}
}

pub fn format_keys(t: &Table) -> String {
	let mut s = String::new();
	if t.keys.len() > 0 {
		for i in 0..t.keys.len() - 1 {
			s.push_str(str!(t.keys[i].key));
			s.push('.');
		}
		s.push_str(str!(t.keys[t.keys.len() - 1].key));
	}
	s
}

pub fn format_tt_keys(tabletype: &TableType) -> String {
	match tabletype {
		&TableType::Standard(ref t) | &TableType::Array(ref t) => {
			let mut s = String::new();
			if t.keys.len() > 0 {
				for i in 0..t.keys.len() - 1 {
					s.push_str(str!(t.keys[i].key));
					s.push('.');
				}
				s.push_str(str!(t.keys[t.keys.len() - 1].key));
			}
			s
		}
	}
}

// Standard: [<ws.ws1><key><subkeys*><ws.ws2>]
// Array: [[<ws.ws1><key><subkeys*><ws.ws2>]]
#[derive(Debug, Eq)]
pub struct Table<'a> {
	pub keys: Vec<WSKeySep<'a>>,
}

impl<'a> PartialEq for Table<'a> {
	fn eq(&self, other: &Table<'a>) -> bool {
		self.keys == other.keys
	}
}

impl<'a> Display for Table<'a> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    if self.keys.len() > 0 {
      try!(write!(f, "{}{}", self.keys[0].ws.ws1, self.keys[0].key));
    	for i in 1..self.keys.len() {
    		try!(write!(f, "{}", self.keys[i]));
    	}
    	write!(f, "{}", self.keys[0].ws.ws2)
    }
    else {
      Ok(())
    }
  }
}

#[allow(dead_code)]
impl<'a> Table<'a> {
  pub fn new_str(ws: WSSep<'a>, key: &'a str, mut subkeys: Vec<WSKeySep<'a>>) -> Table<'a> {
  	subkeys.insert(0, WSKeySep::new_str(ws, key));
    Table{keys: subkeys}
  }
  pub fn new_string(ws: WSSep<'a>, key: String, mut subkeys: Vec<WSKeySep<'a>>) -> Table<'a> {
  	subkeys.insert(0, WSKeySep::new_string(ws, key));
    Table{keys: subkeys}
  }
}

impl<'a> TableType<'a> {
	pub fn is_subtable_of(&self, prev: &TableType<'a>) -> bool {
		match self { 
			&TableType::Standard(ref t) | &TableType::Array(ref t) => {
				match prev {
					&TableType::Standard(ref prev_table) | &TableType::Array(ref prev_table) => {
						let prev_key = format_keys(&prev_table);
						let key = format_keys(t);
						if let Some(i) = key.find(&prev_key) {
							return i == 0;
						}
					}
				}
			}
		}
		false
	}
}

// <comment><newlines+>
#[derive(Debug, Eq)]
pub struct CommentNewLines<'a> {
	pub pre_ws_nl: Str<'a>,
	pub comment: Comment<'a>,
	pub newlines: Str<'a>,
}

impl<'a> PartialEq for CommentNewLines<'a> {
	fn eq(&self, other: &CommentNewLines<'a>) -> bool {
		self.pre_ws_nl == other.pre_ws_nl &&
		self.comment == other.comment &&
		self.newlines == other.newlines
	}
}

impl<'a> Display for CommentNewLines<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    	write!(f, "{}{}{}", self.pre_ws_nl, self.comment, self.newlines)
    }
}

#[allow(dead_code)]
impl<'a> CommentNewLines<'a> {
    pub fn new_str(pre_ws_nl: &'a str, comment: Comment<'a>, newlines: &'a str)
    	-> CommentNewLines<'a> {
    	CommentNewLines{pre_ws_nl: Str::Str(pre_ws_nl), comment: comment,
    		newlines: Str::Str(newlines)}
    }
    pub fn new_string(pre_ws_nl: String, comment: Comment<'a>, newlines: String)
    	-> CommentNewLines<'a> {
    	CommentNewLines{pre_ws_nl: Str::String(pre_ws_nl), comment: comment,
    		newlines: Str::String(newlines)}
    }
}

#[derive(Debug, Eq)]
pub enum CommentOrNewLines<'a> {
	Comment(CommentNewLines<'a>),
	NewLines(Str<'a>),
}

impl<'a> PartialEq for CommentOrNewLines<'a> {
	fn eq(&self, other: &CommentOrNewLines<'a>) -> bool {
		match (self, other) {
			(&CommentOrNewLines::Comment(ref i), &CommentOrNewLines::Comment(ref j)) if i == j => true,
			(&CommentOrNewLines::NewLines(ref i), &CommentOrNewLines::NewLines(ref j)) if i == j => true,
			_ => false
		}
	}
}

impl<'a> Display for CommentOrNewLines<'a> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      &CommentOrNewLines::Comment(ref c) => write!(f, "{}", c),
      &CommentOrNewLines::NewLines(ref n) => write!(f, "{}", n),
    }
  }
}

// <val><<array_sep.ws1>,<array_sep.ws2>?><comment_nl?><array_vals?>
#[derive(Debug, Eq)]
pub struct ArrayValue<'a> {
	pub val: Rc<RefCell<Value<'a>>>,
	pub array_sep: Option<WSSep<'a>>,
	pub comment_nls: Vec<CommentOrNewLines<'a>>,
}

impl<'a> PartialEq for ArrayValue<'a> {
	fn eq(&self, other: &ArrayValue<'a>) -> bool {
		self.val == other.val &&
		comp_opt(&self.array_sep, &other.array_sep) &&
		self.comment_nls == other.comment_nls
	}
}

impl<'a> Display for ArrayValue<'a> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    if self.comment_nls.len() > 0 {
      match self.array_sep {
        Some(ref s) => try!(write!(f, "{}{},{}", *self.val.borrow(), s.ws1, s.ws2)),
        None => try!(write!(f, "{}", *self.val.borrow())),
      }
      for i in 0..self.comment_nls.len() - 1 {
        try!(write!(f, "{}", self.comment_nls[i]));
      }
      write!(f, "{}", self.comment_nls[self.comment_nls.len() - 1])
    } else {
      match self.array_sep {
        Some(ref s) => write!(f, "{}{},{}", *self.val.borrow(), s.ws1, s.ws2),
        None => write!(f, "{}", *self.val.borrow()),
      }
    }
  }
}

impl<'a> ArrayValue<'a> {
  pub fn new(val: Rc<RefCell<Value<'a>>>, array_sep: Option<WSSep<'a>>,
  	comment_nls: Vec<CommentOrNewLines<'a>>) -> ArrayValue<'a> {
  	ArrayValue{val: val, array_sep: array_sep, comment_nls: comment_nls}
  }
  pub fn default(val: Rc<RefCell<Value<'a>>>) -> ArrayValue<'a> {
    ArrayValue{val: val, array_sep: Some(WSSep::new_str("", " ")), comment_nls: vec![]}
  }
  pub fn last(val: Rc<RefCell<Value<'a>>>) -> ArrayValue<'a> {
    ArrayValue{val: val, array_sep: None, comment_nls: vec![]}
  }
}

// [<ws.ws1><values?><ws.ws2>]
#[derive(Debug, Eq)]
pub struct Array<'a> {
	pub values: Vec<ArrayValue<'a>>,
	pub comment_nls1: Vec<CommentOrNewLines<'a>>,
	pub comment_nls2: Vec<CommentOrNewLines<'a>>,
}

impl<'a> PartialEq for Array<'a> {
	fn eq(&self, other: &Array<'a>) -> bool {
		self.values == other.values &&
		self.comment_nls1 == other.comment_nls1 &&
		self.comment_nls2 == other.comment_nls2
	}
}

impl<'a> Display for Array<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    	try!(write!(f, "["));
    	for comment_nl in self.comment_nls1.iter() {
    		try!(write!(f, "{}", comment_nl));
    	}
			for val in self.values.iter() {
				try!(write!(f, "{}", val));
			}
    	for comment_nl in self.comment_nls2.iter() {
    		try!(write!(f, "{}", comment_nl));
    	}
			write!(f, "]")
    }
}

impl<'a> Array<'a> {
  pub fn new(values: Vec<ArrayValue<'a>>, comment_nls1: Vec<CommentOrNewLines<'a>>,
  	comment_nls2: Vec<CommentOrNewLines<'a>>,) -> Array<'a> {
  	Array{values: values, comment_nls1: comment_nls1, comment_nls2: comment_nls2}
  }
}

// <key><keyval_sep.ws1>=<keyval_sep.ws2><val><<table_sep.ws1>,<table_sep.ws2>?><keyvals?>
#[derive(Debug, Eq)]
pub struct TableKeyVal<'a> {
	pub keyval: KeyVal<'a>,
	pub kv_sep: WSSep<'a>,
}

impl<'a> PartialEq for TableKeyVal<'a> {
	fn eq(&self, other: &TableKeyVal<'a>) -> bool {
		self.keyval == other.keyval &&
		self.kv_sep == other.kv_sep
	}
}

impl<'a> Display for TableKeyVal<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    	write!(f, "{}{}{}", self.kv_sep.ws1, self.keyval, self.kv_sep.ws2)
    }
}

impl<'a> TableKeyVal<'a> {
    pub fn new(keyval: KeyVal<'a>, kv_sep: WSSep<'a>) -> TableKeyVal<'a> {
    	TableKeyVal{keyval: keyval, kv_sep: kv_sep}
    }
    pub fn default(key: &Str<'a>, val: Rc<RefCell<Value<'a>>>) -> TableKeyVal<'a> {
      match key {
        &Str::Str(s) => {
          let keyval = KeyVal::new_str(s, WSSep::new_str(" ", " "), val);
          TableKeyVal{keyval: keyval, kv_sep: WSSep::new_str(" ", "")}
        },
        &Str::String(ref s) => {
          let keyval = KeyVal::new_string(s.clone(), WSSep::new_str(" ", " "), val);
          TableKeyVal{keyval: keyval, kv_sep: WSSep::new_str(" ", "")}
        }
      }
    }
    pub fn first(key: &Str<'a>, val: Rc<RefCell<Value<'a>>>) -> TableKeyVal<'a> {
      match key {
        &Str::Str(s) => {
          let keyval = KeyVal::new_str(s, WSSep::new_str(" ", " "), val);
          TableKeyVal{keyval: keyval, kv_sep: WSSep::new_str("", "")}
        },
        &Str::String(ref s) => {
          let keyval = KeyVal::new_string(s.clone(), WSSep::new_str(" ", " "), val);
          TableKeyVal{keyval: keyval, kv_sep: WSSep::new_str("", "")}
        }
      }
    }
}

// {<ws.ws1><keyvals><ws.ws2>}
#[derive(Debug, Eq)]
pub struct InlineTable<'a> {
	pub keyvals: Vec<TableKeyVal<'a>>,
	pub ws: WSSep<'a>,
}

impl<'a> PartialEq for InlineTable<'a> {
	fn eq(&self, other: &InlineTable<'a>) -> bool {
		self.keyvals == other.keyvals &&
		self.ws == other.ws
	}
}

impl<'a> Display for InlineTable<'a> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
  	try!(write!(f, "{{{}", self.ws.ws1));
		for i in 0..self.keyvals.len() - 1 {
			try!(write!(f, "{},", self.keyvals[i]));
		}
		if self.keyvals.len() > 0 {
			try!(write!(f, "{}", self.keyvals[self.keyvals.len() - 1]));
		}
		write!(f, "{}}}", self.ws.ws2)
  }
}

impl<'a> InlineTable<'a> {
	pub fn new(keyvals: Vec<TableKeyVal<'a>>, ws: WSSep<'a>) -> InlineTable<'a> {
		InlineTable{keyvals: keyvals, ws: ws}
	}
}

#[cfg(test)]
mod test {
	use ast::structs::comp_opt;
	#[test]
	fn test_comp_opt() {
  	let (a, b) = (Some("value"), Some("value"));
		assert!(comp_opt(&a, &b));
		let d: Option<&str> = None;
		let c = Some("stuff");
		assert!(!comp_opt(&c, &d));
	}
}


