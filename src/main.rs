#![allow(dead_code)]
#![feature(plugin)]
#![plugin(regex_macros)]
mod ast;
use ast::{Val, Comment, WSSep, KeyVal, WSKeySep, TableType, Table,
          PartialTime, TimeOffsetAmount, TimeOffset, FullTime, PosNeg,
          FullDate, DateTime, CommentNewLines, CommentOrNewLines,
          ArrayValues, Array, TableKeyVals, ValLength, Type};
#[macro_use]
extern crate nom;
extern crate regex;
use nom::IResult;
use nom::IResult::Done;
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

// named!(basic_unescaped<&str, &str>, re_match!(r" |!|[#-\[]|[\]-􏿿]"));
// named!(escaped<&str, &str>, re_match!("(\\\\\")|(\\\\)|(\\\\/)|(\\b)|(\\f)|(\\n)|(\\r)|(\\t)|(u[0-9A-Z]{4})|(U[0-9A-Z]{8})"));
// named!(basic_char<&str, &str>, alt!(basic_unescaped | escaped));
named!(unquoted_key<&str, &str>, take_while1_s!(is_keychar));
named!(quoted_key<&str, &str>, re_find_static!("\"( |!|[#-\\[]|[\\]-􏿿]|(\\\\\")|(\\\\\\\\)|(\\\\/)|(\\\\b)|(\\\\f)|(\\\\n)|(\\\\r)|(\\\\t)|(\\\\u[0-9A-Z]{4})|(\\\\U[0-9A-Z]{8}))+\""));

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

fn ParseVal<I, O>(input: &str) -> IResult<&str,Val> {
  named!(peek_array<&str, Array>, peek!(array));
  named!(peek_date_time<&str, DateTime>, peek!(date_time));
  named!(peek_integer<&str, &str>, peek!(integer));
  named!(peek_float<&str, &str>, peek!(float));
  named!(peek_boolean<&str, &str>, peek!(boolean));
  //named!(peek_inline_table<&str, &str>, peek!(inline_table));

  named!(parse_array<&str, Array>, call!(array));
  named!(parse_date_time<&str, DateTime>, call!(date_time));
  named!(parse_integer<&str, &str>, call!(integer));
  named!(parse_float<&str, &str>, call!(float));
  named!(parse_boolean<&str, &str>, call!(boolean));
  //named!(parse_inline_table<&str, &str>, call!(inline_table));
  let v = ValLength(Type::Array, -1);
  v = match (peek_array(input), v) {
    (Done(ref i, ref o), ValLength(ref a, ref b)) if o.len() > *b => ValLength(Type::Array, o.len()),
    _ => v
  };
  v = match (peek_date_time(input), v) {
    (Done(ref i, ref o), ValLength(ref a, ref b)) if o.len() > *b => ValLength(Type::DateTime, o.len()),
    _ => v
  };
  v = match (peek_integer(input), v) {
    (Done(ref i, ref o), ValLength(ref a, ref b)) if o.len() > *b => ValLength(Type::Integer, o.len()),
    _ => v
  };
  v = match (peek_float(input), v) {
    (Done(ref i, ref o), ValLength(ref a, ref b)) if o.len() > *b => ValLength(Type::Float, o.len()),
    _ => v
  };
  v = match (peek_boolean(input), v) {
    (Done(ref i, ref o), ValLength(ref a, ref b)) if o.len() > *b => ValLength(Type::Boolean, o.len()),
    _ => v
  };
  /*v = match (peek_array(input), v) {
    (Done(i, o), ValLength(a, b)) if o.length() > b => ValLength(Type::InlineTable, o.len()),
    _ => v
  };*/
  match v {
    ValLength(Type::Array, _)          => return match &parse_array(input) {
                                                    &IResult::Done(i, ref o) => Done(i, Val::Array(Box::new(o))),
                                                    &IResult::Error(ref i) => IResult::Error(*i),
                                                    &IResult::Incomplete(ref i) => IResult::Incomplete(*i),
                                                 },
    ValLength(Type::DateTime, _)       => return match &parse_date_time(input) {
                                                    &IResult::Done(ref i, ref o) => Done(i, Val::DateTime(*o)),
                                                    &IResult::Error(ref i) => IResult::Error(*i),
                                                    &IResult::Incomplete(ref i) => IResult::Incomplete(*i),
                                                 },
    ValLength(Type::Integer, _)        => return match &parse_integer(input) {
                                                    &IResult::Done(ref i, ref o) => Done(i, Val::Integer(*o)),
                                                    &IResult::Error(ref i) => IResult::Error(*i),
                                                    &IResult::Incomplete(ref i) => IResult::Incomplete(*i),
                                                 },
    ValLength(Type::Float, _)          => return match &parse_float(input) {
                                                    &IResult::Done(ref i, ref o) => Done(i, Val::Float(*o)),
                                                    &IResult::Error(ref i) => IResult::Error(*i),
                                                    &IResult::Incomplete(ref i) => IResult::Incomplete(*i),
                                                 },
    ValLength(Type::Boolean, _)        => return match &parse_boolean(input) {
                                                    &IResult::Done(ref i, ref o) => Done(i, Val::Boolean(*o)),
                                                    &IResult::Error(ref i) => IResult::Error(*i),
                                                    &IResult::Incomplete(ref i) => IResult::Incomplete(*i),
                                                 },
    /*ValLength(Type::InlineTable, _)  => return parse_inline_table(input),*/
  };
}
named!(val<&str, Val>,
  alt!(
    complete!(array)        => {|arr|   Val::Array(Box::new(&arr))}  |
    complete!(date_time)    => {|dt|    Val::DateTime(dt)}          |
    complete!(integer)      => {|int|   Val::Integer(int) }         |
    complete!(float)        => {|flt|   Val::Float(flt)}            |
    complete!(boolean)      => {|b|     Val::Boolean(b)}          /*|
    complete!(inline_table) => {|itab|  Val::InlineTable(itab)}*/
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
          tag_s!(":")                 ~
  minute: re_find_static!("[0-9]{2}") ~
          tag_s!(":")                 ~
  second: re_find_static!("[0-9]{2}") ~
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
   hour: re_find_static!("[0-9]{2}")                                                                      ~
         tag_s!(":")                                                                                      ~
minute: re_find_static!("[0-9]{2}")                                                                       ,
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
   year: re_find_static!("[0-9]{4}") ~
         tag_s!("-") ~
  month: re_find_static!("[0-9]{2}") ~
         tag_s!("-") ~
    day: re_find_static!("[0-9]{2}"),
    ||{
      FullDate{
        year: year, month: month, day: day
      }
    }
  )
);

