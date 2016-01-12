use std::collections::LinkedList;
use ast::structs::{Array, InlineTable};
use ast::structs::Value;
use std::hash::{Hash, Hasher};

#[derive(Eq)]
pub struct HashValue<'a> {
	value: Option<Value<'a>>, 
	subkeys: LinkedList<&'a str>,
}

impl<'a> HashValue<'a> {
	pub fn new(value: Value<'a>) -> HashValue<'a> {
		HashValue {
			value: Some(value),
			subkeys: LinkedList::new(),
		}
	}
}

impl<'a> PartialEq for HashValue<'a> {
	fn eq(&self, other: &HashValue<'a>) -> bool {
		self.value == other.value
	}
}

pub enum ParseResult<'a> {
	Success(&'a str),
	SuccessComplete(&'a str, &'a str),
	Error(Vec<ParseError<'a>>),
	Failure,
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