#![allow(dead_code)]
pub use ast::{Val, Comment, WSSep, KeyVal, WSKeySep, TableType, Table,
          PartialTime, TimeOffsetAmount, TimeOffset, FullTime, PosNeg,
          FullDate, DateTime, CommentNewLines, CommentOrNewLines,
          ArrayValues, Array, TableKeyVals, InlineTable};
#[macro_use]
// TOML
// TODO: toml
// TODO: expression

// Newline
named!(newline<&str, &str>,
  alt!(
    complete!(tag_s!("\r\n")) |
    complete!(tag_s!("\n"))
  )
);

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
    ||{
      Comment{
        text: comment_txt
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

named!(key<&str, &str>, alt!(complete!(quoted_key) | complete!(unquoted_key)));

named!(keyval_sep<&str, WSSep>,
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

named!(val<&str, Val>,
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

named!(keyval<&str, KeyVal>,
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

// Standard Table
named!(table_sub_key<&str, WSKeySep>,
  chain!(
    ws1: ws         ~
         tag_s!(".")~
    ws2: ws         ~
    key: key        ,
    ||{
      WSKeySep{
        ws: WSSep{
          ws1: ws1, ws2: ws2
        },
        key: key
      }
    } 
  )
);

named!(table_sub_keys<&str, Vec<WSKeySep> >, many0!(table_sub_key));

named!(std_table<&str, Table>,
  chain!(
         tag_s!("[")    ~
    ws1: ws             ~
    key: key            ~
subkeys: table_sub_keys ~
    ws2: ws             ~
         tag_s!("]")    ,
    ||{
      Table{
        ttype: TableType::Standard, ws: WSSep{
          ws1: ws1, ws2: ws2}, key: key, subkeys: subkeys
      }
    }
  )
);

// Array Table
named!(array_table<&str, Table>,
  chain!(
         tag_s!("[[")   ~
    ws1: ws             ~
    key: key            ~
subkeys: table_sub_keys ~
    ws2: ws             ~
         tag_s!("]]")   ,
    ||{
      Table{
        ttype: TableType::Array, ws: WSSep{
          ws1: ws1, ws2: ws2},key: key, subkeys: subkeys
      }
    }
  )
);

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
       re_find_static!("^\"\"\"([ -\\[]|[\\]-􏿿]|(\\\\\")|(\\\\)|(\\\\/)|(\\b)|(\\f)|(\\n)|(\\r)|(\\t)|(\\\\u[0-9A-Z]{4})|(\\\\U[0-9A-Z]{8})|\n|(\r\n)|(\\\\(\n|(\r\n))))*\"\"\""));

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
    ||{println!("parse full date");
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

// Array
named!(array_sep<&str, WSSep>,
  chain!(
    ws1: ws         ~
         tag_s!(",")~
    ws2: ws         ,
    ||{//println!("Parse array sep");
      WSSep{ws1: ws1, ws2: ws2
      }
    }
  )
);

named!(comment_nl<&str, CommentNewLines>,
  chain!(
 comment: comment ~
newlines: newlines,
    ||{
      CommentNewLines{
        comment: comment, newlines: newlines
      }
    }
  )
);

named!(comment_or_nl<&str, CommentOrNewLines>,
  alt!(
    complete!(comment_nl) => {|com| CommentOrNewLines::Comment(com)} |
    complete!(newlines)   => {|nl|  CommentOrNewLines::NewLines(nl)}
  )
);

named!(array_values<&str, ArrayValues>,
  alt!(
    complete!(
      chain!(
        val: val ~
  array_sep: array_sep ~
  comment_nl: comment_or_nl? ~
  array_vals: array_values,
        ||{
          ArrayValues{
            val: val,
            array_sep: Some(array_sep),
            comment_nl: comment_nl,
            array_vals: Some(Box::new(array_vals))
          }
        }
      )
    )|
    complete!(
      chain!(
        val: val              ~
  array_sep: array_sep?       ~
  comment_nl: comment_or_nl?  ,
        move ||{
          ArrayValues{
            val: val,
            array_sep: array_sep,
            comment_nl: comment_nl,
            array_vals: None
          }
        }
      )
    )
    |
    complete!(
      chain!(
        val: val                       ,
        move ||{
          ArrayValues{
            val: val,
            array_sep: None,
            comment_nl: None,
            array_vals: None
          }
        }
      )
    )
  )
);

named!(array<&str, Array>,
  chain!(
            tag_s!("[")   ~
       ws1: ws            ~
array_vals: array_values? ~
       ws2: ws            ~
            tag_s!("]")   ,
    ||{
      Array{
        values: array_vals,
        ws: WSSep{ws1: ws1, ws2: ws2},
      }
    }
  )
);
// Inline Table
// Note inline-table-sep and array-sep are identical so we'll reuse array-sep
named!(single_keyval<&str, TableKeyVals>,
      chain!(
        key1: key        ~
 keyval_sep1: keyval_sep ~
        val1: val        ,
        ||{//println!("Parse table keyvals");
          TableKeyVals{
            key: key1,
            keyval_sep: keyval_sep1,
            val: val1,
            table_sep: None,
            keyvals: None,
          }
        }
      ) 
);

named!(recursive_keyval<&str, TableKeyVals>,
      chain!(
        key2: key                            ~
 keyval_sep2: keyval_sep                     ~
        val2: val                            ~
  table_sep2: array_sep                      ~
    keyvals2: inline_table_keyvals_non_empty ,
        ||{//println!("Parse recursive table keyvals");
          TableKeyVals{
            key: key2,
            keyval_sep: keyval_sep2,
            val: val2,
            table_sep: Some(table_sep2),
            keyvals: Some(Box::new(keyvals2)),
          }
        }
      )
);

named!(inline_table_keyvals_non_empty<&str, TableKeyVals>,
  alt!(
    complete!(
      recursive_keyval
    )|
    complete!(
      single_keyval
    )
  )
);

named!(pub inline_table<&str, InlineTable>,
  chain!(
        tag_s!("{")                     ~
   ws1: ws                              ~
keyvals:inline_table_keyvals_non_empty  ~
   ws2: ws                              ~
        tag_s!("}")                     ,
        ||{
          InlineTable{
            keyvals: keyvals,
            ws: WSSep{ws1: ws1, ws2: ws2},
          }
        }
  )
);
