#![allow(dead_code)]
#![feature(plugin)]
#![plugin(regex_macros)]
mod ast;
use ast::{Comment, WSSep, WSKeySep, TableType, Table, PartialTime,
          TimeOffsetAmount, TimeOffset, FullTime, PosNeg, FullDate,
          DateTime};
#[macro_use]
extern crate nom;
extern crate regex;
// TOML
// TODO: toml
// TODO: expression

// Newline
named!(newline<&str, &str>, alt!(complete!(tag_s!("\r\n")) | complete!(tag_s!("\n"))));

named!(newlines<&str, Vec<&str> >, many1!(newline));

// Whitespace
fn is_space(chr: char) -> bool {
    chr as u32 == 0x20 || chr as u32 == 0x09
}

named!(ws<&str, &str>, take_while_s!(is_space));

// Comment
fn not_eol(chr: char) -> bool {
  chr as u32 == 0x09 || (chr as u32 >= 0x20 && chr as u32 <=0x10FFF)
}

named!(comment<&str, Comment>,
       chain!(
              tag_s!("#") ~
 comment_txt: take_while_s!(not_eol),
              || {Comment{text: comment_txt}}
             )
      );

// Key-Value pairs
fn is_keychar(chr: char) -> bool {
  let uchr = chr as u32;
  uchr >= 0x41 && uchr <= 0x5A || // A-Z
  uchr >= 0x61 && uchr <= 0x7A || // A-Z
  uchr >= 0x30 && uchr <= 0x39 || // 0-9
  uchr == 0x2D || uchr == 0x5f // "-", "_"
}


// named!(basic_unescaped<&str, &str>, re_match!(r" |!|[#-\[]|[\]-􏿿]"));
// named!(escaped<&str, &str>, re_match!("(\\\\\")|(\\\\)|(\\\\/)|(\\b)|(\\f)|(\\n)|(\\r)|(\\t)|(u[0-9A-Z]{4})|(U[0-9A-Z]{8})"));
// named!(basic_char<&str, &str>, alt!(basic_unescaped | escaped));
named!(unquoted_key<&str, &str>, take_while1_s!(is_keychar));
named!(quoted_key<&str, &str>, re_find_static!("\"( |!|[#-\\[]|[\\]-􏿿]|(\\\\\")|(\\\\)|(\\\\/)|(\\b)|(\\f)|(\\n)|(\\r)|(\\t)|(\\\\u[0-9A-Z]{4})|(\\\\U[0-9A-Z]{8})){1,}\""));

named!(key<&str, &str>, alt!(unquoted_key | quoted_key));
named!(keyval_sep<&str, WSSep>,
       chain!(
         ws1: ws ~
              tag_s!("=") ~
         ws2: ws,
              || {WSSep{ws1: ws1, ws2: ws2}}     
             )
      );
// TODO: named!(val<&str, &str>, ...);
// TODO: named!(keyval<&str, ast::KeyVal>, ...);

// Standard Table
named!(table_sub_key<&str, WSKeySep>,
       chain!(
         ws1: ws ~
              tag_s!(".") ~
         ws2: ws ~
         key: key,
              || {WSKeySep{ws: WSSep{ws1: ws1, ws2: ws2}, key: key}} 
             )
      );
named!(table_sub_keys<&str, Vec<WSKeySep> >, many0!(table_sub_key));
named!(std_table<&str, Table>,
       chain!(
              tag_s!("[") ~
         ws1: ws ~
         key: key ~
     subkeys: table_sub_keys ~
         ws2: ws ~
              tag_s!("]"),
              || {Table{ttype: TableType::Standard,
                             ws: WSSep{ws1: ws1, ws2: ws2},
                             key: key, subkeys: subkeys}}
             )
      );

