use ast::structs::Value;
use std::collections::HashMap;
use std::hash::Hasher;
use std::rc::Rc;
use std::cell::RefCell;
use std::fmt;
use std::fmt::Display;


pub enum ParseResult<'a> {
	Full,
	Partial(Str<'a>),
	FullError(Vec<ParseError<'a>>),
	PartialError(Str<'a>, Vec<ParseError<'a>>),
	Failure(u64, u64),
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum StrType {
	Basic,
	MLBasic,
	Literal,
	MLLiteral,
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum Bool {
	False,
	True,
}

impl Display for Bool {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		if let &Bool::False = self {
			write!(f, "false")
		} else {
			write!(f, "true")
		}
	}
}

#[derive(Debug, PartialEq, Eq)]
pub enum TOMLValue<'a> {
	Integer(Str<'a>),
	Float(Str<'a>),
	Boolean(Bool),
	DateTime(DateTime<'a>),
	Array(Rc<Vec<TOMLValue<'a>>>),
	String(Str<'a>, StrType),
	InlineTable(Rc<Vec<(Str<'a>, TOMLValue<'a>)>>)
}

impl<'a> Display for TOMLValue<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			&TOMLValue::Integer(ref v) | &TOMLValue::Float(ref v) =>
				write!(f, "{}", v),
			&TOMLValue::Boolean(ref b) => write!(f, "{}", b),
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
			&TOMLValue::String(ref s, ref t) => {
				match t {
					&StrType::Basic => write!(f, "\"{}\"", s),
					&StrType::MLBasic => write!(f, "\"\"\"{}\"\"\"", s),
					&StrType::Literal => write!(f, "'{}'", s),
					&StrType::MLLiteral =>  write!(f, "'''{}'''", s),
				}
			},
			&TOMLValue::InlineTable(ref it) => {
				try!(write!(f, "{{"));
				for i in 0..it.len() - 1 {
					try!(write!(f, "{} = {}, ", it[i].0, it[i].1));
				}
				if it.len() > 0 {
					try!(write!(f, "{} = {}", it[it.len()-1].0, it[it.len()-1].1));
				}
				write!(f, "}}")
			}
		}
	}
}

#[derive(Debug, Eq, Clone)]
pub struct DateTime<'a> {
	pub year: Str<'a>,
	pub month: Str<'a>,
	pub day: Str<'a>,
	pub hour: Str<'a>,
	pub minute: Str<'a>,
	pub second: Str<'a>,
	pub fraction: Option<Str<'a>>,
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

impl<'a> TimeOffsetAmount<'a> {
  pub fn new_str(pos_neg: &'a str, hour: &'a str, minute: &'a str) -> TimeOffsetAmount<'a> {
  	TimeOffsetAmount{pos_neg: Str::Str(pos_neg), hour: Str::Str(hour), minute: Str::Str(minute)}
  }
  pub fn new_string(pos_neg: String, hour: String, minute: String) -> TimeOffsetAmount<'a> {
  	TimeOffsetAmount{pos_neg: Str::String(pos_neg), hour: Str::String(hour), minute: Str::String(minute)}
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
  		Some(ref frac) => write!(f, "{}-{}-{}T{}:{}:{}.{}{}",
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

impl<'a> DateTime<'a> {
	pub fn new(year: Str<'a>, month: Str<'a>, day: Str<'a>, hour: Str<'a>, minute: Str<'a>,
		second: Str<'a>, fraction: Option<Str<'a>>, offset: TimeOffset<'a>) -> DateTime<'a> {
		DateTime{year: year, month: month, day: day, hour: hour, minute: minute, second: second,
			fraction: fraction, offset: offset}
	}
	pub fn new_str(year: &'a str, month: &'a str, day: &'a str, hour: &'a str, minute: &'a str,
		second: &'a str, fraction: Option<&'a str>, offset: TimeOffset<'a>) -> DateTime<'a> {
		match fraction {
			Some(f) => DateTime{year: Str::Str(year), month: Str::Str(month), day: Str::Str(day),
				hour: Str::Str(hour), minute: Str::Str(minute), second: Str::Str(second),
				fraction: Some(Str::Str(f)), offset: offset},
			None 		=> DateTime{year: Str::Str(year), month: Str::Str(month), day: Str::Str(day),
				hour: Str::Str(hour), minute: Str::Str(minute), second: Str::Str(second),
				fraction: None, offset: offset},
		}
	}
	pub fn new_string(year: String, month: String, day: String, hour: String, minute: String,
		second: String, fraction: Option<String>, offset: TimeOffset<'a>) -> DateTime<'a> {
		match fraction {
			Some(f) => DateTime{year: Str::String(year), month: Str::String(month), day: Str::String(day),
				hour: Str::String(hour), minute: Str::String(minute), second: Str::String(second),
				fraction: Some(Str::String(f)), offset: offset},
			None 		=> DateTime{year: Str::String(year), month: Str::String(month), day: Str::String(day),
				hour: Str::String(hour), minute: Str::String(minute), second: Str::String(second),
				fraction: None, offset: offset},
		}
	}
}

pub enum ParseError<'a> {
	MixedArray(Vec<Rc<RefCell<Value<'a>>>>),
	DuplicateKey(String, Rc<RefCell<Value<'a>>>),
	DuplicateArrayOfTableKey(String, String, Rc<Value<'a>>),
	InvalidTable(String, RefCell<HashMap<String, Rc<RefCell<Value<'a>>>>>)
}