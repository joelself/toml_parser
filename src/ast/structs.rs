use std::fmt;
use std::fmt::Display;
use std::option::Option;
use nom::IResult;
use ::types::{DateTime, TimeOffset, TimeOffsetAmount, Value};

/// Compares two Options that contain comparable structs
///
/// # Examples
///
/// ```
/// # extern crate tomllib;
/// let (a, b) = (Some("value"), Some("value"));
/// assert!(tomllib::ast::structs::comp_opt(&a, &b));
/// ```
pub fn comp_opt<T: Eq>(left: &Option<T>, right: &Option<T>) -> bool {
	match (left, right) {
		(&Some(ref i), &Some(ref j)) if i == j => true,
		(&None, &None) => true,
		_ => false
	}
}

pub struct MyResult<'a>(pub IResult<&'a str, Toml<'a>>);

impl<'a> Display for MyResult<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let MyResult(ref res) = *self;
		match res {
			&IResult::Done( _, ref o) => write!(f, "{}", o),
			ref a => write!(f, "{:?}", a),
		}
	}
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

#[derive(Debug, Eq)]
pub struct NLExpression<'a> {
	pub nl: &'a str,
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

// <ws.ws1>
// <ws.ws1><comment>
// <ws.ws1><keyval><ws.ws2><comment?>
// <ws.ws1><table><ws.ws2><comment?>
#[derive(Debug, Eq)]
pub struct Expression<'a> {
	pub ws: WSSep<'a>,
	pub keyval: Option<KeyVal<'a>>,
	pub table: Option<TableType<'a>>,
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

impl<'a> PartialEq for Value<'a> {
	fn eq(&self, other: &Value<'a>) -> bool {
		match (self, other) {
			(&Value::Integer(ref i), &Value::Integer(ref j)) if i == j => true,
			(&Value::Float(ref i), &Value::Float(ref j)) if i == j => true,
			(&Value::Boolean(ref i), &Value::Boolean(ref j)) if i == j => true,
			(&Value::DateTime(ref i), &Value::DateTime(ref j)) if i == j => true,
			(&Value::Array(ref i), &Value::Array(ref j)) if i == j => true,
			(&Value::String(ref i), &Value::String(ref j)) if i == j => true,
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
			&Value::Array(ref i) => write!(f, "{}", i),
			&Value::String(ref i) => write!(f, "{}", i),
			&Value::InlineTable(ref i) => write!(f, "{}", i),
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

impl<'a> PartialEq for TimeOffset<'a> {
	fn eq(&self, other: &TimeOffset<'a>) -> bool {
		match (self, other) {
			(&TimeOffset::Z, &TimeOffset::Z) => true,
			(&TimeOffset::Time(ref i), &TimeOffset::Time(ref j)) if(i == j) => true,
			_ => false
		}
	}
}

impl<'a> Display for TimeOffset<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    	match self {
    		&TimeOffset::Z => write!(f, "Z"),
    		&TimeOffset::Time(ref t) => write!(f, "{}", t),
    	}
    }
}

// #<text>
#[derive(Debug, Eq)]
pub struct Comment<'a> {
	pub text: &'a str,
	pub nl: &'a str,
}

impl<'a> PartialEq for Comment<'a> {
	fn eq(&self, other: &Comment<'a>) -> bool {
		self.text == other.text &&
		self.nl == other.nl
	}
}

impl<'a> Display for Comment<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    	write!(f, "#{}{}", self.text, self.nl)
    }
}

#[derive(Debug, Eq)]
pub struct WSSep<'a> {
	pub ws1: &'a str,
	pub ws2: &'a str,
}

impl<'a> PartialEq for WSSep<'a> {
	fn eq(&self, other: &WSSep<'a>) -> bool {
		self.ws1 == other.ws1 &&
		self.ws2 == other.ws2
	}
}