// Array Table
named!(array_table<&str, Table>,
       chain!(
              tag_s!("[[") ~
         ws1: ws ~
         key: key ~
     subkeys: table_sub_keys ~
         ws2: ws ~
              tag_s!("]]"),
              || {Table{ttype: TableType::Array,
                             ws: WSSep{ws1: ws1, ws2: ws2},
                             key: key, subkeys: subkeys}}
             )
      );

// Integer
named!(integer<&str, &str>, re_find_static!("(\\+|-)?([1-9](\\d|(_\\d))+|\\d)"));

// Float
named!(float<&str, &str>,
       re_find_static!("(\\+|-)?([1-9](\\d|(_\\d))+|\\d)((\\.\\d(\\d|(_\\d))*)((e|E)(\\+|-)?([1-9](\\d|(_\\d))+|\\d))|(\\.\\d(\\d|(_\\d))*)|((e|E)(\\+|-)?([1-9](\\d|(_\\d))+|\\d)))"));

// String
// TODO: named!(string<&str, &str>, alt!(basic_string | ml_basic_string | literal_string | ml_literal_string));

// Basic String
named!(basic_string<&str, &str>,
       re_find_static!("\"( |!|[#-\\[]|[\\]-􏿿]|(\\\\\")|(\\\\)|(\\\\/)|(\\b)|(\\f)|(\\n)|(\\r)|(\\t)|(\\\\u[0-9A-Z]{4})|(\\\\U[0-9A-Z]{8})){0,}\""));

// Multiline Basic String
named!(ml_basic_string<&str, &str>,
       re_find_static!("\"\"\"([ -\\[]|[\\]-􏿿]|(\\\\\")|(\\\\)|(\\\\/)|(\\b)|(\\f)|(\\n)|(\\r)|(\\t)|(\\\\u[0-9A-Z]{4})|(\\\\U[0-9A-Z]{8})|\n|(\r\n)|(\\\\(\n|(\r\n))))*\"\"\""));

// Literal String
named!(literal_string<&str, &str>,
       re_find_static!("'(	|[ -&]|[\\(-􏿿])*'"));

// Multiline Literal String
named!(ml_literal_string<&str, &str>, 
	   re_find_static!("'''(	|[ -􏿿]|\n|(\r\n))*'''"));

// Boolean
named!(boolean<&str, &str>, alt!(complete!(tag_s!("false")) | complete!(tag_s!("true"))));


// Datetime
named!(fractional<&str, Vec<&str> >, re_capture_static!(r".([0-9]+)"));
named!(partial_time<&str, PartialTime>,
       chain!(
        hour: re_find_static!("[0-9]{2}") ~
              tag_s!(":") ~
      minute: re_find_static!("[0-9]{2}") ~
              tag_s!(":") ~
      second: re_find_static!("[0-9]{2}") ~
    fraction: fractional ? ,
              || {PartialTime{hour: hour,
                              minute: minute,
                              second: second,
                              fraction: match fraction {
                                                        Some(ref x) => x[1],
                                                        None => "",
                                                       }
                             }
                 }
             )
      );
named!(time_offset_amount<&str, TimeOffsetAmount>,
       chain!(
     pos_neg: alt!(tag_s!("+") => {|_| PosNeg::Pos} | tag_s!("-") => {|_| PosNeg::Neg}) ~
        hour: re_find_static!("[0-9]{2}") ~
              tag_s!(":") ~
      minute: re_find_static!("[0-9]{2}"),
              || {TimeOffsetAmount{pos_neg: pos_neg, hour: hour, minute: minute}}
             )
      );
named!(time_offset<&str, TimeOffset>,
       alt!( tag_s!("Z") => {|_| TimeOffset::Z} |
             time_offset_amount => {|offset| TimeOffset::Time(offset)}
           )
      );
named!(full_time<&str, FullTime>,
       chain!(
     partial: partial_time ~
      offset: time_offset,
              || {FullTime{partial_time: partial,
                           time_offset: offset}}
             )
      );
