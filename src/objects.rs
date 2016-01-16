use std::slice::SliceConcatExt;
use ast::structs::{TableType, WSKeySep, Table, CommentNewLines,
                   CommentOrNewLines, ArrayValue, Array,
                   InlineTable, WSSep, TableKeyVal};
use util::{ws, comment};
use primitives::{key, val, keyval};
use parser::{LINE_COUNT, LAST_TABLE, count_lines};

// Table
named!(pub table<&str, TableType>,
  alt!(
    complete!(array_table) |
    complete!(std_table)
  )
);

named!(table_subkeys<&str, Vec<WSKeySep> >, many0!(table_subkey));

named!(table_subkey<&str, WSKeySep>,
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
subkeys: table_subkeys  ~
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
subkeys: table_subkeys  ~
    ws2: ws             ~
         tag_s!("]]")   ,
    ||{
      LAST_TABLE.with(|t| *t.borrow_mut() = key);
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
    ||{
      WSSep{ws1: ws1, ws2: ws2
      }
    }
  )
);

named!(ws_newline<&str, &str>,
  chain!(
 string: re_find!("^( |\t|\n|(\r\n))*"),
    ||{LINE_COUNT.with(|f| *f.borrow_mut() = *f.borrow() + count_lines(string)); string}
  )
 );

named!(ws_newlines<&str, &str>,
  chain!(
 string: re_find!("^(\n|(\r\n))( |\t|\n|(\r\n))*"),
    ||{LINE_COUNT.with(|f| *f.borrow_mut() = *f.borrow() + count_lines(string)); string}
  )
);

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

// TODO: Redo this with array_sep wrapped in a complete!() ?
named!(array_value<&str, ArrayValue>,
  alt!(
    complete!(
      chain!(
        val: val                        ~
  array_sep: array_sep                  ~
  comment_nl: complete!(comment_or_nl)? ,
        ||{
          ArrayValue{
            val: val,
            array_sep: Some(array_sep),
            comment_nl: comment_nl,
          }
        }
      )
    ) |
    complete!(
      chain!(
        val: val                        ~
  comment_nl: complete!(comment_or_nl)? ,
        ||{
          ArrayValue{
            val: val,
            array_sep: None,
            comment_nl: comment_nl,
          }
        }
      )
    )
  )
);

