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
			/*(&Val::InlineTable(ref i), &Val::InlineTable(ref j)) if i == j => true,*/
			_ => false
		}
	}
}
#[derive(Debug, Eq)]
pub enum TableType {
	Standard,
	Array,
}

impl PartialEq for TableType {
	fn eq(&self, other: &TableType) -> bool {
		match (self, other) {
			(&TableType::Standard, &TableType::Standard) => true,
			(&TableType::Array, &TableType::Array) => true,
			_ => false
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

// <ws.ws1>.<ws.ws2><key>
#[derive(Debug)]
pub struct WSKeySep<'a> {
	pub ws: WSSep<'a>,
	pub key: &'a str,
}

// Standard: [<ws.ws1><key><subkeys*><ws.ws2>]
// Array: [[<ws.ws1><key><subkeys*><ws.ws2>]]
#[derive(Debug)]
pub struct Table<'a> {
    pub ttype: TableType,
	pub ws: WSSep<'a>, // opening whitespace and closing whitespace
	pub key: &'a str,
	pub subkeys: Vec<WSKeySep<'a>>,
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

// <comment><newlines+>
#[derive(Debug, Eq)]
pub struct CommentNewLines<'a> {
	pub comment: Comment<'a>,
	pub newlines: Vec<&'a str>,
}

impl<'a> PartialEq for CommentNewLines<'a> {
	fn eq(&self, other: &CommentNewLines<'a>) -> bool {
		self.comment == other.comment &&
		self.newlines == other.newlines
	}
}

#[derive(Debug, Eq)]
pub enum CommentOrNewLines<'a> {
	Comment(CommentNewLines<'a>),
	NewLines(Vec<&'a str>),
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
		match (&self.array_sep, &other.array_sep) {
			(&Some(ref i), &Some(ref j)) if i == j => true,
			(&None, &None) => true,
			_ => false
		} &&
		match (&self.comment_nl, &other.comment_nl) {
			(&Some(ref i), &Some(ref j)) if i == j => true,
			(&None, &None) => true,
			_ => false
		} &&
		match (&self.array_vals, &other.array_vals) {
			(&Some(ref i), &Some(ref j)) if i == j => true,
			(&None, &None) => true,
			_ => false
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
		match (&self.table_sep, &other.table_sep) {
			(&Some(ref i), &Some(ref j)) if i == j => true,
			(&None, &None) => true,
			_ => false
		} &&
		match (&self.keyvals, &other.keyvals) {
			(&Some(ref i), &Some(ref j)) if i == j => true,
			(&None, &None) => true,
			_ => false
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