named!(full_date<&str, FullDate>,
       chain!(
        year: re_find_static!("[0-9]{4}") ~
              tag_s!("-") ~
       month: re_find_static!("[0-9]{2}") ~
              tag_s!("-") ~
         day: re_find_static!("[0-9]{2}"),
              || {FullDate{year: year, month: month, day: day}}
             )
      );
named!(date_time<&str, DateTime>,
       chain!(
        date: full_date ~
        time: full_time,
              || {DateTime{date: date, time: time}}
             )
      );

// For testing as I go
// TODO: remove when finished
fn main() {
    let s = "11:22:33.456+09:13";
    let r = full_time(s);
    println!("{:?}", r);
}

#[cfg(test)]
mod test {
	use nom::IResult::{Done};
	use ::{literal_string, ml_literal_string, boolean, partial_time,
         time_offset_amount, time_offset, full_time, full_date,
         date_time};
  use ast::{PartialTime, TimeOffsetAmount, TimeOffset, FullTime, PosNeg,
            FullDate, DateTime};

	#[test]
	fn test_literal_string() {
		assert_eq!(literal_string("'Abc џ'"), Done("", "'Abc џ'"));	
	}

  #[test]
  fn test_ml_literal_string() {
    assert_eq!(ml_literal_string(r#"'''
                                    Abc џ
                                    '''"#), Done("", r#"'''
                                    Abc џ
                                    '''"#));
  }

  #[test]
  fn test_boolean() {
    assert_eq!(boolean("true"), Done("", "true"));
    assert_eq!(boolean("false"), Done("", "false"));
  }

  #[test]
  fn test_partial_time() {
    assert_eq!(partial_time("11:22:33.456"), Done("", PartialTime{hour: "11",
                                                                  minute: "22",
                                                                  second: "33",
                                                                  fraction: "456"}));
    assert_eq!(partial_time("04:05:06"), Done("", PartialTime{hour: "04",
                                                                  minute: "05",
                                                                  second: "06",
                                                                  fraction: ""}));
  }

  #[test]
  fn test_time_offset_amount() {
    assert_eq!(time_offset_amount("+12:34"), Done("", TimeOffsetAmount{pos_neg: PosNeg::Pos,
                                                                       hour: "12",
                                                                       minute: "34"}));
  }

  #[test]
  fn test_time_offset() {
    assert_eq!(time_offset("+12:34"), Done("",
                                           TimeOffset::Time(TimeOffsetAmount{pos_neg: PosNeg::Pos,
                                                                             hour: "12",
                                                                             minute: "34"})));
    assert_eq!(time_offset("Z"), Done("", TimeOffset::Z));
  }

  #[test]
  fn test_full_time() {
    assert_eq!(full_time("10:30:55.83+12:54"), Done("",
                                                    FullTime{partial_time:
                                                             PartialTime{hour: "10",
                                                                         minute: "30",
                                                                         second: "55",
                                                                         fraction: "83"},
                                                             time_offset:
                                                             TimeOffset::Time(TimeOffsetAmount{pos_neg: PosNeg::Pos,
                                                                                               hour: "12",
                                                                                               minute: "54"})}));
  }

  #[test]
  fn test_full_date() {
    assert_eq!(full_date("1942-12-07"), Done("", FullDate{year: "1942", month: "12", day: "07"}));
  }

  #[test]
  fn test_date_time() {
    assert_eq!(date_time("1999-03-21T20:15:44.5-07:00"),
               Done("", DateTime{date: FullDate{year: "1999", month: "03", day: "21"},
                                 time: FullTime{partial_time:
                                                 PartialTime{hour: "20",
                                                             minute: "15",
                                                             second: "44",
                                                             fraction: "5"},
                                                 time_offset:
                                                 TimeOffset::Time(TimeOffsetAmount{pos_neg: PosNeg::Neg,
                                                                                   hour: "07",
                                                                                   minute: "00"})}}));
  }
}