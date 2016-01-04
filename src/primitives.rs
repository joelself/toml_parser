use std::fmt;
use std::fmt::Display;
use ast::structs::{PartialTime, TimeOffsetAmount, TimeOffset, PosNeg,
                   FullTime, FullDate, DateTime, Val, KeyVal, WSSep};
use util::{ws};
use objects::{array, inline_table};
// Integer
named!(integer<&str, &str>, re_find_static!("^((\\+|-)?(([1-9](\\d|(_\\d))+)|\\d))")) ;

// Float
named!(float<&str, &str>,
       re_find_static!("^(\\+|-)?([1-9](\\d|(_\\d))+|\\d)((\\.\\d(\\d|(_\\d))*)((e|E)(\\+|-)?([1-9](\\d|(_\\d))+|\\d))|(\\.\\d(\\d|(_\\d))*)|((e|E)(\\+|-)?([1-9](\\d|(_\\d))+|\\d)))"));

// String
// TODO: named!(string<&str, &str>, alt!(basic_string | ml_basic_string | literal_string | ml_literal_string));

// Basic String
named!(basic_string<&str, &str>,
       re_find_static!("^\"( |!|[#-\\[]|[\\]-􏿿]|(\\\\\")|(\\\\)|(\\\\/)|(\\b)|(\\f)|(\\n)|(\\r)|(\\t)|(\\\\u[0-9A-Z]{4})|(\\\\U[0-9A-Z]{8})){0,}\""));

// Multiline Basic String
named!(ml_basic_string<&str, &str>,
       re_find_static!("^\"\"\"([ -\\[]|[\\]-􏿿]|(\\\\\")|(\\\\)|(\\\\/)|(\\b)|(\\f)|(\\n)|(\\r)|(\t)|(\\\\u[0-9A-Z]{4})|(\\\\U[0-9A-Z]{8})|\n|(\r\n)|(\\\\(\n|(\r\n))))*\"\"\""));

// Literal String
named!(literal_string<&str, &str>,
       re_find_static!("^'(	|[ -&]|[\\(-􏿿])*'"));

// Multiline Literal String
named!(ml_literal_string<&str, &str>, 
	   re_find_static!("^'''(	|[ -􏿿]|\n|(\r\n))*'''"));

named!(string<&str, &str>,
  alt!(
    complete!(ml_literal_string) |
    complete!(ml_basic_string)   |
    complete!(basic_string)      |
    complete!(literal_string)
  )
);

// Boolean
named!(boolean<&str, &str>, alt!(complete!(tag_s!("false")) | complete!(tag_s!("true"))));


// Datetime
// I use re_capture here because I only want the number without the dot. It captures the entire match
// in the 0th position and the first capture group in the 1st position
named!(fractional<&str, Vec<&str> >, re_capture_static!("^\\.([0-9]+)"));

named!(partial_time<&str, PartialTime>,
  chain!(
    hour: re_find_static!("^[0-9]{2}") ~
          tag_s!(":")                 ~
  minute: re_find_static!("^[0-9]{2}") ~
          tag_s!(":")                 ~
  second: re_find_static!("^[0-9]{2}") ~
 fraction: fractional?                ,
    ||{
      PartialTime{
        hour: hour, minute: minute, second: second, fraction: match fraction {
          Some(ref x) => x[1],
          None        => "",
        }
      }
    }
  )
);

named!(time_offset_amount<&str, TimeOffsetAmount>,
  chain!(
pos_neg: alt!(complete!(tag_s!("+")) => {|_| PosNeg::Pos} | complete!(tag_s!("-")) => {|_| PosNeg::Neg})  ~
   hour: re_find_static!("^[0-9]{2}")                                                                      ~
         tag_s!(":")                                                                                      ~
minute: re_find_static!("^[0-9]{2}")                                                                       ,
    ||{
      TimeOffsetAmount{
        pos_neg: pos_neg, hour: hour, minute: minute
      }
    }
  )
);

named!(time_offset<&str, TimeOffset>,
  alt!(
    complete!(tag_s!("Z"))        => {|_|       TimeOffset::Z} |
    complete!(time_offset_amount) => {|offset|  TimeOffset::Time(offset)}
  )
);

named!(full_time<&str, FullTime>,
  chain!(
partial: partial_time ~
 offset: time_offset,
    ||{
      FullTime{
        partial_time: partial, time_offset: offset
      }
    }
  )
);