// <key><keyval_sep.ws1>=<keyval_sep.ws2><val>
#[derive(Debug, Eq)]
pub struct KeyVal<'a> {
	pub key: &'a str,
	pub keyval_sep: WSSep<'a>,
	pub val: Value<'a>,
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
    	write!(f, "{}{}={}{}", self.key, self.keyval_sep.ws1, self.keyval_sep.ws2, self.val)
    }
}

// <ws.ws1>.<ws.ws2><key>
#[derive(Debug, Eq)]
pub struct WSKeySep<'a> {
	pub ws: WSSep<'a>,
	pub key: &'a str,
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

// Standard: [<ws.ws1><key><subkeys*><ws.ws2>]
// Array: [[<ws.ws1><key><subkeys*><ws.ws2>]]
#[derive(Debug, Eq)]
pub struct Table<'a> {
	pub ws: WSSep<'a>, // opening whitespace and closing whitespace
	pub key: &'a str,
	pub subkeys: Vec<WSKeySep<'a>>,
}

impl<'a> PartialEq for Table<'a> {
	fn eq(&self, other: &Table<'a>) -> bool {
		self.ws == other.ws &&
		self.key == other.key &&
		self.subkeys == other.subkeys
	}
}

impl<'a> Display for Table<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    	try!(write!(f, "{}{}", self.ws.ws1, self.key));
    	for key in &self.subkeys {
    		try!(write!(f, "{}", key));
    	}
    	write!(f, "{}", self.ws.ws2)
    }
}

// <hour>:<minute>:<second>(.<fraction>)?
#[derive(Debug, Eq)]
pub struct Time<'a> {
    pub hour: &'a str,
	pub minute: &'a str,
	pub second: &'a str,
	pub fraction: &'a str,
}

impl<'a> PartialEq for Time<'a> {
	fn eq(&self, other: &Time<'a>) -> bool {
		self.hour == other.hour &&
		self.minute == other.minute &&
		self.second == other.second &&
		self.fraction == other.fraction
	}
}

impl<'a> Display for Time<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    	if self.fraction == "" {
    		write!(f, "{}:{}:{}", self.hour, self.minute, self.second)
    	} else {
    		write!(f, "{}:{}:{}.{}", self.hour, self.minute, self.second, self.fraction)
    	}
    }
}

impl<'a> PartialEq for TimeOffsetAmount<'a> {
	fn eq(&self, other: &TimeOffsetAmount<'a>) -> bool {
		self.pos_neg == other.pos_neg &&
		self.hour == other.hour &&
		self.minute == other.minute
	}
}

impl<'a> Display for TimeOffsetAmount<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    	write!(f, "{}{}:{}", self.pos_neg, self.hour, self.minute)
    }
}

// <year>-<month>-<day>
#[derive(Debug, Eq)]
pub struct FullDate<'a> {
	pub year: &'a str,
	pub month: &'a str,
	pub day: &'a str,
}

impl<'a> PartialEq for FullDate<'a> {
	fn eq(&self, other: &FullDate<'a>) -> bool {
		self.year == other.year &&
		self.month == other.month &&
		self.day == other.day
	}
}

impl<'a> Display for FullDate<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    	write!(f, "{}-{}-{}", self.year, self.month, self.day)
    }
}

impl<'a> PartialEq for DateTime<'a> {
	fn eq(&self, other: &DateTime<'a>) -> bool {
		self.year == other.year &&
		self.month == other.month &&
		self.day == other.day && 
		self.hour == other.hour &&
		self.minute == other.minute &&
		self.second == other.second &&
		self.fraction == other.fraction &&
		self.offset == other.offset
	}
}

impl<'a> Display for DateTime<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    	write!(f, "{}-{}-{}T{}:{}:{}.{}{}",
    		self.year, self.month, self.day,
    		self.hour, self.minute, self.second, self.fraction,
    		self.offset)
    }
}

