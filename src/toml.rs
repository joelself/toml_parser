use ast::structs::{Toml, NLExpression, Expression, WSSep};
use util::{newline, ws, comment};
use objects::{table};
use primitives::{keyval};

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
    ws1: ws       ~
  table: table    ~
    ws2: ws       ~
comment: comment  ? ,
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
comment: comment? ,
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
  use super::{ws_comment, keyval_comment, table_comment};
  use ast::structs::{Expression, Comment, WSSep, KeyVal, Table, WSKeySep,
                     TableType, Value};
// named!(pub toml<&str, Toml>,
// named!(nl_expressions<&str, Vec<NLExpression> >,

// named!(nl_expression<&str, NLExpression>,
// // Expression
// named!(pub expression<&str,  Expression>,
// named!(ws_expr<&str, Expression>,
// named!(pub table_comment<&str, Expression>,
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
