use std::collections::HashMap;
use std::hash::Hasher;
use std::rc::Rc;
use std::cell::{Cell, RefCell};
use std::fmt;
use std::error::Error;
use std::fmt::Display;
use std::str::FromStr;
use std::borrow::Cow;
use parser::Parser;
use nom::IResult;


pub enum ParseResult<'a> {
	Full,
	Partial(Cow<'a, str>),
	FullError(&'a RefCell<Vec<ParseError<'a>>>),
	PartialError(Cow<'a, str>, &'a RefCell<Vec<ParseError<'a>>>),
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
  Keys(RefCell<Vec<String>>)
}

impl Children {
  pub fn combine_keys<S>(base_key: S, child_key: S) -> String where S: Into<String> {
    let mut full_key;
    let base = base_key.into();
    let child = child_key.into();
    if base != "" {
      full_key = base.clone();
      full_key.push('.');
      full_key.push_str(&child);
    } else {
      full_key = child.clone();
    }
    return full_key;
  }
  pub fn combine_keys_index<S>(base_key: S, child_key: usize) -> String where S: Into<String> {
    return format!("{}[{}]", base_key.into(), child_key);
  }
  pub fn combine_all_child_keys<S>(base_key: S, child_keys: Children) -> Vec<String> where S: Into<String> {
    let mut all_keys = vec![];
    let base = base_key.into();
    match child_keys {
      Children::Count(c) => {
        for i in 0..c.get() {
          all_keys.push(format!("{}[{}]", base, i));
        }
      },
      Children::Keys(hs_rc) => {
        for subkey in hs_rc.borrow().iter() {
          if base != "" {
            let mut full_key = base.clone();
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
	Integer(Cow<'a, str>),
	Float(Cow<'a, str>),
	Boolean(Bool),
	DateTime(DateTime<'a>),
	Array(Rc<Vec<TOMLValue<'a>>>),
	String(Cow<'a, str>, StrType),
	InlineTable(Rc<Vec<(Cow<'a, str>, TOMLValue<'a>)>>)
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
    TOMLValue::Integer(format!("{}", int).into())
  }
  pub fn int_from_str<S>(int: S) -> Result<TOMLValue<'a>, TOMLError> where S: Into<String> + Clone {
    let result = TOMLValue::Integer(int.clone().into().into());
    if result.validate() {
      return Result::Ok(result);
    } else {
      return Result::Err(TOMLError::new(format!("Error parsing int. Argument: {}", int.into())));
    }
  }
  pub fn float(float: f64) -> TOMLValue<'a> {
    TOMLValue::Float(format!("{}", float).into())
  }
  pub fn float_from_str<S>(float: S) -> Result<TOMLValue<'a>, TOMLError> where S: Into<String> + Clone {
    let result = TOMLValue::Float(float.clone().into().into());
    if result.validate() {
      return Result::Ok(result);
    } else {
      return Result::Err(TOMLError::new(format!("Error parsing float. Argument: {}", float.into())));
    }
  }
  pub fn bool(b: bool) -> TOMLValue<'a> {
    if b {
      TOMLValue::Boolean(Bool::True)
    } else {
      TOMLValue::Boolean(Bool::False)
    }
  }
  pub fn bool_from_str<S>(b: S) -> Result<TOMLValue<'a>, TOMLError> where S: Into<String> + Clone {
    let lower = b.clone().into().to_lowercase();
    if lower == "true" {
      Result::Ok(TOMLValue::Boolean(Bool::True))
    } else if lower == "false" {
      Result::Ok(TOMLValue::Boolean(Bool::False))
    } else {
      return Result::Err(TOMLError::new(
        format!("Error parsing bool. Argument: {}", b.into())
      ))
    }
  }
  
  pub fn date_from_int(year: usize, month: usize, day: usize) -> Result<TOMLValue<'a>, TOMLError> {
    let y = format!("{:0>4}", year);
    let m = format!("{:0>2}", month);
    let d = format!("{:0>2}", day);
    let datetime = TOMLValue::DateTime(DateTime::new(Date::from_str(y, m, d), None));
    if datetime.validate() {
      return Result::Ok(datetime);
    } else {
      return Result::Err(TOMLError::new(
        format!("Error parsing int date. Arguments: {}, {}, {}", year, month, day)
      ));
    }
  }
  pub fn date_from_str<S>(year: S, month: S, day: S) -> Result<TOMLValue<'a>, TOMLError> where S: Into<String> + Clone {
    let datetime = TOMLValue::DateTime(DateTime::new(Date::from_str(year.clone().into(), month.clone().into(), day.clone().into()), None));
    if datetime.validate() {
      return Result::Ok(datetime);
    } else {
      return Result::Err(TOMLError::new(
        format!("Error parsing &str date. Arguments: {}, {}, {}", year.into(), month.into(), day.into())
      ));
    }
  }
  pub fn datetime_from_int(year: usize, month: usize, day: usize, hour: usize, minute: usize, second: usize) -> Result<TOMLValue<'a>, TOMLError> {
    let y = format!("{:0>4}", year);
    let m = format!("{:0>2}", month);
    let d = format!("{:0>2}", day);
    let h = format!("{:0>2}", hour);
    let min = format!("{:0>2}", minute);
    let s = format!("{:0>2}", second);
    let datetime = TOMLValue::DateTime(DateTime::new(Date::from_str(y, m, d), Some(
      Time::from_str(h, min, s, None, None)
    )));
    if datetime.validate() {
      return Result::Ok(datetime);
    } else {
      return Result::Err(TOMLError::new(
        format!("Error parsing int datetime. Arguments: {}, {}, {}, {}, {}, {}", year, month, day, hour, minute, second)
      ));
    }
  }
  pub fn datetime_from_str<S>(year: S, month: S, day: S, hour: S, minute: S, second: S) -> Result<TOMLValue<'a>, TOMLError> where S: Into<String> + Clone {
    let datetime = TOMLValue::DateTime(DateTime::new(Date::from_str(year.clone().into(), month.clone().into(), day.clone().into()), Some(
      Time::from_str(hour.clone().into(), minute.clone().into(), second.clone().into(), None, None)
    )));
    if datetime.validate() {
      return Result::Ok(datetime);
    } else {
      return Result::Err(TOMLError::new(
        format!("Error parsing &str datetime. Arguments: {}, {}, {}, {}, {}, {}", year.into(), month.into(), day.into(), hour.into(), minute.into(), second.into())
      ));
    }
  }

  pub fn datetime_frac_from_int(year: usize, month: usize, day: usize, hour: usize, minute: usize, second: usize, frac: usize) -> Result<TOMLValue<'a>, TOMLError> {  
    let y = format!("{:0>4}", year);
    let m = format!("{:0>2}", month);
    let d = format!("{:0>2}", day);
    let h = format!("{:0>2}", hour);
    let min = format!("{:0>2}", minute);
    let s = format!("{:0>2}", second);
    let f = format!("{}", frac);
    let datetime = TOMLValue::DateTime(DateTime::new(Date::from_str(y, m, d), Some(
      Time::from_str(h, min, s, Some(f), None)
    )));
    if datetime.validate() {
      return Result::Ok(datetime);
    } else {
      return Result::Err(TOMLError::new(
        format!("Error parsing int datetime_frac. Arguments: {}, {}, {}, {}, {}, {}, {}", year, month, day, hour, minute, second, frac)
      ));
    }
  }
  pub fn datetime_frac_from_str<S>(year: S, month: S, day: S, hour: S, minute: S, second: S, frac: S) -> Result<TOMLValue<'a>, TOMLError> where S: Into<String> + Clone{ 
    let datetime = TOMLValue::DateTime(DateTime::new(Date::from_str(year.clone().into(), month.clone().into(), day.clone().into()), Some(
      Time::from_str(hour.clone().into(), minute.clone().into(), second.clone().into(), Some(frac.clone().into()), None)
    )));
    if datetime.validate() {
      return Result::Ok(datetime);
    } else {
      return Result::Err(TOMLError::new(
        format!("Error parsing &str datetime_frac. Arguments: {}, {}, {}, {}, {}, {}, {}", year.into(), month.into(), day.into(), hour.into(), minute.into(), second.into(), frac.into())
      ));
    }
  }
  pub fn datetime_offset_from_int<S>(year: usize, month: usize, day: usize, hour: usize, minute: usize, second: usize, posneg: S, off_hour: usize, off_minute: usize) -> Result<TOMLValue<'a>, TOMLError> where S: Into<String> + Clone{  
    let y = format!("{:0>4}", year);
    let m = format!("{:0>2}", month);
    let d = format!("{:0>2}", day);
    let h = format!("{:0>2}", hour);
    let min = format!("{:0>2}", minute);
    let s = format!("{:0>2}", second);
    let oh = format!("{:0>2}", off_hour);
    let omin = format!("{:0>2}", off_minute);
    let datetime = TOMLValue::DateTime(DateTime::new(Date::from_str(y, m, d), Some(
      Time::from_str(h, min, s, None, Some(
        TimeOffset::Time(TimeOffsetAmount::from_str(posneg.clone().into(), oh, omin))
      ))
    )));
    if datetime.validate() {
      return Result::Ok(datetime);
    } else {
      return Result::Err(TOMLError::new(
        format!("Error parsing int datetime_offset. Arguments: {}, {}, {}, {}, {}, {}, {}, {}, {}", year, month, day, hour, minute, second, posneg.into(), off_hour, off_minute)
      ));
    }
  }
  pub fn datetime_offset_from_str<S>(year: S, month: S, day: S, hour: S, minute: S, second: S, posneg: S, off_hour: S, off_minute: S) -> Result<TOMLValue<'a>, TOMLError> where S: Into<String> + Clone{ 
    let datetime = TOMLValue::DateTime(DateTime::new(Date::from_str(year.clone().into(), month.clone().into(), day.clone().into()), Some(
      Time::from_str(hour.clone().into(), minute.clone().into(), second.clone().into(), None, Some(
        TimeOffset::Time(TimeOffsetAmount::from_str(posneg.clone().into(), off_hour.clone().into(), off_minute.clone().into()))
      ))
    )));
    if datetime.validate() {
      return Result::Ok(datetime);
    } else {
      return Result::Err(TOMLError::new(
        format!("Error parsing &str datetime_offset. Arguments: {}, {}, {}, {}, {}, {}, {}, {}, {}", year.into(), month.into(), day.into(), hour.into(), minute.into(), second.into(), posneg.into(), off_hour.into(), off_minute.into())
      ));
    }
  }
  pub fn datetime_zulu_from_int(year: usize, month: usize, day: usize, hour: usize, minute: usize, second: usize) -> Result<TOMLValue<'a>, TOMLError> {  
    let y = format!("{:0>4}", year);
    let m = format!("{:0>2}", month);
    let d = format!("{:0>2}", day);
    let h = format!("{:0>2}", hour);
    let min = format!("{:0>2}", minute);
    let s = format!("{:0>2}", second);
    let datetime = TOMLValue::DateTime(DateTime::new(Date::from_str(y, m, d), Some(
      Time::from_str(h, min, s, None, Some(
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
  pub fn datetime_zulu_from_str<S>(year: S, month: S, day: S, hour: S, minute: S, second: S) -> Result<TOMLValue<'a>, TOMLError> where S: Into<String> + Clone{ 
    let datetime = TOMLValue::DateTime(DateTime::new(Date::from_str(year.clone().into(), month.clone().into(), day.clone().into()), Some(
      Time::from_str(hour.clone().into(), minute.clone().into(), second.clone().into(), None, Some(
        TimeOffset::Zulu
      ))
    )));
    if datetime.validate() {
      return Result::Ok(datetime);
    } else {
      return Result::Err(TOMLError::new(
        format!("Error parsing &str datetime_zulu. Arguments: {}, {}, {}, {}, {}, {}", year.into(), month.into(), day.into(), hour.into(), minute.into(), second.into())
      ));
    }
  }
  pub fn datetime_full_from_int<S>(year: usize, month: usize, day: usize, hour: usize, minute: usize, second: usize, frac: usize, posneg: S, off_hour: usize, off_minute: usize) -> Result<TOMLValue<'a>, TOMLError> where S: Into<String> + Clone {  
    let y = format!("{:0>4}", year);
    let m = format!("{:0>2}", month);
    let d = format!("{:0>2}", day);
    let h = format!("{:0>2}", hour);
    let min = format!("{:0>2}", minute);
    let s = format!("{:0>2}", second);
    let f = format!("{}", frac);
    let oh = format!("{:0>2}", off_hour);
    let omin = format!("{:0>2}", off_minute);
    let datetime = TOMLValue::DateTime(DateTime::new(Date::from_str(y, m, d), Some(
      Time::from_str(h, min, s, Some(f), Some(
        TimeOffset::Time(TimeOffsetAmount::from_str(posneg.clone().into(), oh, omin))
      ))
    )));
    if datetime.validate() {
      return Result::Ok(datetime);
    } else {
      return Result::Err(TOMLError::new(
        format!("Error parsing int datetime_full. Arguments: {}, {}, {}, {}, {}, {}, {}, {}, {}, {}", year, month, day, hour, minute, second, frac, posneg.into(), off_hour, off_minute)
      ));
    }
  }
  pub fn datetime_full_from_str<S>(year: S, month: S, day: S, hour: S, minute: S, second: S, frac: S, posneg: S, off_hour: S, off_minute: S) -> Result<TOMLValue<'a>, TOMLError> where S: Into<String> + Clone { 
    let datetime = TOMLValue::DateTime(DateTime::new(Date::from_str(year.clone().into(), month.clone().into(), day.clone().into()), Some(
      Time::from_str(hour.clone().into(), minute.clone().into(), second.clone().into(), Some(frac.clone().into()), Some(
        TimeOffset::Time(TimeOffsetAmount::from_str(posneg.clone().into(), off_hour.clone().into(), off_minute.clone().into()))
      ))
    )));
    if datetime.validate() {
      return Result::Ok(datetime);
    } else {
      return Result::Err(TOMLError::new(
        format!("Error parsing &str datetime_full. Arguments: {}, {}, {}, {}, {}, {}, {}, {}, {}, {}", year.into(), month.into(), day.into(), hour.into(), minute.into(), second.into(), frac.into(), posneg.into(), off_hour.into(), off_minute.into())
      ));
    }
  }
  pub fn datetime_parse<S>(dt: S) -> Result<TOMLValue<'a>, TOMLError> where S: Into<&'a str> {
    let datetime = dt.into();
    let p = Parser::new();
    match p.date_time(datetime) {
      (_, IResult::Done(_, o)) => {
        let result = TOMLValue::DateTime(o);
        if !result.validate() {
          return Result::Err(TOMLError::new(format!("Error parsing string as datetime. Argument: {}", datetime)));
        } else {
          return Result::Ok(result);
        }
      },
      (_,_) => return Result::Err(TOMLError::new(format!("Error parsing string as datetime. Argument: {}", datetime))),
    }
  }
  pub fn basic_string<S>(s: S) -> Result<TOMLValue<'a>, TOMLError> where S: Into<String> + Clone {
    let result = TOMLValue::String(s.clone().into().into(), StrType::Basic);
    if result.validate() {
      return Result::Ok(result);
    } else {
      return Result::Err(TOMLError::new(format!("Error parsing string as basic_string. Argument: {}", s.into())));
    }
  }
  pub fn ml_basic_string<S>(s: S) -> Result<TOMLValue<'a>, TOMLError> where S: Into<String> + Clone {
    let result = TOMLValue::String(s.clone().into().into(), StrType::MLBasic);
    if result.validate() {
      return Result::Ok(result);
    } else {
      return Result::Err(TOMLError::new(format!("Error parsing string as ml_basic_string. Argument: {}", s.into())));
    }
  }
  pub fn literal_string<S>(s: S) -> Result<TOMLValue<'a>, TOMLError> where S: Into<String> + Clone {
    let result = TOMLValue::String(s.clone().into().into(), StrType::Literal);
    if result.validate() {
      return Result::Ok(result);
    } else {
      return Result::Err(TOMLError::new(format!("Error parsing string as literal_string. Argument: {}", s.into())));
    }
  }
  pub fn ml_literal_string<S>(s: S) -> Result<TOMLValue<'a>, TOMLError> where S: Into<String> + Clone {
    let result = TOMLValue::String(s.clone().into().into(), StrType::MLLiteral);
    if result.validate() {
      return Result::Ok(result);
    } else {
      return Result::Err(TOMLError::new(format!("Error parsing string as ml_literal_string. Argument: {}", s.into())));
    }
  }
  
  pub fn validate(&self) -> bool{
    match self {
      &TOMLValue::Integer(ref s) => {
        let p = Parser::new();
        match p.integer(s) {
           (_, IResult::Done(_, _)) => true,
           (_,_) => false,
        }
      },
      &TOMLValue::Float(ref s) => {
        let p = Parser::new();
        match p.float(s) {
           (_, IResult::Done(_, _)) => true,
           (_,_) => false,
        }
      },
      &TOMLValue::DateTime(ref dt) => {dt.validate()},
      &TOMLValue::String(ref s, st) => {
        match st {
          StrType::Basic => {
            match Parser::quoteless_basic_string(s) {
              IResult::Done(_,_) => true,
              _ => false,
            }
          },
          StrType::MLBasic => {
            match Parser::quoteless_ml_basic_string(s) {
              IResult::Done(_,_) => true,
              _ => false,
            }
          },
          StrType::Literal => {
            match Parser::quoteless_literal_string(s) {
              IResult::Done(_,_) => true,
              _ => false,
            }
          },
          StrType::MLLiteral => {
            match Parser::quoteless_ml_literal_string(s) {
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
	pub hour: Cow<'a, str>,
	pub minute: Cow<'a, str>,
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
  	write!(f, "{}{}:{}", self.pos_neg, &self.hour, &self.minute)
  }
}

impl<'a> TimeOffsetAmount<'a> {
  pub fn from_str<S>(pos_neg: S, hour: S, minute: S) -> TimeOffsetAmount<'a> where S: Into<String>{
  	let pn = match pos_neg.into().as_ref() {
  		"+" => PosNeg::Pos,
  		"-"	=> PosNeg::Neg,
      _   => {error!("PosNeg value is neither a '+' or a '-', defaulting to '+'."); PosNeg::Pos},
  	};
  	TimeOffsetAmount{pos_neg: pn, hour: hour.into().into(), minute: minute.into().into()}
  }
  
  pub fn validate(&self) -> bool {
    if self.hour.len() != 2 || self.minute.len() != 2 {
      return false;
    }
    return self.validate_numbers();
   }
   
  pub fn validate_numbers(&self) -> bool {
    if let Ok(h) = usize::from_str(&self.hour) {
      if h > 23 {
        return false;
      }
    } else {
      return false;
    }
    if let Ok(m) = usize::from_str(&self.minute) {
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
	pub year: Cow<'a, str>,
	pub month: Cow<'a, str>,
	pub day: Cow<'a, str>,
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
  pub fn from_str<S>(year: S, month: S, day: S) -> Date<'a> where S: Into<String> {
  	Date{year: year.into().into(), month: month.into().into(), day: day.into().into()}
  }
  
  pub fn validate(&self) -> bool {
    if self.year.len() != 4 || self.month.len() != 2 || self.day.len() != 2 {
      return false;
    }
    return self.validate_numbers();
  }
  
  pub fn validate_numbers(&self) -> bool {
    if let Ok(y) = usize::from_str(&self.year) {
      if y == 0 {
        return false;
      }
      if let Ok(m) = usize::from_str(&self.month) {
        if m < 1 || m > 12 {
          return false;
        }
        if let Ok(d) = usize::from_str(&self.day) {
          if d < 1 {
            return false;
          }
          match m {
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
  pub hour: Cow<'a, str>,
	pub minute: Cow<'a, str>,
	pub second: Cow<'a, str>,
	pub fraction: Option<Cow<'a, str>>,
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
  pub fn from_str<S>(hour: S, minute: S, second: S, fraction: Option<S>, offset: Option<TimeOffset<'a>>) 
  	-> Time<'a> where S: Into<String> {
  	if let Some(s) = fraction {
  		Time{hour: hour.into().into(), minute: minute.into().into(), second: second.into().into(),
  			fraction: Some(s.into().into()), offset: offset}
  	} else {
  		Time{hour: hour.into().into(), minute: minute.into().into(), second: second.into().into(),
  			fraction: None, offset: offset}
    }
  }
  
  pub fn validate(&self) -> bool {
    if self.hour.len() != 2 || self.minute.len() != 2 || self.second.len() != 2 {
      return false;
    }
    return self.validate_numbers(); 
  }
  
  pub fn validate_numbers(&self) -> bool {
    if let Ok(h) = usize::from_str(&self.hour) {
      if h > 23 {
        return false;
      }
    } else {
      return false;
    }
    if let Ok(m) = usize::from_str(&self.minute) {
      if m > 59 {
        return false;
      }
    } else {
      return false;
    }
    if let Ok(s) = usize::from_str(&self.second) {
      if s > 59 {
        return false;
      }
    } else {
      return false;
    }
    if let Some(ref frac) = self.fraction {
      if usize::from_str(frac).is_err() {
        return false;
      }
    }
    if let Some(ref off) = self.offset {
      if !off.validate() {
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
  
  pub fn validate(&self) -> bool {
    if self.date.validate() {
      if let Some(ref time) = self.time {
        return time.validate();
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