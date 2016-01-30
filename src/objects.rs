use ast::structs::{TableType, WSKeySep, Table, CommentNewLines,
                   CommentOrNewLines, ArrayValue, Array,
                   InlineTable, WSSep, TableKeyVal};
use parser::{Parser, count_lines};

impl<'a> Parser<'a> {
  // Table
  method!(pub table<Parser<'a>, &'a str, TableType>, mut self,
    alt!(
      complete!(call_m!(self.array_table)) |
      complete!(call_m!(self.std_table))
    )
  );

  method!(table_subkeys<Parser<'a>, &'a str, Vec<WSKeySep> >, mut self, many0!(call_m!(self.table_subkey)));

  method!(table_subkey<Parser<'a>, &'a str, WSKeySep>, mut self,
    chain!(
      ws1: call_m!(self.ws)         ~
           tag_s!(".")~
      ws2: call_m!(self.ws)         ~
      key: call_m!(self.key)        ,
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
  method!(std_table<Parser<'a>, &'a str, TableType>, mut self,
    chain!(
           tag_s!("[")    ~
      ws1: call_m!(self.ws)             ~
      key: call_m!(self.key)            ~
  subkeys: call_m!(self.table_subkeys)  ~
      ws2: call_m!(self.ws)             ~
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
  method!(array_table<Parser<'a>, &'a str, TableType>, mut self,
    chain!(
           tag_s!("[[")   ~
      ws1: call_m!(self.ws)             ~
      key: call_m!(self.key)            ~
  subkeys: call_m!(self.table_subkeys)  ~
      ws2: call_m!(self.ws)             ~
           tag_s!("]]")   ,
      ||{
        *self.last_table.borrow_mut() = key;
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
  method!(array_sep<Parser<'a>, &'a str, WSSep>, mut self,
    chain!(
      ws1: call_m!(self.ws)         ~
           tag_s!(",")~
      ws2: call_m!(self.ws)         ,
      ||{
        WSSep{ws1: ws1, ws2: ws2
        }
      }
    )
  );

  // TODO: remove the chain macro
  method!(ws_newline<Parser<'a>, &'a str, &'a str>, self,
    chain!(
   string: re_find!("^( |\t|\n|(\r\n))*"),
      ||{string}
    )
   );

  method!(ws_newlines<Parser<'a>, &'a str, &'a str>, self,
    chain!(
   string: re_find!("^(\n|(\r\n))( |\t|\n|(\r\n))*"),
      ||{self.line_count.set(self.line_count.get() + count_lines(string)); string}
    )
  );

  method!(comment_nl<Parser<'a>, &'a str, CommentNewLines>, mut self,
    chain!(
   prewsnl: call_m!(self.ws_newline)  ~
   comment: call_m!(self.comment)     ~
  newlines: call_m!(self.ws_newlines) ,
      ||{
        CommentNewLines{
          pre_ws_nl: prewsnl, comment: comment, newlines: newlines
        }
      }
    )
  );

  method!(comment_or_nl<Parser<'a>, &'a str, CommentOrNewLines>, mut self,
    alt!(
      complete!(call_m!(self.comment_nl))   => {|com| CommentOrNewLines::Comment(com)} |
      complete!(call_m!(self.ws_newlines))  => {|nl|  CommentOrNewLines::NewLines(nl)}
    )
  );

  // method!(comment_or_ws_or_nl<Parser<'a>, &'a str, CommentOrNewLines>, mut self,
  //   alt!(
  //     complete!(call_m!(self.comment_nl))   => {|com| CommentOrNewLines::Comment(com)} |
  //     complete!(call_m!(self.ws_newlines))  => {|nl|  CommentOrNewLines::NewLines(nl)} |
  //     complete!(call_m!(self.ws_newline))   => {|ws|  CommentOrNewLines::NewLines(ws)}
  //   )
  // );

  // TODO: Redo this with array_sep wrapped in a complete!() ?
  method!(array_value<Parser<'a>, &'a str, ArrayValue>, mut self,
    alt!(
      complete!(
        chain!(
          val: call_m!(self.val)                        ~
    array_sep: call_m!(self.array_sep)                  ~
    comment_nl: complete!(call_m!(self.comment_or_nl)) ?,
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
          val: call_m!(self.val)                       ~
    comment_nl: complete!(call_m!(self.comment_or_nl)) ,
          ||{
            ArrayValue{
              val: val,
              array_sep: None,
              comment_nl: Some(comment_nl),
            }
          }
        )
      )
    )
  );

  method!(array_values<Parser<'a>, &'a str, Vec<ArrayValue> >, mut self,
    chain!(
     vals: many0!(call_m!(self.array_value)) ,
     ||{let mut tmp = vec![];
        tmp.extend(vals);
        tmp
      }
    )
  );

  method!(pub array<Parser<'a>, &'a str, Array>, mut self,
    chain!(
              tag_s!("[")   ~
         ws1: call_m!(self.ws_newline)    ~
  array_vals: call_m!(self.array_values) ~
         ws2: call_m!(self.ws_newline)            ~
              tag_s!("]")   ,
      ||{
        Array{
          values: array_vals,
          ws: WSSep{ws1: ws1, ws2: ws2},
        }
      }
    )
  );

  method!(table_keyval<Parser<'a>, &'a str, TableKeyVal>, mut self,
        chain!(
          ws1: call_m!(self.ws)     ~
       keyval: call_m!(self.keyval) ~
          ws2: call_m!(self.ws)     ,
          ||{
            TableKeyVal{
              keyval: keyval,
              kv_sep: WSSep{ws1: ws1, ws2: ws2}
            }
          }
        )
  );

  method!(inline_table_keyvals_non_empty<Parser<'a>, &'a str, Vec<TableKeyVal> >, mut self, separated_list!(tag_s!(","), call_m!(self.table_keyval)));

  method!(pub inline_table<Parser<'a>, &'a str, InlineTable>, mut self,
    chain!(
           tag_s!("{")                                ~
      ws1: call_m!(self.ws)                                         ~
  keyvals: complete!(call_m!(self.inline_table_keyvals_non_empty))? ~
      ws2: call_m!(self.ws)                                         ~
           tag_s!("}")                                ,
          ||{
            InlineTable{
              keyvals: keyvals,
              ws: WSSep{ws1: ws1, ws2: ws2}
            }
          }
    )
  );
}

#[cfg(test)]
mod test {
  use nomplusplus::IResult::Done;
  use ast::structs::{Array, ArrayValue, WSSep, TableKeyVal, InlineTable, WSKeySep,
                     KeyVal, CommentNewLines, Comment, CommentOrNewLines, Table,
                     TableType, Value, StrType};
  use ::types::{DateTime, TimeOffset, TimeOffsetAmount};
  use parser::Parser;

  #[test]
  fn test_table() {
    let mut p = Parser::new();
    assert_eq!(p.table("[ _underscore_ . \"-δáƨλèƨ-\" ]").1, Done("",
      TableType::Standard(Table{
        ws: WSSep{ws1: " ", ws2: " "}, key: "_underscore_", subkeys: vec![
          WSKeySep{ws: WSSep{ws1: " ", ws2: " "}, key: "\"-δáƨλèƨ-\""}
        ]
      })
    ));
    p = Parser::new();
    assert_eq!(p.table("[[\t NumberOne\t.\tnUMBERtWO \t]]").1, Done("",
      TableType::Array(Table{
        ws: WSSep{ws1: "\t ", ws2: " \t"}, key: "NumberOne", subkeys: vec![
          WSKeySep{ws: WSSep{ws1: "\t", ws2: "\t"}, key: "nUMBERtWO"}
        ]
      })
    ));
  }

  #[test]
  fn test_table_subkey() {
    let p = Parser::new();
    assert_eq!(p.table_subkey("\t . \t\"áƭúƨôèλôñèƭúññèôúñôèƭú\"").1, Done("",
      WSKeySep{ws: WSSep{ws1: "\t ", ws2: " \t"}, key: "\"áƭúƨôèλôñèƭúññèôúñôèƭú\""},
    ));
  }

  #[test]
  fn test_table_subkeys() {
    let p = Parser::new();
    assert_eq!(p.table_subkeys(" .\tAPPLE.MAC . \"ßÓÓK\"").1, Done("",
      vec![
        WSKeySep{ws: WSSep{ws1: " ", ws2: "\t"}, key: "APPLE"},
        WSKeySep{ws: WSSep{ws1: "", ws2: ""}, key: "MAC"},
        WSKeySep{ws: WSSep{ws1: " ", ws2: " "}, key: "\"ßÓÓK\""}
      ]
    ));
  }

  #[test]
  fn test_std_table() {
    let p = Parser::new();
    assert_eq!(p.std_table("[Dr-Pepper  . \"ƙè¥_TWÓ\"]").1, Done("",
      TableType::Standard(Table{
        ws: WSSep{ws1: "", ws2: ""}, key: "Dr-Pepper", subkeys: vec![
          WSKeySep{ws: WSSep{ws1: "  ", ws2: " "}, key: "\"ƙè¥_TWÓ\""}
        ]
      })
    ));
  }

  #[test]
  fn test_array_table() {
    let p = Parser::new();
    assert_eq!(p.array_table("[[\"ƙè¥ôñè\"\t. key_TWO]]").1, Done("",
      TableType::Array(Table{
        ws: WSSep{ws1: "", ws2: ""}, key: "\"ƙè¥ôñè\"", subkeys: vec![
          WSKeySep{ws: WSSep{ws1: "\t", ws2: " "}, key: "key_TWO"}
        ]
      })
    ));
  }

  #[test]
  fn test_array_sep() {
    let p = Parser::new();
    assert_eq!(p.array_sep("  ,  ").1, Done("", WSSep{ws1: "  ", ws2: "  "}));
  }

  #[test]
  fn test_ws_newline() {
    let p = Parser::new();
    assert_eq!(p.ws_newline("\t\n\n").1, Done("", "\t\n\n"));
  }

  #[test]
  fn test_ws_newlines() {
    let p = Parser::new();
    assert_eq!(p.ws_newlines("\n \t\n\r\n ").1, Done("", "\n \t\n\r\n "));
  }

  #[test]
  fn test_comment_nl() {
    let p = Parser::new();
    assert_eq!(p.comment_nl("\r\n\t#çô₥₥èñƭñèωℓïñè\n \n \n").1, Done("",
      CommentNewLines{
        pre_ws_nl: "\r\n\t", comment: Comment{text: "çô₥₥èñƭñèωℓïñè"},
        newlines: "\n \n \n"
      }
    ));
  }

  #[test]
  fn test_comment_or_nl() {
    let mut p = Parser::new();
    assert_eq!(p.comment_or_nl("#ωôřƙωôřƙ\n").1, Done("",
      CommentOrNewLines::Comment(CommentNewLines{
        pre_ws_nl: "", comment: Comment{text: "ωôřƙωôřƙ"}, newlines: "\n"
      })
    ));
    p = Parser::new();
    assert_eq!(p.comment_or_nl(" \t\n#ωôřƙωôřƙ\n \r\n").1, Done("",
      CommentOrNewLines::Comment(CommentNewLines{
        pre_ws_nl: " \t\n", comment: Comment{text: "ωôřƙωôřƙ"}, newlines: "\n \r\n"
      })
    ));
    p = Parser::new();
    assert_eq!(p.comment_or_nl("\n\t\r\n ").1, Done("", CommentOrNewLines::NewLines("\n\t\r\n ")));
  }

  // #[test]
  // fn test_comment_or_ws_or_nl() {
  //   let mut p = Parser::new();
  //   assert_eq!(p.comment_or_ws_nl("   \t").1, Done("", CommentOrNewLines::NewLines("   \t")));
  // }

  #[test]
  fn test_array_value() {
    let mut p = Parser::new();
    assert_eq!(p.array_value("54.6, \n#çô₥₥èñƭ\n\n").1,
      Done("",ArrayValue{
        val: Value::Float("54.6"), array_sep: Some(WSSep{
          ws1: "", ws2: " "
        }),
        comment_nl: Some(CommentOrNewLines::Comment(CommentNewLines{
          pre_ws_nl: "\n", comment: Comment{text: "çô₥₥èñƭ"}, newlines: "\n\n"
        }))
      })
    );
    p = Parser::new();
    assert_eq!(p.array_value("\"ƨƥáϱλèƭƭï\"").1,
      Done("",ArrayValue{
        val: Value::String("ƨƥáϱλèƭƭï", StrType::Basic), array_sep: None, comment_nl: None
      })
    );
    p = Parser::new();
    assert_eq!(p.array_value("44_9 , ").1,
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
    let mut p = Parser::new();
    assert_eq!(p.array_values("1, 2, 3").1, Done("", vec![
      ArrayValue{val: Value::Integer("1"), array_sep: Some(WSSep{ws1: "", ws2: " "}),
      comment_nl: None},
      ArrayValue{val: Value::Integer("2"), array_sep: Some(WSSep{ws1: "", ws2: " "}),
      comment_nl: None},
      ArrayValue{val: Value::Integer("3"), array_sep: None, comment_nl: None}
    ]));
    p = Parser::new();
    assert_eq!(p.array_values("1, 2, #çô₥₥èñƭ\n3, ").1, Done("", vec![
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
    let p = Parser::new();
    assert_eq!(p.array("[2010-10-10T10:10:10.33Z, 1950-03-30T21:04:14.123+05:00]").1,
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
    let p = Parser::new();
    assert_eq!(p.array("[[3,4], [4,5], [6]]").1,
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
    let p = Parser::new();
    assert_eq!(p.table_keyval("\"Ì WúƲ Húϱƨ!\"\t=\t'Mè ƭôô!' ").1, Done("", TableKeyVal{
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
    let p = Parser::new();
    assert_eq!(p.inline_table_keyvals_non_empty(" Key =\t54,\"Key2\" = '34.99'\t").1,
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
    let p = Parser::new();
    assert_eq!(p.inline_table("{\tKey = 3.14E+5 , \"Key2\" = '''New\nLine'''\t}").1,
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
