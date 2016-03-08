use std::collections::{HashSet, HashMap};
use std::hash::Hasher;
use std::rc::Rc;
use std::cell::{Cell, RefCell};
use std::fmt;
use std::error::Error;
use std::fmt::Display;
use std::str::FromStr;
use parser::Parser;
use nom::IResult;


pub enum ParseResult<'a> {
	Full,
	Partial(Str<'a>),
	FullError(&'a RefCell<Vec<ParseError<'a>>>),
	PartialError(Str<'a>, &'a RefCell<Vec<ParseError<'a>>>),
	Failure(usize, usize),
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

impl Children {
  pub fn combine_keys_string(base_key: String, child_key: String) -> String {
    let mut full_key;
    if base_key != "" {
      full_key = base_key.clone();
      full_key.push('.');
      full_key.push_str(&child_key);
    } else {
      full_key = child_key.clone();
    }
    return full_key;
  }
  pub fn combine_keys_int(base_key: String, child_key: usize) -> String {
    return format!("{}[{}]", base_key, child_key);
  }
  pub fn combine_all_keys(base_key: String, child_keys: Children) -> Vec<String> {
    let mut all_keys = vec![];
    match child_keys {
      Children::Count(c) => {
        for i in 0..c.get() {
          all_keys.push(format!("{}[{}]", base_key, i));
        }
      },
      Children::Keys(hs_rc) => {
        for subkey in hs_rc.borrow().iter() {
          if base_key != "" {
            let mut full_key = base_key.clone();
            full_key.push('.');
            full_key.push_str(&subkey);
            all_keys.push(full_key);
          } else {
            all_keys.push(subkey.clone());
          }
        }
      },
    }
    return all_keys;
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

impl<'a> TOMLValue<'a> {
  pub fn int(int: i64) -> TOMLValue<'a> {
    TOMLValue::Integer(Str::String(format!("{}", int)))
  }
  pub fn int_str(int: &'a str) -> Result<TOMLValue<'a>, TOMLError> {
    let result = TOMLValue::Integer(Str::String(int.to_string()));
    if result.validate() {
      return Result::Ok(result);
    } else {
      return Result::Err(TOMLError::new(format!("Error parsing int. Argument: {}", int)));
    }
  }
  pub fn float(float: f64) -> TOMLValue<'a> {
    TOMLValue::Float(Str::String(format!("{}", float)))
  }
  pub fn float_str(float: &str) -> Result<TOMLValue<'a>, TOMLError> {
    let result = TOMLValue::Float(Str::String(float.to_string()));
    if result.validate() {
      return Result::Ok(result);
    } else {
      return Result::Err(TOMLError::new(format!("Error parsing float. Argument: {}", float)));
    }
  }
  pub fn bool(b: bool) -> TOMLValue<'a> {
    if b {
      TOMLValue::Boolean(Bool::True)
    } else {
      TOMLValue::Boolean(Bool::False)
    }
  }
  pub fn bool_str(b: &str) -> Result<TOMLValue<'a>, TOMLError> {
    let lower = b.to_lowercase();
    if lower == "true" {
      Result::Ok(TOMLValue::Boolean(Bool::True))
    } else if lower == "false" {
      Result::Ok(TOMLValue::Boolean(Bool::False))
    } else {
      return Result::Err(TOMLError::new(
        format!("Error parsing bool. Argument: {}", b)
      ))
    }
  }
  pub fn date_int(year: usize, month: usize, day: usize) -> Result<TOMLValue<'a>, TOMLError> {
    let y = format!("{:0>4}", year);
    let m = format!("{:0>2}", month);
    let d = format!("{:0>2}", day);
    let datetime = TOMLValue::DateTime(DateTime::new(Date::new_string(y, m, d), None));
    if datetime.validate() {
      return Result::Ok(datetime);
    } else {
      return Result::Err(TOMLError::new(
        format!("Error parsing int date. Arguments: {}, {}, {}", year, month, day)
      ));
    }
  }
  pub fn date_str(year: &str, month: &str, day: &str) -> Result<TOMLValue<'a>, TOMLError> {
    let datetime = TOMLValue::DateTime(DateTime::new(Date::new_string(year.to_string(), month.to_string(), day.to_string()), None));
    if datetime.validate() {
      return Result::Ok(datetime);
    } else {
      return Result::Err(TOMLError::new(
        format!("Error parsing &str date. Arguments: {}, {}, {}", year, month, day)
      ));
    }
  }
  pub fn datetime_int(year: usize, month: usize, day: usize, hour: usize, minute: usize, second: usize) -> Result<TOMLValue<'a>, TOMLError> {
    let y = format!("{:0>4}", year);
    let m = format!("{:0>2}", month);
    let d = format!("{:0>2}", day);
    let h = format!("{:0>2}", hour);
    let min = format!("{:0>2}", minute);
    let s = format!("{:0>2}", second);
    let datetime = TOMLValue::DateTime(DateTime::new(Date::new_string(y, m, d), Some(
      Time::new_string(h, min, s, None, None)
    )));
    if datetime.validate() {
      return Result::Ok(datetime);
    } else {
      return Result::Err(TOMLError::new(
        format!("Error parsing int datetime. Arguments: {}, {}, {}, {}, {}, {}", year, month, day, hour, minute, second)
      ));
    }
  }
  pub fn datetime_str(year: &str, month: &str, day: &str, hour: &str, minute: &str, second: &str) -> Result<TOMLValue<'a>, TOMLError> {
    let datetime = TOMLValue::DateTime(DateTime::new(Date::new_string(year.to_string(), month.to_string(), day.to_string()), Some(
      Time::new_string(hour.to_string(), minute.to_string(), second.to_string(), None, None)
    )));
    if datetime.validate() {
      return Result::Ok(datetime);
    } else {
      return Result::Err(TOMLError::new(
        format!("Error parsing &str datetime. Arguments: {}, {}, {}, {}, {}, {}", year, month, day, hour, minute, second)
      ));
    }
  }
  // ************ START HERE ************* convert DateTimes to TOMLValues and call validate on them.
  pub fn datetime_frac_int(year: usize, month: usize, day: usize, hour: usize, minute: usize, second: usize, frac: usize) -> Result<TOMLValue<'a>, TOMLError> {  
    let y = format!("{:0>4}", year);
    let m = format!("{:0>2}", month);
    let d = format!("{:0>2}", day);
    let h = format!("{:0>2}", hour);
    let min = format!("{:0>2}", minute);
    let s = format!("{:0>2}", second);
    let f = format!("{}", frac);
    let datetime = TOMLValue::DateTime(DateTime::new(Date::new_string(y, m, d), Some(
      Time::new_string(h, min, s, Some(f), None)
    )));
    if datetime.validate() {
      return Result::Ok(datetime);
    } else {
      return Result::Err(TOMLError::new(
        format!("Error parsing int datetime_frac. Arguments: {}, {}, {}, {}, {}, {}, {}", year, month, day, hour, minute, second, frac)
      ));
    }
  }
  pub fn datetime_frac_str(year: &str, month: &str, day: &str, hour: &str, minute: &str, second: &str, frac: &str) -> Result<TOMLValue<'a>, TOMLError> { 
    let datetime = TOMLValue::DateTime(DateTime::new(Date::new_string(year.to_string(), month.to_string(), day.to_string()), Some(
      Time::new_string(hour.to_string(), minute.to_string(), second.to_string(), Some(frac.to_string()), None)
    )));
    if datetime.validate() {
      return Result::Ok(datetime);
    } else {
      return Result::Err(TOMLError::new(
        format!("Error parsing &str datetime_frac. Arguments: {}, {}, {}, {}, {}, {}, {}", year, month, day, hour, minute, second, frac)
      ));
    }
  }
  pub fn datetime_offset_int(year: usize, month: usize, day: usize, hour: usize, minute: usize, second: usize, posneg: &str, off_hour: usize, off_minute: usize) -> Result<TOMLValue<'a>, TOMLError> {  
    let y = format!("{:0>4}", year);
    let m = format!("{:0>2}", month);
    let d = format!("{:0>2}", day);
    let h = format!("{:0>2}", hour);
    let min = format!("{:0>2}", minute);
    let s = format!("{:0>2}", second);
    let oh = format!("{:0>2}", off_hour);
    let omin = format!("{:0>2}", off_minute);
    let datetime = TOMLValue::DateTime(DateTime::new(Date::new_string(y, m, d), Some(
      Time::new_string(h, min, s, None, Some(
        TimeOffset::Time(TimeOffsetAmount::new_string(posneg.to_string(), oh.to_string(), omin.to_string()))
      ))
    )));
    if datetime.validate() {
      return Result::Ok(datetime);
    } else {
      return Result::Err(TOMLError::new(
        format!("Error parsing int datetime_offset. Arguments: {}, {}, {}, {}, {}, {}, {}, {}, {}", year, month, day, hour, minute, second, posneg, off_hour, off_minute)
      ));
    }
  }
  pub fn datetime_offset_str(year: &str, month: &str, day: &str, hour: &str, minute: &str, second: &str, posneg: &str, off_hour: &str, off_minute: &str) -> Result<TOMLValue<'a>, TOMLError> { 
    let datetime = TOMLValue::DateTime(DateTime::new(Date::new_string(year.to_string(), month.to_string(), day.to_string()), Some(
      Time::new_string(hour.to_string(), minute.to_string(), second.to_string(), None, Some(
        TimeOffset::Time(TimeOffsetAmount::new_string(posneg.to_string(), off_hour.to_string(), off_minute.to_string()))
      ))
    )));
    if datetime.validate() {
      return Result::Ok(datetime);
    } else {
      return Result::Err(TOMLError::new(
        format!("Error parsing &str datetime_offset. Arguments: {}, {}, {}, {}, {}, {}, {}, {}, {}", year, month, day, hour, minute, second, posneg, off_hour, off_minute)
      ));
    }
  }
  pub fn datetime_zulu_int(year: usize, month: usize, day: usize, hour: usize, minute: usize, second: usize) -> Result<TOMLValue<'a>, TOMLError> {  
    let y = format!("{:0>4}", year);
    let m = format!("{:0>2}", month);
    let d = format!("{:0>2}", day);
    let h = format!("{:0>2}", hour);
    let min = format!("{:0>2}", minute);
    let s = format!("{:0>2}", second);
    let datetime = TOMLValue::DateTime(DateTime::new(Date::new_string(y, m, d), Some(
      Time::new_string(h, min, s, None, Some(
        TimeOffset::Zulu
      ))
    )));
    if datetime.validate() {
      return Result::Ok(datetime);
    } else {
      return Result::Err(TOMLError::new(
        format!("Error parsing int datetime_zulu. Arguments: {}, {}, {}, {}, {}, {}", year, month, day, hour, minute, second)
      ));
    }
  }
  pub fn datetime_zulu_str(year: &str, month: &str, day: &str, hour: &str, minute: &str, second: &str) -> Result<TOMLValue<'a>, TOMLError> { 
    let datetime = TOMLValue::DateTime(DateTime::new(Date::new_string(year.to_string(), month.to_string(), day.to_string()), Some(
      Time::new_string(hour.to_string(), minute.to_string(), second.to_string(), None, Some(
        TimeOffset::Zulu
      ))
    )));
    if datetime.validate() {
      return Result::Ok(datetime);
    } else {
      return Result::Err(TOMLError::new(
        format!("Error parsing &str datetime_zulu. Arguments: {}, {}, {}, {}, {}, {}", year, month, day, hour, minute, second)
      ));
    }
  }
  pub fn datetime_full_int(year: usize, month: usize, day: usize, hour: usize, minute: usize, second: usize, frac: usize, posneg: &str, off_hour: usize, off_minute: usize) -> Result<TOMLValue<'a>, TOMLError> {  
    let y = format!("{:0>4}", year);
    let m = format!("{:0>2}", month);
    let d = format!("{:0>2}", day);
    let h = format!("{:0>2}", hour);
    let min = format!("{:0>2}", minute);
    let s = format!("{:0>2}", second);
    let f = format!("{}", frac);
    let oh = format!("{:0>2}", off_hour);
    let omin = format!("{:0>2}", off_minute);
    let datetime = TOMLValue::DateTime(DateTime::new(Date::new_string(y, m, d), Some(
      Time::new_string(h, min, s, Some(f), Some(
        TimeOffset::Time(TimeOffsetAmount::new_string(posneg.to_string(), oh.to_string(), omin.to_string()))
      ))
    )));
    if datetime.validate() {
      return Result::Ok(datetime);
    } else {
      return Result::Err(TOMLError::new(
        format!("Error parsing int datetime_full. Arguments: {}, {}, {}, {}, {}, {}, {}, {}, {}, {}", year, month, day, hour, minute, second, frac, posneg, off_hour, off_minute)
      ));
    }
  }
  pub fn datetime_full_str(year: &str, month: &str, day: &str, hour: &str, minute: &str, second: &str, frac: &str, posneg: &str, off_hour: &str, off_minute: &str) -> Result<TOMLValue<'a>, TOMLError> { 
    let datetime = TOMLValue::DateTime(DateTime::new(Date::new_string(year.to_string(), month.to_string(), day.to_string()), Some(
      Time::new_string(hour.to_string(), minute.to_string(), second.to_string(), Some(frac.to_string()), Some(
        TimeOffset::Time(TimeOffsetAmount::new_string(posneg.to_string(), off_hour.to_string(), off_minute.to_string()))
      ))
    )));
    if datetime.validate() {
      return Result::Ok(datetime);
    } else {
      return Result::Err(TOMLError::new(
        format!("Error parsing &str datetime_full. Arguments: {}, {}, {}, {}, {}, {}, {}, {}, {}, {}", year, month, day, hour, minute, second, frac, posneg, off_hour, off_minute)
      ));
    }
  }
  pub fn datetime_parse(dt: &'a str) -> Result<TOMLValue<'a>, TOMLError> {
    let p = Parser::new();
    match p.date_time(dt) {
      (_, IResult::Done(_, o)) => {
        let result = TOMLValue::DateTime(o);
        if !result.validate() {
          return Result::Err(TOMLError::new(format!("Error parsing &str as datetime. Argument: {}", dt)));
        } else {
          return Result::Ok(result);
        }
      },
      (_,_) => return Result::Err(TOMLError::new(format!("Error parsing &str as datetime. Argument: {}", dt))),
    }
  }
  pub fn basic_string(s: &str) -> Result<TOMLValue<'a>, TOMLError> {
    let result = TOMLValue::String(Str::String(s.to_string()), StrType::Basic);
    if result.validate() {
      return Result::Ok(result);
    } else {
      return Result::Err(TOMLError::new(format!("Error parsing &str as basic_string. Argument: {}", s)));
    }
  }
  pub fn ml_basic_string(s: &str) -> Result<TOMLValue<'a>, TOMLError> {
    let result = TOMLValue::String(Str::String(s.to_string()), StrType::MLBasic);
    if result.validate() {
      return Result::Ok(result);
    } else {
      return Result::Err(TOMLError::new(format!("Error parsing &str as ml_basic_string. Argument: {}", s)));
    }
  }
  pub fn literal_string(s: &str) -> Result<TOMLValue<'a>, TOMLError> {
    let result = TOMLValue::String(Str::String(s.to_string()), StrType::Literal);
    if result.validate() {
      return Result::Ok(result);
    } else {
      return Result::Err(TOMLError::new(format!("Error parsing &str as literal_string. Argument: {}", s)));
    }
  }
  pub fn ml_literal_string(s: &str) -> Result<TOMLValue<'a>, TOMLError> {
    let result = TOMLValue::String(Str::String(s.to_string()), StrType::Basic);
    if result.validate() {
      return Result::Ok(result);
    } else {
      let msg = format!("Error parsing &str as ml_literal_string. Argument: {}", s);
      error!("{}", msg);
      return Result::Err(TOMLError::new(format!("Error parsing &str as ml_literal_string. Argument: {}", s)));
    }
  }
  pub fn validate(&self) -> bool{
    match self {
      &TOMLValue::Integer(ref s) => {
        let p = Parser::new();
        match p.integer(str_ref!(s)) {
           (_, IResult::Done(_, _)) => true,
           (_,_) => false,
        }
      },
      &TOMLValue::Float(ref s) => {
        let p = Parser::new();
        match p.float(str_ref!(s)) {
           (_, IResult::Done(_, _)) => true,
           (_,_) => false,
        }
      },
      &TOMLValue::DateTime(ref dt) => {dt.validate(true)},
      &TOMLValue::String(ref s, st) => {
        match st {
          StrType::Basic => {
            match Parser::quoteless_basic_string(str_ref!(s)) {
              IResult::Done(_,_) => true,
              _ => false,
            }
          },
          StrType::MLBasic => {
            match Parser::quoteless_ml_basic_string(str_ref!(s)) {
              IResult::Done(_,_) => true,
              _ => false,
            }
          },
          StrType::Literal => {
            match Parser::quoteless_literal_string(str_ref!(s)) {
              IResult::Done(_,_) => true,
              _ => false,
            }
          },
          StrType::MLLiteral => {
            match Parser::quoteless_ml_literal_string(str_ref!(s)) {
              IResult::Done(_,_) => true,
              _ => false,
            }
          },
        }
      },
      _ => true,
    }
  }
}

#[derive(Debug)]
pub struct TOMLError {
  message: String,
}

impl Error for TOMLError {
    fn description(&self) -> &str {
      &self.message
    }
    fn cause(&self) -> Option<&Error> { None }
}

impl Display for TOMLError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.message)
  }
}

impl TOMLError {
  fn new(msg: String) -> TOMLError {
    error!("{}", msg);
    TOMLError{message: msg}
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
	MixedArray(String, usize),
	DuplicateKey(String, usize, TOMLValue<'a>),
	InvalidTable(String, usize, RefCell<HashMap<String, TOMLValue<'a>>>),
  InvalidDateTime(String, usize)
}