named!(full_date<&str, FullDate>,
  chain!(
   year: re_find_static!("^([0-9]{4})") ~
         tag_s!("-") ~
  month: re_find_static!("^([0-9]{2})") ~
         tag_s!("-") ~
    day: re_find_static!("^([0-9]{2})"),
    ||{
      FullDate{
        year: year, month: month, day: day
      }
    }
  )
);

named!(date_time<&str, DateTime>,
  chain!(
   date: full_date  ~
         tag_s!("T") ~
   time: full_time  ,
    ||{
      DateTime{
        date: date, time: time
      }
    }
  )
);

// Key-Value pairs
fn is_keychar(chr: char) -> bool {
  let uchr = chr as u32;
  uchr >= 0x41 && uchr <= 0x5A || // A-Z
  uchr >= 0x61 && uchr <= 0x7A || // a-z
  uchr >= 0x30 && uchr <= 0x39 || // 0-9
  uchr == 0x2D || uchr == 0x5f    // "-", "_"
}

named!(unquoted_key<&str, &str>, take_while1_s!(is_keychar));
named!(quoted_key<&str, &str>, re_find_static!("^\"( |!|[#-\\[]|[\\]-􏿿]|(\\\\\")|(\\\\\\\\)|(\\\\/)|(\\\\b)|(\\\\f)|(\\\\n)|(\\\\r)|(\\\\t)|(\\\\u[0-9A-Z]{4})|(\\\\U[0-9A-Z]{8}))+\""));

named!(pub key<&str, &str>, alt!(complete!(quoted_key) | complete!(unquoted_key)));

named!(pub keyval_sep<&str, WSSep>,
  chain!(
    ws1: ws         ~
         tag_s!("=")~
    ws2: ws         ,
    ||{
      WSSep{
        ws1: ws1, ws2: ws2
      }
    }     
  )
);

named!(pub val<&str, Val>,
  alt!(
    complete!(array)        => {|arr|   Val::Array(Box::new(arr))}      |
    complete!(inline_table) => {|it|    Val::InlineTable(Box::new(it))} |
    complete!(date_time)    => {|dt|    Val::DateTime(dt)}              |
    complete!(float)        => {|flt|   Val::Float(flt)}                |
    complete!(integer)      => {|int|   Val::Integer(int)}              |
    complete!(boolean)      => {|b|     Val::Boolean(b)}                |
    complete!(string)       => {|s|     Val::String(s)}
  )
);

named!(pub keyval<&str, KeyVal>,
  chain!(
    key: key        ~
     ws: keyval_sep ~
    val: val        ,
    || {
      KeyVal{
        key: key, keyval_sep: ws, val: val
      }
    }
  )
);

#[cfg(test)]
mod test {
  use nom::IResult::Done;
  use ast::structs::{PartialTime, TimeOffsetAmount, PosNeg, TimeOffset,
                     FullTime, FullDate, DateTime, WSSep};
  use super::{boolean, partial_time, time_offset_amount, time_offset,
              full_time, full_date, date_time, literal_string,
              ml_literal_string, integer, fractional, float, basic_string,
              ml_basic_string, unquoted_key, quoted_key, key, keyval_sep};

  #[test]
  fn test_integer() {
    assert_eq!(integer("345_12_678"), Done("", "345_12_678"));
  }

  #[test]
  fn test_float() {
    assert_eq!(float("98_7.2_34e-8_8"), Done("", "98_7.2_34e-8_8"));
  }

  #[test]
  fn test_basic_string() {
    assert_eq!(basic_string("\"Tλïƨ ïƨ á βáƨïç ƨƭřïñϱ.\""), Done("", "\"Tλïƨ ïƨ á βáƨïç ƨƭřïñϱ.\""));
  }

