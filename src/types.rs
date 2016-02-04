use ast::structs::{Array, InlineTable};
use ast::structs::Value;
use std::hash::{Hash, Hasher};
use std::rc::Rc;
use std::cell::RefCell;


pub enum ParseResult<'a> {
	Success(&'a str),
	SuccessComplete(&'a str, &'a str),
	Error(Vec<ParseError<'a>>),
	Failure,
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
	DuplicateKeyName(String, Rc<Value<'a>>),
}