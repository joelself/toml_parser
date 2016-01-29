use ast::structs::{Toml, NLExpression, Expression, WSSep};
use parser::{Parser, ParseData};

impl<'a> Parser {
  named!(pub toml<&'a str, Toml<'a>, &mut ParseData<'a> >, data,
    chain!(
      expr: call_d!(Parser::expression, data)    ~
  nl_exprs: call_d!(Parser::nl_expressions, data),
      ||{
        let mut tmp = vec![NLExpression{ nl: "", expr: expr}];
        tmp.extend(nl_exprs); Toml{ exprs: tmp}
      }
    )
  );

  named!(nl_expressions<&'a str, Vec<NLExpression<'a>>, &mut ParseData<'a> >, data, many0!(call_d!(Parser::nl_expression, data)));

  named!(nl_expression<&'a str, NLExpression<'a>, &mut ParseData<'a> >, data,
    chain!(
       nl: call_d!(Parser::newline, data)    ~
     expr: call_d!(Parser::expression, data) ,
      ||{
        NLExpression{
          nl: nl, expr: expr,
        }
      }
    )
  );

  // Expression
  named!(expression<&'a str,  Expression<'a>, &mut ParseData<'a> >, data,
    alt!(
      complete!(call_d!(Parser::table_comment, data))  |
      complete!(call_d!(Parser::keyval_comment, data)) |
      complete!(call_d!(Parser::ws_comment, data))     |
      complete!(call_d!(Parser::ws_expr, data))
    )
  );

  named!(ws_expr<&'a str, Expression<'a>, &mut ParseData<'a> >, data,
    chain!(
      ws: call_d!(Parser::ws, data),
      ||{
        Expression{
          ws: WSSep{
            ws1: ws,
            ws2: "",
          },
          keyval: None, table: None, comment: None,
        }
      }
    ));

  named!(table_comment<&'a str, Expression<'a>, &mut ParseData<'a> >, data,
    chain!(
      ws1: call_d!(Parser::ws, data)                 ~
    table: call_d!(Parser::table, data)              ~
      ws2: call_d!(Parser::ws, data)                 ~
  comment: complete!(call_d!(Parser::comment, data))?,
      ||{
        Expression{
          ws: WSSep{
            ws1: ws1,
            ws2: ws2,
          },
          keyval: None, table: Some(table), comment: comment,
        }
      }
    )
  );

  named!(keyval_comment<&'a str, Expression<'a>, &mut ParseData<'a> >, data,
    chain!(
      ws1: call_d!(Parser::ws, data)       ~
   keyval: call_d!(Parser::keyval, data)   ~
      ws2: call_d!(Parser::ws, data)       ~
  comment: complete!(call_d!(Parser::comment, data)) ? ,
      ||{
        Expression{
          ws: WSSep{
            ws1: ws1,
            ws2: ws2,
          },
          keyval: Some(keyval), table: None, comment: comment,
        }
      }
    )
  );

  named!(ws_comment<&'a str, Expression<'a>, &mut ParseData<'a> >, data,
    chain!(
       ws: call_d!(Parser::ws, data)     ~
  comment: call_d!(Parser::comment, data),
      ||{
        Expression{
          ws: WSSep{
            ws1: ws,
            ws2: "",
          },
          keyval: None, table: None, comment: Some(comment),
        }
      }
    )
  );
}

#[cfg(test)]
mod test {
  use nom::IResult::Done;
  use Parser;
  use ast::structs::{Expression, Comment, WSSep, KeyVal, Table, WSKeySep,
                     TableType, Value, NLExpression, StrType, ArrayValue, Toml,
                     Time, Array};
  use types::{TimeOffsetAmount, DateTime, TimeOffset};
  

