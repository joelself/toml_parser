use std::collections::{HashSet, HashMap};
use std::hash::Hasher;
use std::rc::Rc;
use std::cell::{Cell, RefCell};
use std::fmt;
use std::fmt::Display;
use std::str::FromStr;


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

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Children {
  Count(Cell<usize>),
  Keys(RefCell<HashSet<String>>)
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

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum PosNeg {
	Pos,
	Neg,
}

impl Display for PosNeg {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
  	match self {
  		&PosNeg::Pos => write!(f, "+"),
  		&PosNeg::Neg => write!(f, "-"),
  	}
  	
  }
}

#[derive(Debug, Eq, Clone)]
pub enum TimeOffset<'a> {
	Zulu,
	Time(TimeOffsetAmount<'a>),
}

impl<'a> PartialEq for TimeOffset<'a> {
	fn eq(&self, other: &TimeOffset<'a>) -> bool {
		match (self, other) {
			(&TimeOffset::Zulu, &TimeOffset::Zulu) => true,
			(&TimeOffset::Time(ref i), &TimeOffset::Time(ref j)) if(i == j) => true,
			_ => false
		}
	}
}

impl<'a> Display for TimeOffset<'a> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
  	match self {
  		&TimeOffset::Zulu => write!(f, "Z"),
  		&TimeOffset::Time(ref t) => write!(f, "{}", t),
  	}
  }
}

impl<'a> TimeOffset<'a> {
  pub fn validate_string(&self) -> bool {
    match self {
      &TimeOffset::Zulu => return true,
      &TimeOffset::Time(ref amount) => return amount.validate_string(),
    }
  }
  pub fn validate(&self) -> bool {
    match self {
      &TimeOffset::Zulu => return true,
      &TimeOffset::Time(ref amount) => return amount.validate(),
    }
  }
}

// (+|-)<hour>:<minute>
#[derive(Debug, Eq, Clone)]
pub struct TimeOffsetAmount<'a> {
	pub pos_neg: PosNeg,
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
  	let pn = match pos_neg {
  		"+" => PosNeg::Pos,
  		_		=> PosNeg::Neg,
  	};
  	TimeOffsetAmount{pos_neg: pn, hour: Str::Str(hour), minute: Str::Str(minute)}
  }
  pub fn new_string(pos_neg: String, hour: String, minute: String) -> TimeOffsetAmount<'a> {
  	let pos = String::from("+");
  	let mut pn = PosNeg::Neg;
  	if pos_neg == pos {
  		pn = PosNeg::Pos;
  	}
  	TimeOffsetAmount{pos_neg: pn, hour: Str::String(hour), minute: Str::String(minute)}
  }
  
  pub fn validate_string(&self) -> bool {
    if string!(self.hour).len() != 2 || string!(self.minute).len() != 2 {
      return false;
    }
    return self.validate();
  }
  
  pub fn validate(&self) -> bool {
    if let Ok(h) = usize::from_str(str!(self.hour)) {
      if h > 23 {
        return false;
      }
    } else {
      return false;
    }
    if let Ok(m) = usize::from_str(str!(self.minute)) {
      if m > 59 {
        return false;
      }
    } else {
      return false;
    }
    return true;
  }
}

// <year>-<month>-<day>
#[derive(Debug, Eq, Clone)]
pub struct Date<'a> {
	pub year: Str<'a>,
	pub month: Str<'a>,
	pub day: Str<'a>,
}

impl<'a> PartialEq for Date<'a> {
	fn eq(&self, other: &Date<'a>) -> bool {
		self.year == other.year &&
		self.month == other.month &&
		self.day == other.day
	}
}

impl<'a> Display for Date<'a> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
  	write!(f, "{}-{}-{}", self.year, self.month, self.day)
  }
}

impl<'a> Date<'a> {
  pub fn new_str(year: &'a str, month: &'a str, day: &'a str) -> Date<'a> {
  	Date{year: Str::Str(year), month: Str::Str(month), day: Str::Str(day)}
  }
  pub fn new_string(year: String, month: String, day: String) -> Date<'a> {
  	Date{year: Str::String(year), month: Str::String(month), day: Str::String(day)}
  }
  
  pub fn validate_string(&self) -> bool {
    if string!(self.year).len() != 4 || string!(self.month).len() != 2 || string!(self.day).len() != 2 {
      return false;
    }
    return self.validate();
  }
  
  pub fn validate(&self) -> bool {
    if let Ok(y) = usize::from_str(str!(self.year)) {
      if y == 0 {
        return false;
      }
      if let Ok(m) = usize::from_str(str!(self.month)) {
        if m < 1 || m > 12 {
          return false;
        }
        if let Ok(d) = usize::from_str(str!(self.day)) {
          if d < 1 {
            return false;
          }
          match d {
            2 => {
              let leap_year;
              if y % 4 != 0 {
                leap_year = false;
              } else if y % 100 != 0 {
                leap_year = true;
              } else if y % 400 != 0 {
                leap_year = false;
              } else {
                leap_year = true;
              }
              if leap_year && d > 29 {
                return false;
              } else if !leap_year && d > 28 {
                return false;
              }
            },
            1 | 3 | 5 | 7 | 8 | 10 | 12 => { if d > 31 { return false; } },
            _ => { if d > 30 { return false; } },
          }
        } else {
          return false;
        }
      } else {
        return false;
      }
    } else {
      return false;
    }
    return true;
  }
}

