use ast::structs::Value;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::rc::Rc;
use std::cell::RefCell;
use std::fmt;
use std::fmt::Display;
use std::ops::{Index, IndexMut, Range};


pub enum ParseResult<'a> {
	Full,
	Partial(&'a str),
	FullError(Vec<ParseError<'a>>),
	PartialError(&'a str, Vec<ParseError<'a>>),
	Failure(u64, u64),
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum StrType {
	Basic,
	MLBasic,
	Literal,
	MLLiteral,
}

#[derive(Debug, PartialEq, Eq)]
pub enum TOMLValue<'a> {
	Integer(&'a str),
	Float(&'a str),
	Boolean(&'a str),
	DateTime(DateTime<'a>),
	Array(Rc<Vec<TOMLValue<'a>>>),
	String(&'a str, StrType),
}

impl<'a> Display for TOMLValue<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			&TOMLValue::Integer(v) | &TOMLValue::Float(v) | &TOMLValue::Boolean(v) =>
				write!(f, "{}", v),
			&TOMLValue::DateTime(ref v) => write!(f, "{}", v),
			&TOMLValue::Array(ref arr) => {
				try!(write!(f, "["));
				for i in 0..arr.len() - 1 {
					try!(write!(f, "{}, ", arr[i]));
				}
				if arr.len() > 0 {
					try!(write!(f, "{}", arr[arr.len()-1]));
				}
				write!(f, "]")
			},
			&TOMLValue::String(s, ref t) => {
				match t {
					&StrType::Basic => write!(f, "\"{}\"", s),
					&StrType::MLBasic => write!(f, "\"\"\"{}\"\"\"", s),
					&StrType::Literal => write!(f, "'{}'", s),
					&StrType::MLLiteral =>  write!(f, "'''{}'''", s),
				}
			}
		}
	}
}

#[derive(Debug, Eq, Clone)]
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

#[derive(Debug, Eq, Clone)]
pub enum Str<'a> {
	String(String),
	Str(&'a str)
}

impl<'a> Display for Str<'a> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
  	match self {
  		&Str::Str(s) => write!(f, "{}", s),
  		&Str::String(ref s) => write!(f, "{}", s),
  	}
  }
}

impl<'a> PartialEq for Str<'a> {
	fn eq(&self, other: &Str<'a>) -> bool {
		match (self, other) {
			(&Str::String(ref s), &Str::String(ref o)) => *s == *o,
			(&Str::Str(s), &Str::Str(o)) => s == o,
			(&Str::Str(ref s), &Str::String(ref o)) => *s == *o,
			(&Str::String(ref s), &Str::Str(ref o)) => *s == *o,
		}
	}
}
 
#[derive(Debug, Eq, Clone)]
pub enum TimeOffset<'a> {
	Z,
	Time(TimeOffsetAmount<'a>),
}

// (+|-)<hour>:<minute>
#[derive(Debug, Eq, Clone)]
pub struct TimeOffsetAmount<'a> {
	pub pos_neg: Str<'a>,
	pub hour: Str<'a>,
	pub minute: Str<'a>,
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
    	match self.fraction {
    		Some(frac) => write!(f, "{}-{}-{}T{}:{}:{}.{}{}",
						    		self.year, self.month, self.day,
						    		self.hour, self.minute, self.second, frac,
						    		self.offset),
    		None 		=> write!(f, "{}-{}-{}T{}:{}:{}{}",
						    		self.year, self.month, self.day,
						    		self.hour, self.minute, self.second,
						    		self.offset),
    	}
    }
}

pub enum ParseError<'a> {
	MixedArray(Vec<Rc<Value<'a>>>),
	DuplicateKey(String, Rc<Value<'a>>),
	DuplicateArrayOfTableKey(String, String, Rc<Value<'a>>),
	InvalidTable(String, RefCell<HashMap<String, Rc<Value<'a>>>>)
}