use std::fmt;
use std::fmt::{Display};
use ast::structs::{TableType, WSKeySep, Table, CommentNewLines,
                   CommentOrNewLines, ArrayValue, Array, TableKeyVals,
                   InlineTable, WSSep};
use util::{ws, comment};
use primitives::{key, val, keyval_sep};
// Table
named!(pub table<&str, TableType>,
  alt!(
    complete!(array_table) |
    complete!(std_table)
  )
);

named!(table_subkeys<&str, Vec<WSKeySep> >, many0!(table_sub_key));

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
// Standard Table
named!(std_table<&str, TableType>,
  chain!(
         tag_s!("[")    ~
    ws1: ws             ~
    key: key            ~
subkeys: table_subkeys ~
    ws2: ws             ~
         tag_s!("]")    ,
    ||{
      TableType::Standard(Table{
        ws: WSSep{
          ws1: ws1, ws2: ws2
        },
        key: key, subkeys: subkeys,
      })
    }
  )
);

// Array Table
named!(array_table<&str, TableType>,
  chain!(
         tag_s!("[[")   ~
    ws1: ws             ~
    key: key            ~
subkeys: table_subkeys ~
    ws2: ws             ~
         tag_s!("]]")   ,
    ||{
      TableType::Array(Table{
        ws: WSSep{
          ws1: ws1, ws2: ws2
        },
        key: key, subkeys: subkeys,
      })
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

named!(ws_newline<&str, &str>, re_find_static!("^( | \t|\n|(\r\n))*"));

named!(ws_newlines<&str, &str>, re_find_static!("^(\n|(\r\n))( | \t|\n|(\r\n))*"));

named!(comment_nl<&str, CommentNewLines>,
  chain!(
 prewsnl: ws_newline  ~
 comment: comment     ~
newlines: ws_newlines ,
    ||{
      CommentNewLines{
        pre_ws_nl: prewsnl, comment: comment, newlines: newlines
      }
    }
  )
);

named!(comment_or_nl<&str, CommentOrNewLines>,
  alt!(
    complete!(comment_nl)   => {|com| CommentOrNewLines::Comment(com)} |
    complete!(ws_newlines)  => {|nl|  CommentOrNewLines::NewLines(nl)}
  )
);

named!(array_value<&str, ArrayValue>,
    complete!(
      chain!(
        val: val              ~
  array_sep: array_sep        ~
  comment_nl: comment_or_nl?  ,
        ||{
          ArrayValue{
            val: val,
            array_sep: Some(array_sep),
            comment_nl: comment_nl,
          }
        }
      )
    )
);

// I theory the first alt case should handle all possible values, but in practice it fails to
// to parse some optional combinations, hence the second alt case.
named!(array_value_end<&str, ArrayValue>,
  alt!(    
    complete!(
      chain!(
        val: val              ~
  array_sep: array_sep?       ~
  comment_nl: comment_or_nl?  ,
        ||{
          ArrayValue{
            val: val,
            array_sep: array_sep,
            comment_nl: comment_nl,
          }
        }
      )
    ) |
    complete!(
      chain!(
        val: val             ,
        ||{
          ArrayValue{
            val: val,
            array_sep: None,
            comment_nl: None,
          }
        }
      )
    )
  )
);

named!(array_values<&str, Vec<ArrayValue> >,
  chain!(
   vals: many0!(array_value) ~
   last: array_value_end      ,
   ||{let mut tmp = vec![]; tmp.extend(vals); tmp.push(last); tmp}
  )
);

named!(pub array<&str, Array>,
  chain!(
            tag_s!("[")   ~
       ws1: ws_newline    ~
array_vals: array_values ~
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
        ||{
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
        ||{
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

#[cfg(test)]
mod test {
  use nom::IResult::Done;
  use super::{array, inline_table_keyvals_non_empty, inline_table};
  use ast::structs::{FullDate, Array, ArrayValue, WSSep, TableKeyVals, InlineTable};
  use ::types::{DateTime, TimeOffset, TimeOffsetAmount, Value};
  #[test]
  fn test_non_nested_array() {
    assert_eq!(array("[2010-10-10T10:10:10.33Z, 1950-03-30T21:04:14.123+05:00]"),
      Done("", Array {
        values: vec![ArrayValue {
          val: Value::DateTime(DateTime {
            year: "2010", month: "10", day: "10",
            hour: "10", minute: "10", second: "10", fraction: "33",
            offset: TimeOffset::Z
          }),
          array_sep: Some(WSSep{
            ws1: "", ws2: " "
          }),
          comment_nl: None
        },
        ArrayValue {
          val: Value::DateTime(DateTime{
            year: "1950", month: "03", day: "30",
            hour: "21", minute: "04", second: "14", fraction: "123",
            offset: TimeOffset::Time(TimeOffsetAmount{
              pos_neg: "+", hour: "05", minute: "00"
            })
          }),
          array_sep: None, comment_nl: None
        }],
        ws: WSSep{
          ws1: "", ws2: ""
        }
      })
    );
  }

  #[test]
  fn test_nested_array() {
    assert_eq!(array("[[3,4], [4,5], [6]]"),
      Done("", Array{
        values: vec![
          ArrayValue {
            val: Value::Array(Box::new(Array {
              values: vec![
                ArrayValue {
                  val: Value::Integer("3"), array_sep: Some(WSSep { ws1: "", ws2: "" }),
                  comment_nl: None
                },
                ArrayValue {
                  val: Value::Integer("4"), array_sep: None, comment_nl: None
                }
              ],
              ws: WSSep { ws1 : "", ws2: "" }
            })),
            array_sep: Some(WSSep { ws1: "", ws2: " " }),
            comment_nl: None
          },
          ArrayValue {
            val: Value::Array(Box::new(Array {
              values: vec![
                ArrayValue {
                  val: Value::Integer("4"), array_sep: Some(WSSep { ws1: "", ws2: ""}),
                  comment_nl: None
                },
                ArrayValue {
                    val: Value::Integer("5"), array_sep: None, comment_nl: None
                }
              ],
              ws: WSSep { ws1: "", ws2: ""}
            })),
            array_sep: Some(WSSep { ws1: "", ws2: " "}),
            comment_nl: None
          },
          ArrayValue {
            val: Value::Array(Box::new(Array {
              values: vec![
                ArrayValue {
                  val: Value::Integer("6"), array_sep: None, comment_nl: None
                }
              ],
              ws: WSSep { ws1: "", ws2: ""}
            })),
            array_sep: None, comment_nl: None
          }
        ],
        ws: WSSep {
          ws1: "", ws2: ""
        }
      })
    );
  }

  #[test]
  fn test_inline_table_keyvals_non_empty() {
    assert_eq!(inline_table_keyvals_non_empty("Key = 54 , \"Key2\" = '34.99'"),
      Done("", TableKeyVals{
        key: "Key", keyval_sep: WSSep{
          ws1: " ", ws2: " "
        },
        val: Value::Integer("54"), table_sep: Some(WSSep{
          ws1: " ", ws2: " "
        }),
        keyvals: Some(Box::new(TableKeyVals{
          key: "\"Key2\"", keyval_sep: WSSep{
            ws1: " ", ws2: " "
          },
          val: Value::String("'34.99'"), table_sep: None, keyvals: None
        }))
      })
    );
  }

  #[test]
  fn test_inline_table() {
    assert_eq!(inline_table("{\tKey = 3.14E+5 , \"Key2\" = '''New\nLine'''\t}"),
      Done("", InlineTable{
        keyvals: TableKeyVals{
          key: "Key", keyval_sep: WSSep{
            ws1: " ", ws2: " "
          },
          val: Value::Float("3.14E+5"), table_sep: Some(WSSep{
            ws1: " ", ws2: " "
          }),
          keyvals: Some(Box::new(TableKeyVals{
            key: "\"Key2\"", keyval_sep: WSSep{
              ws1: " ", ws2: " "
            },
            val: Value::String("\'\'\'New\nLine\'\'\'"), table_sep: None, keyvals: None
          }))
        },
        ws: WSSep{
          ws1: "\t", ws2: "\t"
        }
      })
    );
  }
}