// <hour>:<minute>:<second>(.<fraction>)?
#[derive(Debug, Eq, Clone)]
pub struct Time<'a> {
  pub hour: Str<'a>,
	pub minute: Str<'a>,
	pub second: Str<'a>,
	pub fraction: Option<Str<'a>>,
	pub offset: Option<TimeOffset<'a>>,
}

impl<'a> PartialEq for Time<'a> {
	fn eq(&self, other: &Time<'a>) -> bool {
		self.hour == other.hour &&
		self.minute == other.minute &&
		self.second == other.second &&
		self.fraction == other.fraction &&
		self.offset == other.offset
	}
}

impl<'a> Display for Time<'a> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
  	match (&self.fraction, &self.offset) {
  		(&Some(ref frac), &Some(ref offset)) 	=> write!(f, "T{}:{}:{}.{}{}", self.hour, self.minute, self.second, frac, offset),
  		(&Some(ref frac), &None) 							=> write!(f, "T{}:{}:{}.{}", self.hour, self.minute, self.second, frac),
  		(&None, &Some(ref offset)) 						=> write!(f, "T{}:{}:{}{}", self.hour, self.minute, self.second, offset),
  		(&None, &None) 												=> write!(f, "T{}:{}:{}", self.hour, self.minute, self.second),
  	}
  }
}

impl<'a> Time<'a> {
  pub fn new_str(hour: &'a str, minute: &'a str, second: &'a str, fraction: Option<&'a str>, offset: Option<TimeOffset<'a>>)
  	-> Time<'a> {
  	if let Some(s) = fraction {
  		Time{hour: Str::Str(hour), minute: Str::Str(minute), second: Str::Str(second),
  			fraction: Some(Str::Str(s)), offset: offset}
  	} else {
  		Time{hour: Str::Str(hour), minute: Str::Str(minute), second: Str::Str(second),
  			fraction: None, offset: offset}
  	}
  }
  pub fn new_string(hour: String, minute: String, second: String, fraction: Option<String>, offset: Option<TimeOffset<'a>>)
  	-> Time<'a> {
  	if let Some(s) = fraction {
  		Time{hour: Str::String(hour), minute: Str::String(minute), second: Str::String(second),
  			fraction: Option::Some(Str::String(s)), offset: offset}
  	} else {
  		Time{hour: Str::String(hour), minute: Str::String(minute), second: Str::String(second),
  			fraction: None, offset: offset}
  	}
  }
  
  pub fn validate_string(&self) -> bool {
    if string!(self.hour).len() != 2 || string!(self.minute).len() != 2 || string!(self.second).len() != 2 {
      return false;
    }
    return self.validate(true); 
  }
  
  pub fn validate(&self, validate_string: bool) -> bool {
    if let Ok(h) = usize::from_str(str!(self.hour)) {
      if h > 23 {
        return false;
      }
    } else {
      return false;
    }
    if let Ok(m) = usize::from_str(str!(self.minute)) {
      if m > 59 {
        return false;
      }
    } else {
      return false;
    }
    if let Ok(s) = usize::from_str(str!(self.second)) {
      if s > 59 {
        return false;
      }
    } else {
      return false;
    }
    if let Some(ref frac) = self.fraction {
      if usize::from_str(str_ref!(frac)).is_err() {
        return false;
      }
    }
    if let Some(ref off) = self.offset {
      if validate_string && !off.validate_string() || !off.validate() {
        return false;
      }
    }
    return true;
  }
}

#[derive(Debug, Eq, Clone)]
pub struct DateTime<'a> {
	pub date: Date<'a>,
	pub time: Option<Time<'a>>,
}

impl<'a> PartialEq for DateTime<'a> {
	fn eq(&self, other: &DateTime<'a>) -> bool {
		self.date == other.date &&
		self.time == other.time
	}
}

impl<'a> Display for DateTime<'a> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
  	match &self.time {
  		&Some(ref time) => write!(f, "{}{}", self.date, time),
  		&None => write!(f, "{}", self.date),
  	}
  }
}

impl<'a> DateTime<'a> {
	pub fn new(date: Date<'a>, time: Option<Time<'a>>) -> DateTime<'a> {
		DateTime{date: date, time: time}
	}
  
  pub fn validate(&self, validate_string: bool) -> bool {
    if validate_string && self.date.validate_string() || self.date.validate() {
      if let Some(ref time) = self.time {
        return validate_string && !time.validate_string() || time.validate(false);
      }
    } else {
      return false;
    }
    return true;
  }
}

pub enum ParseError<'a> {
	MixedArray(String),
	DuplicateKey(String, TOMLValue<'a>),
	InvalidTable(String, RefCell<HashMap<String, TOMLValue<'a>>>),
  InvalidDateTime(String)
}