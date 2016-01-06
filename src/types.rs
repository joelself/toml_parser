use ast::structs::{Array, InlineTable};
pub enum ParseResult<'a> {
	Success(&'a str),
	SuccessComplete(&'a str, &'a str),
	Error(Vec<ParseError<'a>>),
	Failure,
}

#[derive(Debug, Eq)]
pub enum Value<'a> {
	Integer(&'a str),
	Float(&'a str),
	Boolean(&'a str),
	DateTime(DateTime<'a>),
	Array(Box<Array<'a>>),
	String(&'a str),
	InlineTable(Box<InlineTable<'a>>),
}


#[derive(Debug, Eq)]
pub enum TimeOffset<'a> {
	Z,
	Time(TimeOffsetAmount<'a>),
}
 
// (+|-)<hour>:<minute>
#[derive(Debug, Eq)]
pub struct TimeOffsetAmount<'a> {
	pub pos_neg: &'a str,
	pub hour: &'a str,
	pub minute: &'a str,
}

#[derive(Debug, Eq)]
pub struct DateTime<'a> {
	pub year: &'a str,
	pub month: &'a str,
	pub day: &'a str,
	pub hour: &'a str,
	pub minute: &'a str,
	pub second: &'a str,
	pub fraction: &'a str,
	pub offset: TimeOffset<'a>,
}

pub enum ParseError<'a> {
	MixedArray(Array<'a>),
	DuplicateTableName(String, Box<Fn(&String)>),
}