  #[test]
  fn test_toml() {
    let p = new();
    assert_eq!(p.toml(
r#"# Tλïƨ ïƨ á TÓM£ δôçú₥èñƭ.

title = "TÓM£ Éжá₥ƥℓè"

[owner]
name = "Tô₥ Þřèƨƭôñ-Wèřñèř"
dob = 1979-05-27T07:32:00-08:00 # Fïřƨƭ çℓáƨƨ δáƭèƨ

[database]
server = "192.168.1.1"
ports = [ 8001, 8001, 8002 ]
connection_max = 5000
enabled = true"#), Done("",
      Toml { exprs: vec![
        NLExpression {
          nl: "", expr: Expression {
            ws: WSSep { ws1: "", ws2: "" }, keyval: None, table: None, comment: Some(Comment {
              text: " Tλïƨ ïƨ á TÓM£ δôçú₥èñƭ."
            })
          }
        },
        NLExpression {
          nl: "\n", expr: Expression {
            ws: WSSep { ws1: "", ws2: "" }, keyval: None, table: None, comment: None
          }
        },
        NLExpression {
          nl: "\n", expr: Expression {
            ws: WSSep { ws1: "", ws2: "" }, keyval: Some(KeyVal {
              key: "title", keyval_sep: WSSep { ws1: " ", ws2: " " }, val: Value::String("TÓM£ Éжá₥ƥℓè", StrType::Basic)
            }),
            table: None, comment: None
          }
        },
        NLExpression {
          nl: "\n", expr: Expression {
            ws: WSSep { ws1: "", ws2: "" }, keyval: None, table: None, comment: None
          }
        },
        NLExpression {
          nl: "\n", expr: Expression {
            ws: WSSep { ws1: "", ws2: "" }, keyval: None, table: Some(TableType::Standard(Table {
              ws: WSSep { ws1: "", ws2: "" }, key: "owner", subkeys: vec![]
            })), comment: None
          }
        },
        NLExpression {
          nl: "\n", expr: Expression {
            ws: WSSep { ws1: "", ws2: "" }, keyval: Some(KeyVal {
              key: "name", keyval_sep: WSSep { ws1: " ", ws2: " " }, val: Value::String("Tô₥ Þřèƨƭôñ-Wèřñèř", StrType::Basic)
            }), table: None, comment: None
          }
        },
        NLExpression {
          nl: "\n", expr: Expression {
            ws: WSSep { ws1: "", ws2: " " }, keyval: Some(KeyVal {
              key: "dob", keyval_sep: WSSep { ws1: " ", ws2: " " }, val: Value::DateTime(DateTime {
                year: "1979", month: "05", day: "27", hour: "07", minute: "32", second: "00", fraction: "", offset: TimeOffset::Time(TimeOffsetAmount {
                  pos_neg: "-", hour: "08", minute: "00"
                })
              })
            }),
            table: None, comment: Some(Comment {
              text: " Fïřƨƭ çℓáƨƨ δáƭèƨ"
            })
          }
        },
        NLExpression {
          nl: "\n", expr: Expression {
            ws: WSSep { ws1: "", ws2: "" }, keyval: None, table: None, comment: None
          }
        },
        NLExpression {
          nl: "\n", expr: Expression {
            ws: WSSep { ws1: "", ws2: "" }, keyval: None, table: Some(TableType::Standard(Table {
              ws: WSSep { ws1: "", ws2: "" }, key: "database", subkeys: vec![]
            })), comment: None
          }
        },
        NLExpression {
          nl: "\n", expr: Expression {
            ws: WSSep { ws1: "", ws2: "" }, keyval: Some(KeyVal {
              key: "server", keyval_sep: WSSep { ws1: " ", ws2: " "}, val: Value::String("192.168.1.1", StrType::Basic)
            }),
            table: None, comment: None
          }
        },
        NLExpression {
          nl: "\n", expr: Expression {
            ws: WSSep { ws1: "", ws2: "" }, keyval: Some(KeyVal {
              key: "ports", keyval_sep: WSSep { ws1: " ", ws2: " " }, val: Value::Array(Box::new(Array {
                values: vec![
                  ArrayValue {
                    val: Value::Integer("8001"), array_sep: Some(WSSep { ws1: "", ws2: " " }), comment_nl: None
                  },
                  ArrayValue {
                    val: Value::Integer("8001"), array_sep: Some(WSSep { ws1: "", ws2: " " }), comment_nl: None
                  },
                  ArrayValue {
                    val: Value::Integer("8002"), array_sep: None, comment_nl: None
                  }
                ],
                ws: WSSep { ws1: " ", ws2: " " }
              }))
            }),
            table: None, comment: None
          }
        },
        NLExpression {
          nl: "\n", expr: Expression {
            ws: WSSep { ws1: "", ws2: "" }, keyval: Some(KeyVal {
              key: "connection_max", keyval_sep: WSSep { ws1: " ", ws2: " " }, val: Value::Integer("5000")
            }),
            table: None, comment: None
          }
        },
        NLExpression {
          nl: "\n", expr: Expression {
            ws: WSSep { ws1: "", ws2: "" }, keyval: Some(KeyVal {
              key: "enabled", keyval_sep: WSSep { ws1: " ", ws2: " " }, val: Value::Boolean("true")
            }),
            table: None, comment: None
          }
        }
      ]} 
    ));
  }

