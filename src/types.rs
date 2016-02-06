use ast::structs::Value;
use std::collections::HashMap;
use std::hash::Hasher;
use std::rc::Rc;
use std::cell::RefCell;


pub enum ParseResult<'a> {
	Full,
	Partial(&'a str),
	FullError(Vec<ParseError<'a>>),
	PartialError(&'a str, Vec<ParseError<'a>>),
	Failure(u64, u64),
}

#[derive(Debug, Eq, Clone, Copy)]
pub enum TimeOffset<'a> {
	Z,
	Time(TimeOffsetAmount<'a>),
}
 
// (+|-)<hour>:<minute>
#[derive(Debug, Eq, Clone, Copy)]
pub struct TimeOffsetAmount<'a> {
	pub pos_neg: &'a str,
	pub hour: &'a str,
	pub minute: &'a str,
}

#[derive(Debug, Eq, Clone, Copy)]
pub struct DateTime<'a> {
	pub year: &'a str,
	pub month: &'a str,
	pub day: &'a str,
	pub hour: &'a str,
	pub minute: &'a str,
	pub second: &'a str,
	pub fraction: Option<&'a str>,
	pub offset: TimeOffset<'a>,
}

pub enum ParseError<'a> {
	MixedArray(Vec<Rc<Value<'a>>>),
	DuplicateKey(String, Rc<Value<'a>>),
	DuplicateArrayOfTableKey(String, String, Rc<Value<'a>>),
	InvalidTable(String, RefCell<HashMap<String, Rc<Value<'a>>>>)
}