// <comment><newlines+>
#[derive(Debug, Eq)]
pub struct CommentNewLines<'a> {
	pub pre_ws_nl: &'a str,
	pub comment: Comment<'a>,
	pub newlines: &'a str,
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

#[derive(Debug, Eq)]
pub enum CommentOrNewLines<'a> {
	Comment(CommentNewLines<'a>),
	NewLines(&'a str),
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
	pub val: Value<'a>,
	pub array_sep: Option<WSSep<'a>>,
	pub comment_nl: Option<CommentOrNewLines<'a>>,
}

impl<'a> PartialEq for ArrayValue<'a> {
	fn eq(&self, other: &ArrayValue<'a>) -> bool {
		self.val == other.val &&
		comp_opt(&self.array_sep, &other.array_sep) &&
		comp_opt(&self.comment_nl, &other.comment_nl)
	}
}

impl<'a> Display for ArrayValue<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    	match(&self.array_sep, &self.comment_nl) {
    		(&Some(ref s), &None) => write!(f, "{}{},{}", self.val, s.ws1, s.ws2),
    		(&Some(ref s), &Some(ref c)) => write!(f, "{}{},{}{}", self.val, s.ws1, s.ws2, c),
    		(&None, &Some(ref c)) => write!(f, "{}{}", self.val, c),
    		(&None, &None) => write!(f, "{}", self.val)
    	}
    }
}

// [<ws.ws1><values?><ws.ws2>]
#[derive(Debug, Eq)]
pub struct Array<'a> {
	pub values: Vec<ArrayValue<'a>>,
	pub ws: WSSep<'a>,
}

impl<'a> PartialEq for Array<'a> {
	fn eq(&self, other: &Array<'a>) -> bool {
		self.values == other.values &&
		self.ws == other.ws
	}
}

impl<'a> Display for Array<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    	try!(write!(f, "[{}", self.ws.ws1));
		for val in self.values.iter() {
			try!(write!(f, "{}", val));
		}
		write!(f, "{}]", self.ws.ws2)
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
    	write!(f, "{}{}{}", self.keyval, self.kv_sep.ws1, self.kv_sep.ws2)
    }
}

/*#[derive(Debug, Eq)]
pub struct TableKeyVals<'a> {
	pub keyvals: Vec<TableKeyVal<'a>>
}

impl<'a> PartialEq for TableKeyVals<'a> {
	fn eq(&self, other: &TableKeyVals<'a>) -> bool {
		self.keyvals == other.keyvals
	}
}

impl<'a> Display for TableKeyVals<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    	for i in 0..self.keyvals.len() - 1 {
    		try!(write!(f, "{}", self.keyvals[i]))
    	}
    	write!(f, "{}", self.keyvals[self.keyvals.len() - 1])
    }
}*/

// {<ws.ws1><keyvals><ws.ws2>}
#[derive(Debug, Eq)]
pub struct InlineTable<'a> {
	pub keyvals: Option<Vec<TableKeyVal<'a>>>,
	pub ws: WSSep<'a>,
}

impl<'a> PartialEq for InlineTable<'a> {
	fn eq(&self, other: &InlineTable<'a>) -> bool {
		comp_opt(&self.keyvals, &other.keyvals) &&
		self.ws == other.ws
	}
}

fn write_table_vector<'a>(kvs: &Vec<TableKeyVal<'a>>, ws: &WSSep<'a>, f: &mut fmt::Formatter) -> fmt::Result {
	try!(write!(f, "{{{}", ws.ws1));
	for i in 0..kvs.len() - 1 {
		try!(write!(f, "{},", kvs[i]));
	}
	write!(f, "{}{}}}", kvs[kvs.len() - 1], ws.ws2)
}

impl<'a> Display for InlineTable<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    	match &self.keyvals {
    		&Some(ref k)	=> write_table_vector(k, &self.ws, f),
    		&None			=> write!(f, "{{{}{}}}", self.ws.ws1, self.ws.ws2),
    	}
    }
}