named!(array_values<&str, Vec<ArrayValue> >,
  chain!(
   vals: many0!(array_value) ,
   ||{let mut tmp = vec![];
      tmp.extend(vals);
      tmp
    }
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

named!(table_keyval<&str, TableKeyVal>,
      chain!(
        ws1: ws     ~
     keyval: keyval ~
        ws2: ws     ,
        ||{
          TableKeyVal{
            keyval: keyval,
            kv_sep: WSSep{ws1: ws1, ws2: ws2}
          }
        }
      )
);

named!(inline_table_keyvals_non_empty<&str, Vec<TableKeyVal> >, separated_list!(tag_s!(","), table_keyval));

named!(pub inline_table<&str, InlineTable>,
  chain!(
         tag_s!("{")                                ~
    ws1: ws                                         ~
keyvals: complete!(inline_table_keyvals_non_empty)? ~
    ws2: ws                                         ~
         tag_s!("}")                                ,
        ||{
          InlineTable{
            keyvals: keyvals,
            ws: WSSep{ws1: ws1, ws2: ws2}
          }
        }
  )
);

#[cfg(test)]
mod test {
  use nom::IResult::Done;
  use super::{array, inline_table_keyvals_non_empty, inline_table, table_keyval,
              array_values, comment_or_nl, ws_newlines, ws_newline, array_value,
              array_sep, array_table, comment_nl, std_table, table_subkeys,
              table_subkey, table};
  use ast::structs::{Array, ArrayValue, WSSep, TableKeyVal, InlineTable, WSKeySep,
                     KeyVal, CommentNewLines, Comment, CommentOrNewLines, Table,
                     TableType, Value, StrType};
  use ::types::{DateTime, TimeOffset, TimeOffsetAmount};

  #[test]
  fn test_table() {
    assert_eq!(table("[ _underscore_ . \"-δáƨλèƨ-\" ]"), Done("",
      TableType::Standard(Table{
        ws: WSSep{ws1: " ", ws2: " "}, key: "_underscore_", subkeys: vec![
          WSKeySep{ws: WSSep{ws1: " ", ws2: " "}, key: "\"-δáƨλèƨ-\""}
        ]
      })
    ));
    assert_eq!(table("[[\t NumberOne\t.\tnUMBERtWO \t]]"), Done("",
      TableType::Array(Table{
        ws: WSSep{ws1: "\t ", ws2: " \t"}, key: "NumberOne", subkeys: vec![
          WSKeySep{ws: WSSep{ws1: "\t", ws2: "\t"}, key: "nUMBERtWO"}
        ]
      })
    ));
  }

  #[test]
  fn test_table_subkey() {
    assert_eq!(table_subkey("\t . \t\"áƭúƨôèλôñèƭúññèôúñôèƭú\""), Done("",
      WSKeySep{ws: WSSep{ws1: "\t ", ws2: " \t"}, key: "\"áƭúƨôèλôñèƭúññèôúñôèƭú\""},
    ));
  }

  #[test]
  fn test_table_subkeys() {
    assert_eq!(table_subkeys(" .\tAPPLE.MAC . \"ßÓÓK\""), Done("",
      vec![
        WSKeySep{ws: WSSep{ws1: " ", ws2: "\t"}, key: "APPLE"},
        WSKeySep{ws: WSSep{ws1: "", ws2: ""}, key: "MAC"},
        WSKeySep{ws: WSSep{ws1: " ", ws2: " "}, key: "\"ßÓÓK\""}
      ]
    ));
  }

  #[test]
  fn test_std_table() {
    assert_eq!(std_table("[Dr-Pepper  . \"ƙè¥_TWÓ\"]"), Done("",
      TableType::Standard(Table{
        ws: WSSep{ws1: "", ws2: ""}, key: "Dr-Pepper", subkeys: vec![
          WSKeySep{ws: WSSep{ws1: "  ", ws2: " "}, key: "\"ƙè¥_TWÓ\""}
        ]
      })
    ));
  }

  #[test]
  fn test_array_table() {
    assert_eq!(array_table("[[\"ƙè¥ôñè\"\t. key_TWO]]"), Done("",
      TableType::Array(Table{
        ws: WSSep{ws1: "", ws2: ""}, key: "\"ƙè¥ôñè\"", subkeys: vec![
          WSKeySep{ws: WSSep{ws1: "\t", ws2: " "}, key: "key_TWO"}
        ]
      })
    ));
  }

  #[test]
  fn test_array_sep() {
    assert_eq!(array_sep("  ,  "), Done("", WSSep{ws1: "  ", ws2: "  "}));
  }

  #[test]
  fn test_ws_newline() {
    assert_eq!(ws_newline("\t\n\n"), Done("", "\t\n\n"));
  }

  #[test]
  fn test_ws_newlines() {
    assert_eq!(ws_newlines("\n \t\n\r\n "), Done("", "\n \t\n\r\n "));
  }

  #[test]
  fn test_comment_nl() {
    assert_eq!(comment_nl("\r\n\t#çô₥₥èñƭñèωℓïñè\n \n \n"), Done("",
      CommentNewLines{
        pre_ws_nl: "\r\n\t", comment: Comment{text: "çô₥₥èñƭñèωℓïñè"},
        newlines: "\n \n \n"
      }
    ));
  }

  #[test]
  fn test_comment_or_nl() {
    assert_eq!(comment_or_nl("#ωôřƙωôřƙ\n"), Done("",
      CommentOrNewLines::Comment(CommentNewLines{
        pre_ws_nl: "", comment: Comment{text: "ωôřƙωôřƙ"}, newlines: "\n"
      })
    ));
    assert_eq!(comment_or_nl(" \t\n#ωôřƙωôřƙ\n \r\n"), Done("",
      CommentOrNewLines::Comment(CommentNewLines{
        pre_ws_nl: " \t\n", comment: Comment{text: "ωôřƙωôřƙ"}, newlines: "\n \r\n"
      })
    ));
    assert_eq!(comment_or_nl("\n\t\r\n "), Done("", CommentOrNewLines::NewLines("\n\t\r\n ")));
  }

  #[test]
  fn test_array_value() {
    assert_eq!(array_value("54.6, \n#çô₥₥èñƭ\n\n"),
      Done("",ArrayValue{
        val: Value::Float("54.6"), array_sep: Some(WSSep{
          ws1: "", ws2: " "
        }),
        comment_nl: Some(CommentOrNewLines::Comment(CommentNewLines{
          pre_ws_nl: "\n", comment: Comment{text: "çô₥₥èñƭ"}, newlines: "\n\n"
        }))
      })
    );
    assert_eq!(array_value("\"ƨƥáϱλèƭƭï\""),
      Done("",ArrayValue{
        val: Value::String("ƨƥáϱλèƭƭï", StrType::Basic), array_sep: None, comment_nl: None
      })
    );
    assert_eq!(array_value("44_9 , "),
      Done("",ArrayValue{
        val: Value::Integer("44_9"), array_sep: Some(WSSep{
          ws1: " ", ws2: " "
        }),
        comment_nl: None
      })
    );
  }

  #[test]
  fn test_array_values() {
    assert_eq!(array_values("1, 2, 3"), Done("", vec![
      ArrayValue{val: Value::Integer("1"), array_sep: Some(WSSep{ws1: "", ws2: " "}),
      comment_nl: None},
      ArrayValue{val: Value::Integer("2"), array_sep: Some(WSSep{ws1: "", ws2: " "}),
      comment_nl: None},
      ArrayValue{val: Value::Integer("3"), array_sep: None, comment_nl: None}
    ]));
    assert_eq!(array_values("1, 2, #çô₥₥èñƭ\n3, "), Done("", vec![
      ArrayValue{val: Value::Integer("1"), array_sep: Some(WSSep{ws1: "", ws2: " "}),
      comment_nl: None},
      ArrayValue{val: Value::Integer("2"), array_sep: Some(WSSep{ws1: "", ws2: " "}),
      comment_nl: Some(CommentOrNewLines::Comment(CommentNewLines{pre_ws_nl: "",
        comment: Comment{text: "çô₥₥èñƭ"},
        newlines: "\n"}))},
      ArrayValue{val: Value::Integer("3"), array_sep: Some(WSSep{ws1: "", ws2: " "}),
      comment_nl: None}
    ]));
  }

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
  fn test_table_keyval() {
    assert_eq!(table_keyval("\"Ì WúƲ Húϱƨ!\"\t=\t'Mè ƭôô!' "), Done("", TableKeyVal{
      keyval: KeyVal{
        key: "\"Ì WúƲ Húϱƨ!\"", val: Value::String("Mè ƭôô!", StrType::Literal), keyval_sep: WSSep{
          ws1: "\t", ws2: "\t"
        }
      },
      kv_sep: WSSep{
        ws1: "", ws2: " "
      },
    }));
  }

  #[test]
  fn test_inline_table_keyvals_non_empty() {
    assert_eq!(inline_table_keyvals_non_empty(" Key =\t54,\"Key2\" = '34.99'\t"),
      Done("", vec![
        TableKeyVal{
          keyval: KeyVal{
            key: "Key", keyval_sep: WSSep{
              ws1: " ", ws2: "\t"
            },
            val: Value::Integer("54")
          },
          kv_sep: WSSep{
            ws1: " ", ws2: ""
          }
        },
        TableKeyVal{
          keyval: KeyVal{
            key: "\"Key2\"", keyval_sep: WSSep{
              ws1: " ", ws2: " "
            },
            val: Value::String("34.99", StrType::Literal)
          },
          kv_sep: WSSep{
            ws1: "", ws2: "\t"
          }
        }
      ])
    );
  }

  #[test]
  fn test_inline_table() {
    assert_eq!(inline_table("{\tKey = 3.14E+5 , \"Key2\" = '''New\nLine'''\t}"),
      Done("", InlineTable{
        keyvals: Some(vec![
          TableKeyVal{
            keyval: KeyVal{
              key: "Key", keyval_sep: WSSep{
                ws1: " ", ws2: " "
              },
              val: Value::Float("3.14E+5")
            },
            kv_sep: WSSep{
              ws1: "", ws2: " "
            }
          },
          TableKeyVal{
            keyval: KeyVal{
              key: "\"Key2\"", keyval_sep: WSSep{
                ws1: " ", ws2: " "
              },
              val: Value::String("New\nLine", StrType::MLLiteral)
            },
            kv_sep: WSSep{
              ws1: " ", ws2: "\t"
            }
          }
        ]),
        ws: WSSep{ws1: "\t", ws2: ""}
      })
    );
  }
}
