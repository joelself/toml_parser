use std::fmt;
use std::fmt::Display;
use std::option::Option;
use nom::IResult;

/// Compares two Options that contain comparable structs
///
/// # Examples
///
/// ```
/// # use tomllib::ast::structs::comp_opt;
/// let (a, b) = (Some("value"), Some("value"));
/// assert!(comp_opt(&a, &b));
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
	pub expr: Expression<'a>,
	pub nl_exprs: Vec<NLExpression<'a>>,
}

impl<'a> PartialEq for Toml<'a> {
	fn eq(&self, other: &Toml<'a>) -> bool {
		self.expr == other.expr &&
		self.nl_exprs == other.nl_exprs
	}
}

impl<'a> Display for Toml<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    	let j = self.nl_exprs.len();
    	if j == 0 {
    		write!(f, "{}", self.expr)
    	} else {
    		try!(write!(f, "{}", self.expr));
	    	for i in 0..j-1 {
	    		try!(write!(f, "{}", self.nl_exprs[i]));
	    	}
    		write!(f, "{}", self.nl_exprs[j-1])
    	}
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

#[derive(Debug, Eq)]
pub enum Val<'a> {
	Integer(&'a str),
	Float(&'a str),
	Boolean(&'a str),
	DateTime(DateTime<'a>),
	Array(Box<Array<'a>>),
	String(&'a str),
	InlineTable(Box<InlineTable<'a>>),
}

impl<'a> PartialEq for Val<'a> {
	fn eq(&self, other: &Val<'a>) -> bool {
		match (self, other) {
			(&Val::Integer(ref i), &Val::Integer(ref j)) if i == j => true,
			(&Val::Float(ref i), &Val::Float(ref j)) if i == j => true,
			(&Val::Boolean(ref i), &Val::Boolean(ref j)) if i == j => true,
			(&Val::DateTime(ref i), &Val::DateTime(ref j)) if i == j => true,
			(&Val::Array(ref i), &Val::Array(ref j)) if i == j => true,
			(&Val::String(ref i), &Val::String(ref j)) if i == j => true,
			(&Val::InlineTable(ref i), &Val::InlineTable(ref j)) if i == j => true,
			_ => false
		}
	}
}

impl<'a> Display for Val<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			&Val::Integer(ref i) => write!(f, "{}", i),
			&Val::Float(ref i) => write!(f, "{}", i),
			&Val::Boolean(ref i) => write!(f, "{}", i),
			&Val::DateTime(ref i) => write!(f, "{}", i),
			&Val::Array(ref i) => write!(f, "{}", i),
			&Val::String(ref i) => write!(f, "{}", i),
			&Val::InlineTable(ref i) => write!(f, "{}", i),
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

#[derive(Debug, Eq)]
pub enum TimeOffset<'a> {
	Z,
	Time(TimeOffsetAmount<'a>),
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

#[derive(Debug, Eq)]
pub enum PosNeg {
	Pos,
	Neg,
}

impl PartialEq for PosNeg {
	fn eq(&self, other: &PosNeg) -> bool {
		match (self, other) {
			(&PosNeg::Pos, &PosNeg::Pos) => true,
			(&PosNeg::Neg, &PosNeg::Neg) => true,
			_ => false
		}
	}
}

impl Display for PosNeg {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    	match self {
    		&PosNeg::Pos => write!(f, "+"),
    		&PosNeg::Neg => write!(f, "-"),
    	}
    }
}

// #<text>
#[derive(Debug, Eq)]
pub struct Comment<'a> {
	pub text: &'a str,
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
	pub val: Val<'a>,
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
pub struct PartialTime<'a> {
    pub hour: &'a str,
	pub minute: &'a str,
	pub second: &'a str,
	pub fraction: &'a str,
}

impl<'a> PartialEq for PartialTime<'a> {
	fn eq(&self, other: &PartialTime<'a>) -> bool {
		self.hour == other.hour &&
		self.minute == other.minute &&
		self.second == other.second &&
		self.fraction == other.fraction
	}
}

impl<'a> Display for PartialTime<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    	if self.fraction == "" {
    		write!(f, "{}:{}:{}", self.hour, self.minute, self.second)
    	} else {
    		write!(f, "{}:{}:{}.{}", self.hour, self.minute, self.second, self.fraction)
    	}
    }
}

// (+|-)<hour>:<minute>
#[derive(Debug, Eq)]
pub struct TimeOffsetAmount<'a> {
	pub pos_neg: PosNeg,
	pub hour: &'a str,
	pub minute: &'a str,
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

// <partial_time><time_offset>
#[derive(Debug, Eq)]
pub struct FullTime<'a> {
    pub partial_time: PartialTime<'a>,
    pub time_offset: TimeOffset<'a>,
}

impl<'a> PartialEq for FullTime<'a> {
	fn eq(&self, other: &FullTime<'a>) -> bool {
		self.partial_time == other.partial_time &&
		self.time_offset == other.time_offset
	}
}

impl<'a> Display for FullTime<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    	write!(f, "{}{}", self.partial_time, self.time_offset)
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

// <date>T<time>
#[derive(Debug, Eq)]
pub struct DateTime<'a> {
	pub date: FullDate<'a>,
	pub time: FullTime<'a>,
}