  #[test]
  fn test_nl_expressions() {
    let p = new();
    // allow for zero expressions
    assert_eq!(p.nl_expressions("aoeunth £ôřè₥ ïƥƨú₥ doℓôř ƨïƭ amet, çônƨèçƭeƭuř áδïƥïscïñϱ èℓïƭ"),
      Done(
        "aoeunth £ôřè₥ ïƥƨú₥ doℓôř ƨïƭ amet, çônƨèçƭeƭuř áδïƥïscïñϱ èℓïƭ", vec![]
      )
    );
    assert_eq!(p.nl_expressions("\n[\"δřá\"]#Mèƨsaϱè\r\nkey=\"value\"#wλïƭeƨƥáçè\n"),
      Done(
        "", vec![
          NLExpression{
            nl: "\n", expr: Expression{
              ws: WSSep{ws1: "", ws2: ""},
              keyval: None,
              table: Some(TableType::Standard(Table{
                ws: WSSep{ws1: "", ws2: ""}, key: "\"δřá\"", subkeys: vec![]
              })),
              comment: Some(Comment{text: "Mèƨsaϱè"})
            }
          },
          NLExpression{
            nl: "\r\n", expr: Expression{
              ws: WSSep{ws1: "", ws2: ""},
              table: None,
              keyval: Some(KeyVal{
                key: "key", keyval_sep: WSSep{
                  ws1: "", ws2: ""
                },
                val: Value::String("value", StrType::Basic)
              }),
              comment: Some(Comment{text: "wλïƭeƨƥáçè"})
            }
          },
          // A whitespace expression only requires a newline, and a newline is required to terminate the comment
          // of the previous expression so expressions ending in comments always end up with an extra whitespace
          // expression at the end of the list
          // The exceptions are for characters that end comments, but are not "newlines". It's something that
          // needs to be fixed in the ABNF
          NLExpression { 
            nl: "\n", expr: Expression { 
              ws: WSSep { 
                ws1: "", ws2: ""
              },
              keyval: None, table: None, comment: None
            }
          }
        ]
      )
    );
    assert_eq!(p.nl_expressions("\n[[NODOTNET.\"NÓJÂVÂ\"]]"),
      Done(
        "", vec![
          NLExpression{
            nl: "\n", expr: Expression{
              ws: WSSep{ws1: "", ws2: ""},
              keyval: None,
              table: Some(TableType::Array(Table{
                ws: WSSep{ws1: "", ws2: ""}, key: "NODOTNET", subkeys: vec![
                  WSKeySep{ws: WSSep{ws1: "", ws2: ""}, key: "\"NÓJÂVÂ\""}
                ]
              })),
              comment: None
            }
          }
        ]
      )
    );
  }
// named!(nl_expression<&'a str, NLExpression>,
  #[test]
  fn test_nl_expression() {
    let p = new();
    assert_eq!(p.nl_expression("\r\n   SimpleKey = 1_2_3_4_5     #  áñ áƭƭè₥ƥƭ ƭô δèƒïñè TÓM£\r\n"),
      Done("\r\n", NLExpression{
        nl: "\r\n", expr: Expression{
          ws: WSSep{ws1: "   ", ws2: "     "},
          table: None,
          keyval: Some(KeyVal{
            key: "SimpleKey", keyval_sep: WSSep{
              ws1: " ", ws2: " "
            },
            val: Value::Integer("1_2_3_4_5")
          }),
          comment: Some(Comment{text: "  áñ áƭƭè₥ƥƭ ƭô δèƒïñè TÓM£"})
        } 
      })
    );
  }