named!(date_time<&str, DateTime>,
  chain!(
   date: full_date~
   time: full_time,
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
    ||{println!("Parse array sep");
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
        val: val            ~
  array_sep: array_sep?     ~
 comment_nl: comment_or_nl? ,
        ||{
          ArrayValues{
            val: val,
            array_sep: array_sep,
            comment_nl: comment_nl,
            array_vals: None
          }
        }
      )
    ) |
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
    )
  )
);

named!(array<&str, Array>,
  chain!(
            tag_s!("[")   ~
array_vals: array_values? ~
            tag_s!("]")   ,
    ||{
      Array{
        values: array_vals
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
        ||{println!("Parse table keyvals");
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
        ||{println!("Parse recursive table keyvals");
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

// For testing as I go
// TODO: remove when finished
fn main() {
    //let s = "[2010-10-10T10:10:10.33Z , true, 56, \t 1950-03-30T21:04:14.123+05:00]";
    let s = r#""Key\"Name"=55,Key = 23.45"#;
    let r = inline_table_keyvals_non_empty(s);
    println!("{:?}", r);
}

#[cfg(test)]
mod test {
	use nom::IResult::{Done};
	use ::{literal_string, ml_literal_string, boolean, partial_time,
         time_offset_amount, time_offset, full_time, full_date,
         date_time, array};
  use ast::{Val, PartialTime, TimeOffsetAmount, TimeOffset, FullTime, PosNeg, WSSep,
            FullDate, DateTime, ArrayValues, Array};

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
  fn test_non_nested_array() {
    assert_eq!(array("[2010-10-10T10:10:10.33Z, 1950-03-30T21:04:14.123+05:00]"),
      Done("", Array {
        values: Some(ArrayValues {
          val: Val::DateTime(DateTime {
            date: FullDate {
              year: "2010", month: "10", day: "10"
            },
            time: FullTime {
              partial_time: PartialTime {
                hour: "10", minute: "10", second: "10", fraction: "33"
              },
              time_offset: TimeOffset::Z
            }
          }),
          array_sep: Some(WSSep {
            ws1: "", ws2: " "
          }),
          comment_nl: None, array_vals: Some(Box::new(ArrayValues {
            val: Val::DateTime(DateTime {
              date: FullDate {
                year: "1950", month: "03", day: "30"
              },
              time: FullTime {
                partial_time: PartialTime {
                  hour: "21", minute: "04", second: "14", fraction: "123"
                },
                time_offset: TimeOffset::Time(TimeOffsetAmount {
                  pos_neg: PosNeg::Pos, hour: "05", minute: "00"
                })
              }
            }),
            array_sep: None, comment_nl: None, array_vals: None
          }))
        })
      }));
  }

  #[test]
  fn test_nested_array() {
    assert_eq!(array("[[3,4], [4,5], [6]]"),
      Done("", Array {
        values: Some(ArrayValues{
          val: Val::Array(Box::new(Array{
            values: Some(ArrayValues {
              val: Val::Integer("3"), array_sep: Some(WSSep{
                ws1: "", ws2: ""
              }),
              comment_nl: None, array_vals: Some(Box::new(ArrayValues{
                val: Val::Integer("4"), array_sep: None, comment_nl: None, array_vals: None
              }))
            })
          })),
          array_sep: Some(WSSep{
            ws1: "", ws2: " "
          }),
          comment_nl: None, array_vals: Some(Box::new(ArrayValues{
            val: Val::Array(Box::new(Array{
              values: Some(ArrayValues{
                val: Val::Integer("4"), array_sep: Some(WSSep{
                  ws1: "", ws2: ""
                }),
                comment_nl: None, array_vals: Some(Box::new(ArrayValues{
                  val: Val::Integer("5"), array_sep: None, comment_nl: None, array_vals: None
                }))
              })
            })),
            array_sep: Some(WSSep{
              ws1: "", ws2: " "
            }), 
            comment_nl: None, array_vals: Some(Box::new(ArrayValues{
              val: Val::Array(Box::new(Array{
                values: Some(ArrayValues{
                  val: Val::Integer("6"), array_sep: None, comment_nl: None, array_vals: None
                })
              })),
              array_sep: None, comment_nl: None, array_vals: None
            }))
          }))
        })
      })
    );
  }
}