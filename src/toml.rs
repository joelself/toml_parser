use ast::structs::{Toml, NLExpression, Expression, WSSep};
use util::{newline, ws, comment};
use objects::{table};
use primitives::{keyval};
use nom::eof;

named!(pub toml<&str, Toml>,
  chain!(
    expr: expression    ~
nl_exprs: nl_expressions,
    ||{
      let mut tmp = vec![NLExpression{ nl: "", expr: expr}];
      tmp.extend(nl_exprs); Toml{ exprs: tmp}
    }
  )
);

named!(nl_expressions<&str, Vec<NLExpression> >, many0!(nl_expression));

named!(nl_expression<&str, NLExpression>,
  chain!(
     nl: newline    ~
   expr: expression ,
    ||{
      NLExpression{
        nl: nl, expr: expr,
      }
    }
  )
);

// Expression
named!(pub expression<&str,  Expression>,
  alt!(
    complete!(table_comment)  |
    complete!(keyval_comment) |
    complete!(ws_comment)     |
    complete!(ws_expr)
  )
);

named!(ws_expr<&str, Expression>,
  chain!(
    ws: ws,
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

named!(pub table_comment<&str, Expression>,
  chain!(
    ws1: ws                 ~
  table: table              ~
    ws2: ws                 ~
comment: complete!(comment)?,
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

named!(pub keyval_comment<&str, Expression>,
  chain!(
    ws1: ws       ~
 keyval: keyval   ~
    ws2: ws       ~
comment: complete!(comment) ? ,
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

named!(ws_comment<&str, Expression>,
  chain!(
     ws: ws     ~
comment: comment,
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

#[cfg(test)]
mod test {
  use nom::IResult::Done;
  use super::{ws_comment, keyval_comment, table_comment, expression, ws_expr,
              nl_expression, nl_expressions};
  use ast::structs::{Expression, Comment, WSSep, KeyVal, Table, WSKeySep,
                     TableType, Value, NLExpression, StrType};
// named!(pub toml<&str, Toml>,
  #[test]
  fn test_nl_expressions() {
    // allow for zero expressions
    assert_eq!(nl_expressions("aoeunth £ôřè₥ ïƥƨú₥ doℓôř ƨïƭ amet, çônƨèçƭeƭuř áδïƥïscïñϱ èℓïƭ"),
      Done(
        "aoeunth £ôřè₥ ïƥƨú₥ doℓôř ƨïƭ amet, çônƨèçƭeƭuř áδïƥïscïñϱ èℓïƭ", vec![]
      )
    );
    assert_eq!(nl_expressions("\n[\"δřá\"]#Mèƨsaϱè\r\nkey=\"value\"#wλïƭeƨƥáçè\n"),
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
    assert_eq!(nl_expressions("\n[[NODOTNET.\"NÓJÂVÂ\"]]"),
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
// named!(nl_expression<&str, NLExpression>,
  #[test]
  fn test_nl_expression() {
    assert_eq!(nl_expression("\r\n   SimpleKey = 1_2_3_4_5     #  áñ áƭƭè₥ƥƭ ƭô δèƒïñè TÓM£\r\n"),
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
    assert_eq!(expression(" \t[\"δřáƒƭ\".THISKEY  . \tkeythethird] \t#Mèƨƨáϱè Rèƥℓïèδ\n"),
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
    assert_eq!(expression("\t\t\t\"řúññïñϱôúƭôƒωôřδƨ\" = 0.1  #Â₥èřïçáñ Éжƥřèƨƨ\n"),
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
    assert_eq!(expression("\t \t #Þℓèáƨè Ʋèřïƒ¥ Your áççôúñƭ\n"), Done("\n",
      Expression{
        ws: WSSep{ws1: "\t \t ", ws2: ""},
        keyval: None,
        table: None,
        comment: Some(Comment{text: "Þℓèáƨè Ʋèřïƒ¥ Your áççôúñƭ"})
      }
    ));
    assert_eq!(expression("\t  \t  \t\n"), Done("\n",
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
    assert_eq!(ws_expr("  \t \t \n"), Done("\n", 
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
    assert_eq!(table_comment(" [table.\"ƭáβℓè\"] #úñïçôřñřôβôƭ\n"),
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
    assert_eq!(keyval_comment(" \"Tôƭáℓℓ¥\" = true\t#λèřè ïƨ ₥¥ çô₥₥èñƭ\n"),
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
    assert_eq!(ws_comment(" \t #This is RÂNÐÓM §TRÌNG\n"), Done("\n",
      Expression{
        ws: WSSep{ws1: " \t ", ws2: ""},
        keyval: None,
        table: None,
        comment: Some(Comment{text: "This is RÂNÐÓM §TRÌNG"})
      }
    ));
  }
}