  #[test]
  fn test_expression() {
    let p = new();
    assert_eq!(p.expression(" \t[\"δřáƒƭ\".THISKEY  . \tkeythethird] \t#Mèƨƨáϱè Rèƥℓïèδ\n"),
      Done("\n",
        Expression{
          ws: WSSep{ws1: " \t", ws2: " \t"},
          keyval: None,
          table: Some(TableType::Standard(Table{
            ws: WSSep{ws1: "", ws2: ""}, key: "\"δřáƒƭ\"", subkeys: vec![
              WSKeySep{ws: WSSep{ws1: "", ws2: ""}, key: "THISKEY"},
              WSKeySep{ws: WSSep{ws1: "  ", ws2: " \t"}, key: "keythethird"}
            ]
          })),
          comment: Some(Comment{text: "Mèƨƨáϱè Rèƥℓïèδ"})
        }
    ));
    assert_eq!(p.expression("\t\t\t\"řúññïñϱôúƭôƒωôřδƨ\" = 0.1  #Â₥èřïçáñ Éжƥřèƨƨ\n"),
      Done("\n",
        Expression{
          ws: WSSep{ws1: "\t\t\t", ws2: "  "},
          table: None,
          keyval: Some(KeyVal{
            key: "\"řúññïñϱôúƭôƒωôřδƨ\"", keyval_sep: WSSep{
              ws1: " ", ws2: " "
            },
            val: Value::Float("0.1")
          }),
          comment: Some(Comment{text: "Â₥èřïçáñ Éжƥřèƨƨ"})
        }
      ));
    assert_eq!(p.expression("\t \t #Þℓèáƨè Ʋèřïƒ¥ Your áççôúñƭ\n"), Done("\n",
      Expression{
        ws: WSSep{ws1: "\t \t ", ws2: ""},
        keyval: None,
        table: None,
        comment: Some(Comment{text: "Þℓèáƨè Ʋèřïƒ¥ Your áççôúñƭ"})
      }
    ));
    assert_eq!(p.expression("\t  \t  \t\n"), Done("\n",
      Expression{
        ws: WSSep{
          ws1: "\t  \t  \t",
          ws2: "",
        },
        keyval: None, table: None, comment: None,
      }));
  }

  #[test]
  fn test_ws_expr() {
    let p = new();
    assert_eq!(p.ws_expr("  \t \t \n"), Done("\n", 
      Expression{
        ws: WSSep{
          ws1: "  \t \t ",
          ws2: "",
        },
        keyval: None, table: None, comment: None,
      }
    ));
  }

  #[test]
  fn test_table_comment() {
    let p = new();
    assert_eq!(p.table_comment(" [table.\"ƭáβℓè\"] #úñïçôřñřôβôƭ\n"),
      Done("\n",
        Expression{
          ws: WSSep{ws1: " ", ws2: " "},
          keyval: None,
          table: Some(TableType::Standard(Table{
            ws: WSSep{ws1: "", ws2: ""}, key: "table", subkeys: vec![
              WSKeySep{ws: WSSep{ws1: "", ws2: ""}, key: "\"ƭáβℓè\""}
            ]
          })),
          comment: Some(Comment{text: "úñïçôřñřôβôƭ"})
        }
    ));
  }

  #[test]
  fn test_keyval_comment() {
    let p = new();
    assert_eq!(p.keyval_comment(" \"Tôƭáℓℓ¥\" = true\t#λèřè ïƨ ₥¥ çô₥₥èñƭ\n"),
      Done("\n",
        Expression{
          ws: WSSep{ws1: " ", ws2: "\t"},
          table: None,
          keyval: Some(KeyVal{
            key: "\"Tôƭáℓℓ¥\"", keyval_sep: WSSep{
              ws1: " ", ws2: " "
            },
            val: Value::Boolean("true")
          }),
          comment: Some(Comment{text: "λèřè ïƨ ₥¥ çô₥₥èñƭ"})
        }
    ));
  }

  #[test]
  fn test_ws_comment() {
    let p = new();
    assert_eq!(p.ws_comment(" \t #This is RÂNÐÓM §TRÌNG\n"), Done("\n",
      Expression{
        ws: WSSep{ws1: " \t ", ws2: ""},
        keyval: None,
        table: None,
        comment: Some(Comment{text: "This is RÂNÐÓM §TRÌNG"})
      }
    ));
  }
}