impl<'a> PartialEq for DateTime<'a> {
	fn eq(&self, other: &DateTime<'a>) -> bool {
		self.date == other.date &&
		self.time == other.time
	}
}

impl<'a> Display for DateTime<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    	write!(f, "{}T{}", self.date, self.time)
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
pub struct ArrayValues<'a> {
	pub val: Val<'a>,
	pub array_sep: Option<WSSep<'a>>,
	pub comment_nl: Option<CommentOrNewLines<'a>>,
	pub array_vals: Option<Box<ArrayValues<'a>>>,
}

impl<'a> PartialEq for ArrayValues<'a> {
	fn eq(&self, other: &ArrayValues<'a>) -> bool {
		self.val == other.val &&
		comp_opt(&self.array_sep, &other.array_sep) &&
		comp_opt(&self.comment_nl, &other.comment_nl) &&
		comp_opt(&self.array_vals, &other.array_vals)
	}
}

impl<'a> Display for ArrayValues<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    	match(&self.array_sep, &self.comment_nl, &self.array_vals) {
    		(&Some(ref s), &None, &None) => write!(f, "{}{},{}", self.val, s.ws1, s.ws2),
    		(&Some(ref s), &Some(ref c), &None) => write!(f, "{}{},{}{}", self.val, s.ws1, s.ws2, c),
    		(&None, &Some(ref c), &None) => write!(f, "{}{}", self.val, c),
    		(&None, &None, &None) => write!(f, "{}", self.val),
    		(&Some(ref s), &None, &Some(ref a)) => write!(f, "{}{},{}{}", self.val, s.ws1, s.ws2, a),
    		(&Some(ref s), &Some(ref c), &Some(ref a)) => write!(f, "{}{},{}{}{}", self.val, s.ws1, s.ws2, c, a),
    		_ => panic!("Invalid ArrayValues: val: {}, array_sep {:?}, comment_nl: {:?}, array_vals: {:?}",
    			self.val, self.array_sep, self.comment_nl, self.array_vals),
    	}
    }
}

// [<ws.ws1><values?><ws.ws2>]
#[derive(Debug, Eq)]
pub struct Array<'a> {
	pub values: Option<ArrayValues<'a>>,
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
    	match &self.values {
    		&Some(ref a) => write!(f, "[{}{}{}]", self.ws.ws1, a, self.ws.ws2),
    		&None => write!(f, "[{}{}]", self.ws.ws1, self.ws.ws2),
    	}
    }
}

// <key><keyval_sep.ws1>=<keyval_sep.ws2><val><<table_sep.ws1>,<table_sep.ws2>?><keyvals?>
#[derive(Debug, Eq)]
pub struct TableKeyVals<'a> {
	pub key: &'a str,
	pub keyval_sep: WSSep<'a>,
	pub val: Val<'a>,
	pub table_sep: Option<WSSep<'a>>,
	pub keyvals: Option<Box<TableKeyVals<'a>>>,
}

impl<'a> PartialEq for TableKeyVals<'a> {
	fn eq(&self, other: &TableKeyVals<'a>) -> bool {
		self.key == other.key &&
		self.keyval_sep == other.keyval_sep &&
		self.val == other.val &&
		comp_opt(&self.table_sep, &other.table_sep) && 
		comp_opt(&self.keyvals, &other.keyvals)
	}
}

impl<'a> Display for TableKeyVals<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    	match (&self.table_sep, &self.keyvals) {
    		(&Some(ref t), &Some(ref k)) => write!(f, "{}{}={}{}{},{}{}",
    			self.key, self.keyval_sep.ws1, self.keyval_sep.ws2, self.val,
    			t.ws1, t.ws2, k),
    		(&None, &None) => write!(f, "{}{}={}{}",
    			self.key, self.keyval_sep.ws1, self.keyval_sep.ws2, self.val),
    		_ => panic!("Invalid TableKeyVals: key: {}, keyval_sep.ws1: {},
    			keyval_sep.ws2: {}, val: {}, table_sep: {:?}, keyvals: {:?}",
    			self.key, self.keyval_sep.ws1, self.keyval_sep.ws2, self.val,
    			self.table_sep, self.keyvals),
    	}
    }
}

// {<ws.ws1><keyvals><ws.ws2>}
#[derive(Debug, Eq)]
pub struct InlineTable<'a> {
	pub keyvals: TableKeyVals<'a>,
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
    	write!(f, "{}{}{}", self.ws.ws1, self.keyvals, self.ws.ws2)
    }
}