  #[test]
  fn test_ml_basic_string() {
    assert_eq!(ml_basic_string(r#""""£ïñè Óñè
£ïñè Tωô
£ïñè Tλřèè""""#), Done("", r#""""£ïñè Óñè
£ïñè Tωô
£ïñè Tλřèè""""# ));
  }

  #[test]
  fn test_literal_string() {
    assert_eq!(literal_string("'Abc џ'"), Done("", "'Abc џ'")); 
  }

  #[test]
  fn test_ml_literal_string() {
    assert_eq!(ml_literal_string(r#"'''
                                    Abc џ
                                    '''"#),
      Done("", r#"'''
                                    Abc џ
                                    '''"#));
  }

  #[test]
  fn test_boolean() {
    assert_eq!(boolean("true"), Done("", "true"));
    assert_eq!(boolean("false"), Done("", "false"));
  }

  fn test_fractional() {
    assert_eq!(fractional(".03856"), Done("", vec![".03856", "03856"]));
  }

  #[test]
  fn test_partial_time() {
    assert_eq!(partial_time("11:22:33.456"),
      Done("", PartialTime{
        hour: "11",
        minute: "22",
        second: "33",
        fraction: "456"
      })
    );
    assert_eq!(partial_time("04:05:06"),
      Done("", PartialTime{
        hour: "04",
        minute: "05",
        second: "06",
        fraction: ""
      })
    );
  }

  #[test]
  fn test_time_offset_amount() {
    assert_eq!(time_offset_amount("+12:34"),
      Done("", TimeOffsetAmount{
        pos_neg: PosNeg::Pos,
        hour: "12",
        minute: "34"
      })
    );
  }

  #[test]
  fn test_time_offset() {
    assert_eq!(time_offset("+12:34"),
      Done("", TimeOffset::Time(TimeOffsetAmount{
        pos_neg: PosNeg::Pos,
        hour: "12",
        minute: "34"
      }))
    );
    assert_eq!(time_offset("Z"), Done("", TimeOffset::Z));
  }

  #[test]
  fn test_full_time() {
    assert_eq!(full_time("10:30:55.83+12:54"),
      Done("", FullTime{
        partial_time: PartialTime{
          hour: "10",
          minute: "30",
          second: "55",
          fraction: "83"
        },
        time_offset: TimeOffset::Time(TimeOffsetAmount{
          pos_neg: PosNeg::Pos,
          hour: "12",
          minute: "54"
        })
      })
    );
  }

  #[test]
  fn test_full_date() {
    assert_eq!(full_date("1942-12-07"),
      Done("", FullDate{
        year: "1942", month: "12", day: "07"
      })
    );
  }

  #[test]
  fn test_date_time() {
    assert_eq!(date_time("1999-03-21T20:15:44.5-07:00"),
      Done("", DateTime{
        date: FullDate{
          year: "1999", month: "03", day: "21"
        },
        time: FullTime{
          partial_time: PartialTime{
            hour: "20",
            minute: "15",
            second: "44",
            fraction: "5"
          },
          time_offset: TimeOffset::Time(TimeOffsetAmount{
            pos_neg: PosNeg::Neg,
            hour: "07",
            minute: "00"
          })
        }
      })
    );
  }

  #[test]
  fn test_unquoted_key() {
    assert_eq!(unquoted_key("Un-Quoted_Key"), Done("", "Un-Quoted_Key"));
  }

  #[test]
  fn test_quoted_key() {
    assert_eq!(quoted_key("\"QúôƭèδKè¥\""), Done("", "\"QúôƭèδKè¥\""));
  }

  #[test]
  fn test_key() {
    assert_eq!(key("\"Gřáƥèƒřúïƭ\""), Done("", "\"Gřáƥèƒřúïƭ\""));
    assert_eq!(key("_is-key"), Done("", "_is-key"));
  }

  #[test]
  fn test_keyval_sep() {
    assert_eq!(keyval_sep("\t \t= \t"), Done("", WSSep{ws1: "\t \t", ws2: " \t"}));
  }
/*
    complete!(array)        => {|arr|   Val::Array(Box::new(arr))}      |
    complete!(inline_table) => {|it|    Val::InlineTable(Box::new(it))} |
    complete!(date_time)    => {|dt|    Val::DateTime(dt)}              |
    complete!(float)        => {|flt|   Val::Float(flt)}                |
    complete!(integer)      => {|int|   Val::Integer(int)}              |
    complete!(boolean)      => {|b|     Val::Boolean(b)}                |
    complete!(string)       => {|s|     Val::String(s)}


  #[test]
  fn test_() {
    assert_eq!("", Done("", ));
  }

  #[test]
  fn test_() {
    assert_eq!("", Done("", ));
  }

  #[test]
  fn test_() {
    assert_eq!("", Done("", ));
  }*/
}