use ast::structs::{TableType, WSKeySep, Table, CommentNewLines,
                   CommentOrNewLines, ArrayValue, Array, Value,
                   InlineTable, WSSep, TableKeyVal, ArrayType,
                   HashValue, format_tt_keys};
use parser::Parser;
use types::{ParseError, Str};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use nomplusplus::IResult;

#[inline(always)]
fn map_val_to_array_type(&Value) -> ArrayType {
  match val {
    &Value::Integer(_)        => ArrayType::Integer,
    &Value::Float(_)          => ArrayType::Float,
    &Value::Boolean(_)        => ArrayType::Boolean,
    &Value::DateTime(_)       => ArrayType::DateTime,
    &Value::Array(_)          => ArrayType::Array,
    &Value::String(_,_)       => ArrayType::String,
  }
}

impl<'a> Parser<'a> {
  // Table
  method!(pub table<Parser<'a>, &'a str, Rc<TableType> >, mut self,
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
      call_m!(self.ws)         ~
      key: call_m!(self.key)        ,
      ||{
        WSKeySep::new_str(WSSep::new_str(ws1, ws2), key)
      } 
    )
  );
  // Standard Table
  method!(std_table<Parser<'a>, &'a str, Rc<TableType> >, mut self,
    chain!(
           tag_s!("[")    ~
      ws1: call_m!(self.ws)             ~
      key: call_m!(self.key)            ~
  subkeys: call_m!(self.table_subkeys)  ~
      call_m!(self.ws)             ~
           tag_s!("]")    ,
      ||{
        let res = Rc::new(TableType::Standard(Table::new_str(
          WSSep::new_str(ws1, ws2), key, subkeys
        )));
        if self.last_array_tables.borrow().len() > 0 {
          let last_table = &*self.last_array_tables.borrow()[self.last_array_tables.borrow().len() - 1];
          if !res.is_subtable_of(last_table) ||
            format_tt_keys(&res) == format_tt_keys(last_table) {
            self.errors.borrow_mut().push(ParseError::InvalidTable (
              format_tt_keys(&res), RefCell::new(HashMap::new())
            ));
            self.array_error.set(true);
          } else {
            self.array_error.set(false);
          }
          let full_key = format_tt_keys(&res);
          if !self.map.contains_key(&full_key) {
            self.map.insert(full_key, HashValue::none());
          }
        }
        self.last_table = Some(res.clone());
        res
      }
    )
  );

  //Array Table
  method!(array_table<Parser<'a>, &'a str, Rc<TableType> >, mut self,
    chain!(
           tag_s!("[[")   ~
      ws1: call_m!(self.ws)             ~
      key: call_m!(self.key)            ~
  subkeys: call_m!(self.table_subkeys)  ~
      call_m!(self.ws)             ~
           tag_s!("]]")   ,
      ||{
        let res = Rc::new(TableType::Array(Table::new_str(
          WSSep::new_str(ws1, ws2), key, subkeys
        )));
        self.array_error.set(false);
        let len = self.last_array_tables.borrow().len();
        if len > 0 {
          let len = self.last_array_tables.borrow().len();
          let last_key = format_tt_keys(&self.last_array_tables.borrow()[len - 1]);
          if format_tt_keys(&*res) == last_key {
            let last_index = self.last_array_tables_index.borrow()[len - 1];
            self.last_array_tables_index.borrow_mut()[len - 1] = last_index + 1;
          } else {
            let subtable = res.is_subtable_of(&self.last_array_tables.borrow()[len - 1]);
            if subtable {
              self.last_array_tables.borrow_mut().push(res.clone());
              self.last_array_tables_index.borrow_mut().push(0);
            } else {
              self.last_array_tables.borrow_mut().clear();
              self.last_array_tables.borrow_mut().push(res.clone());
              self.last_array_tables_index.borrow_mut().clear();
              self.last_array_tables_index.borrow_mut().push(0);
            }
          }
        }else {
          self.last_array_tables.borrow_mut().push(res.clone());
          self.last_array_tables_index.borrow_mut().push(0);
        }
        let full_key = format_tt_keys(&*res);
        let contains_key = self.map.contains_key(&full_key);
        if !contains_key {
          self.map.insert(full_key, HashValue::none());
        }
        self.last_table = Some(res.clone());
        res
      }
    )
  );



  // Array
  method!(array_sep<Parser<'a>, &'a str, WSSep>, mut self,
    chain!(
      ws1: call_m!(self.ws)         ~
           tag_s!(",")~
      call_m!(self.ws)         ,
      ||{
        WSSep::new_str(ws1, ws2)
      }
    )
  );

  method!(ws_newline<Parser<'a>, &'a str, &'a str>, self, re_find!("^( |\t|\n|(\r\n))*"));

  method!(comment_nl<Parser<'a>, &'a str, CommentNewLines>, mut self,
    chain!(
   prewsnl: call_m!(self.ws_newline)  ~
   comment: call_m!(self.comment)     ~
  newlines: call_m!(self.ws_newline) ,
      ||{
        CommentNewLines::new_str(prewsnl, comment, newlines)
      }
    )
  );

  method!(comment_or_nl<Parser<'a>, &'a str, CommentOrNewLines>, mut self,
    alt!(
      complete!(call_m!(self.comment_nl))   => {|com| CommentOrNewLines::Comment(com)} |
      complete!(call_m!(self.ws_newline))  => {|nl|  CommentOrNewLines::NewLines(Str::Str(Str::Str(nl))}
    )
  );

  method!(comment_or_nls<Parser<'a>, &'a str, Vec<CommentOrNewLines> >, mut self,
    many1!(call_m!(self.comment_or_nl)));
  
  // TODO: Redo this with array_sep wrapped in a complete!() ?
  method!(array_value<Parser<'a>, &'a str, ArrayValue>, mut self,
    alt!(
      complete!(
        chain!(
          call_m!(self.val)                        ~
    call_m!(self.array_sep)                  ~
   complete!(call_m!(self.comment_or_nls))   ,
          ||{
            let t = map_val_to_array_type(&val);
            let len = self.last_array_type.borrow().len();
            if len > 0 && self.last_array_type.borrow()[len - 1] != ArrayType::None &&
               self.last_array_type.borrow()[len - 1] != t {
              self.mixed_array.set(true);
            }
            self.last_array_type.borrow_mut().pop();
            self.last_array_type.borrow_mut().push(t);
            ArrayValue{
              val,
              Some(array_sep),
              comment_nls,
            }
          }
        )
      ) |
      complete!(
        chain!(
          call_m!(self.val)                       ~
  complete!(call_m!(self.comment_or_nls)) ,
          ||{
            let t = map_val_to_array_type(&val);
            let len = self.last_array_type.borrow().len();
            if len > 0 && self.last_array_type.borrow()[len - 1] != ArrayType::None &&
               self.last_array_type.borrow()[len - 1] != t {
              self.mixed_array.set(true);
            }
            self.last_array_type.borrow_mut().pop();
            self.last_array_type.borrow_mut().push(t);
            ArrayValue{
              val,
              None,
              comment_nls,
            }
          }
        )
      )
    )
  );

  method!(array_values<Parser<'a>, &'a str, Vec<ArrayValue> >, mut self,
    chain!(
     vals: many0!(call_m!(self.array_value)) ,
     ||{
        let mut tmp = vec![];
        tmp.extend(vals);
        tmp
      }
    )
  );

  pub fn array(mut self: Parser<'a>, input: &'a str) -> (Parser<'a>, IResult<&'a str, Rc<Array>>) {
    // Initialize last array type to None, we need a stack because arrays can be nested
    self.last_array_type.borrow_mut().push(ArrayType::None);
    let (tmp, res) = self.array_internal(input);
    self = tmp; // Restore self
    self.last_array_type.borrow_mut().pop();
    (self, res)
  }

  method!(pub array_internal<Parser<'a>, &'a str, Rc<Array> >, mut self,
    chain!(
              tag_s!("[")                   ~
         cn1: call_m!(self.comment_or_nls)  ~
  array_vals: call_m!(self.array_values)    ~
         cn2: call_m!(self.comment_or_nls)  ~
              tag_s!("]")                   ,
      ||{
       let array_result = Rc::new(Array{
          array_vals,
          comment_nls1: cn1,
          comment_nls2: cn2,
        });
        if self.mixed_array.get() {
          self.mixed_array.set(false);
          let mut vals: Vec<Rc<Value<'a>>> = vec![]; 
          for x in 0..array_result.values.len() {
            vals.push(array_result.values[x].val.clone());
          }
          self.errors.borrow_mut().push(ParseError::MixedArray(vals));
        }
        array_result
      }
    )
  );

  method!(table_keyval<Parser<'a>, &'a str, TableKeyVal>, mut self,
        chain!(
          ws1: call_m!(self.ws)     ~
       keycall_m!(self.keyval) ~
          call_m!(self.ws)     ,
          ||{
            TableKeyVal::new(keyval, WSSep::new_str(ws1, ws2))
          }
        )
  );

  method!(inline_table_keyvals_non_empty<Parser<'a>, &'a str, Vec<TableKeyVal> >, mut self, separated_list!(tag_s!(","), call_m!(self.table_keyval)));

  method!(pub inline_table<Parser<'a>, &'a str, InlineTable>, mut self,
    chain!(
           tag_s!("{")                                ~
      ws1: call_m!(self.ws)                                         ~
  keyvals: complete!(call_m!(self.inline_table_keyvals_non_empty))? ~
      call_m!(self.ws)                                         ~
           tag_s!("}")                                ,
          ||{
            InlineTable::new(keyvals, WSSep::new_str(ws1, ws2))
          }
    )
  );
}

#[cfg(test)]
mod test {
  use nomplusplus::IResult::Done;
  use ast::structs::{Array, ArrayValue, WSSep, TableKeyVal, InlineTable, WSKeySep,
                     KeyVal, CommentNewLines, Comment, CommentOrNewLines, Table,
                     TableType, Value};
  use ::types::{DateTime, TimeOffset, TimeOffsetAmount, StrType};
  use parser::Parser;
  use std::rc::Rc;

  #[test]
  fn test_table() {
    let mut p = Parser::new();
    assert_eq!(p.table("[ _underscore_ . \"-δáƨλèƨ-\" ]").1, Done("",
      Rc::new(TableType::Standard(Table::new_str(
        WSSep::new_str(" ", " "), "_underscore_", vec![
          WSKeySep::new_str(WSSep::new_str(" ", " "), "\"-δáƨλèƨ-\"")
        ]
      ))
    )));
    p = Parser::new();
    assert_eq!(p.table("[[\t NumberOne\t.\tnUMBERtWO \t]]").1, Done("",
      Rc::new(TableType::Array(Table::new_str(
        WSSep::new_str("\t ", " \t"), "NumberOne", vec![
          WSKeySep::new_str(WSSep::new_str("\t", "\t"), "nUMBERtWO")
        ]
      ))
    )));
  }

  #[test]
  fn test_table_subkey() {
    let p = Parser::new();
    assert_eq!(p.table_subkey("\t . \t\"áƭúƨôèλôñèƭúññèôúñôèƭú\"").1, Done("",
      WSKeySep::new_str(WSSep::new_str("\t ", " \t"), "\"áƭúƨôèλôñèƭúññèôúñôèƭú\""),
    ));
  }

  #[test]
  fn test_table_subkeys() {
    let p = Parser::new();
    assert_eq!(p.table_subkeys(" .\tAPPLE.MAC . \"ßÓÓK\"").1, Done("",
      vec![
        WSKeySep::new_str(WSSep::new_str(" ", "\t"), "APPLE"),
        WSKeySep::new_str(WSSep::new_str("", ""), "MAC"),
        WSKeySep::new_str(WSSep::new_str(" ", " "), "\"ßÓÓK\"")
      ]
    ));
  }

  #[test]
  fn test_std_table() {
    let p = Parser::new();
    assert_eq!(p.std_table("[Dr-Pepper  . \"ƙè¥_TWÓ\"]").1, Done("",
      Rc::new(TableType::Standard(Table::new_str(
        WSSep::new_str("", ""), "Dr-Pepper", vec![
          WSKeySep::new_str(WSSep::new_str("  ", " "), "\"ƙè¥_TWÓ\"")
        ]
      )))
    ));
  }

  #[test]
  fn test_array_table() {
    let p = Parser::new();
    assert_eq!(p.array_table("[[\"ƙè¥ôñè\"\t. key_TWO]]").1, Done("",
      Rc::new(TableType::Array(Table::new_str(
        WSSep::new_str("", ""), "\"ƙè¥ôñè\"", vec![
          WSKeySep::new_str(WSSep::new_str("\t", " "), "key_TWO")
        ]
      ))
    )));
  }

  #[test]
  fn test_array_sep() {
    let p = Parser::new();
    assert_eq!(p.array_sep("  ,  ").1, Done("", WSSep::new_str("  ", "  ")));
  }

  #[test]
  fn test_ws_newline() {
    let p = Parser::new();
    assert_eq!(p.ws_newline("\t\n\n").1, Done("", "\t\n\n"));
  }

  #[test]
  fn test_comment_nl() {
    let p = Parser::new();
    assert_eq!(p.comment_nl("\r\n\t#çô₥₥èñƭñèωℓïñè\n \n \n").1, Done("",
      CommentNewLines::new_str(
        "\r\n\t", Comment::new_str("çô₥₥èñƭñèωℓïñè"), "\n \n \n"
      )
    ));
  }

  #[test]
  fn test_comment_or_nl() {
    let mut p = Parser::new();
    assert_eq!(p.comment_or_nl("#ωôřƙωôřƙ\n").1, Done("",
      CommentOrNewLines::Comment(CommentNewLines::new_str(
        "", Comment::new_str("ωôřƙωôřƙ"), "\n"
      ))
    ));
    p = Parser::new();
    assert_eq!(p.comment_or_nl(" \t\n#ωôřƙωôřƙ\n \r\n").1, Done("",
      CommentOrNewLines::Comment(CommentNewLines::new_str(
        " \t\n", Comment::new_str("ωôřƙωôřƙ"), "\n \r\n"
      ))
    ));
    p = Parser::new();
    assert_eq!(p.comment_or_nl("\n\t\r\n ").1, Done("", CommentOrNewLines::NewLines(Str::Str(Str::Str("\n\t\r\n "))));
  }

  #[test]
  fn test_array_value() {
    let mut p = Parser::new();
    assert_eq!(p.array_value("54.6, \n#çô₥₥èñƭ\n\n").1,
      Done("",ArrayValue::new(
        Rc::new(Value::Float(Str::Str("54.6"))), Some(WSSep::new_str("", " ")),
        vec![CommentOrNewLines::Comment(CommentNewLines::new_str(
          "\n", Comment::new_str("çô₥₥èñƭ"), "\n\n"
        ))]
      ))
    );
    p = Parser::new();
    assert_eq!(p.array_value("\"ƨƥáϱλèƭƭï\"").1,
      Done("",ArrayValue::new(
        Rc::new(Value::String(Str::Str("ƨƥáϱλèƭƭï"), StrType::Basic)), None, vec![CommentOrNewLines::NewLines(Str::Str(Str::Str(""))]
      ))
    );
    p = Parser::new();
    assert_eq!(p.array_value("44_9 , ").1,
      Done("",ArrayValue::new_str(
        Rc::new(Value::Integer(Str::Str("44_9"))), Some(WSSep::new_str(" ", " ")),
        vec![CommentOrNewLines::NewLines(Str::Str(Str::Str(""))]
      ))
    );
  }

  #[test]
  fn test_array_values() {
    let mut p = Parser::new();
    assert_eq!(p.array_values("1, 2, 3").1, Done("", vec![
      ArrayValue::new(Rc::new(Value::Integer(Str::Str("1"))), Some(WSSep::new_str("", " ")),
      vec![CommentOrNewLines::NewLines(Str::Str(Str::Str(""))]),
      ArrayValue::new_str(Rc::new(Value::Integer(Str::Str("2"))), Some(WSSep::new_str("", " ")),
      vec![CommentOrNewLines::NewLines(Str::Str(Str::Str(""))]),
      ArrayValue::new(Rc::new(Value::Integer(Str::Str("3"))), None, vec![CommentOrNewLines::NewLines(Str::Str(Str::Str(""))])
    ]));
    p = Parser::new();
    assert_eq!(p.array_values("1, 2, #çô₥₥èñƭ\n3, ").1, Done("", vec![
      ArrayValue::new(Rc::new(Value::Integer(Str::Str("1"))), Some(WSSep::new_str("", " ")),
      vec![CommentOrNewLines::NewLines(Str::Str(Str::Str(""))]),
      ArrayValue::new(Rc::new(Value::Integer(Str::Str("2"))), Some(WSSep::new_str("", " ")),
        vec![CommentOrNewLines::Comment(CommentNewLines::new_str("", Comment::new_str("çô₥₥èñƭ"), "\n"))]),
      ArrayValue::new(Rc::new(Value::Integer(Str::Str("3"))), Some(WSSep::new_str("", " ")),
      vec![CommentOrNewLines::NewLines(Str::Str(Str::Str(""))])
    ]));
  }

  #[test]
  fn test_non_nested_array() {
    let p = Parser::new();
    assert_eq!(p.array("[2010-10-10T10:10:10.33Z, 1950-03-30T21:04:14.123+05:00]").1,
      Done("", Rc::new(Array::new(
        vec![ArrayValue::new(
          Rc::new(Value::DateTime(DateTime::new_str("2010", "10", "10", "10", "10", "10", Some(Str::Str("33")),
            TimeOffset::Z
          ))),
          Some(WSSep::new_str("", " ")),
          vec![CommentOrNewLines::NewLines(Str::Str(Str::Str(""))]
        ),
        ArrayValue::new(
          Rc::new(Value::DateTime(DateTime::new_str("1950", "03", "30", "21", "04", "14", Some(Str::Str("123")),
            TimeOffset::Time(TimeOffsetAmount::new_str("+", "05", "00"))
          ))),
          None, vec![CommentOrNewLines::NewLines(Str::Str(Str::Str(""))]
        )],
        vec![CommentOrNewLines::NewLines(Str::Str(Str::Str(""))], vec![CommentOrNewLines::NewLines(Str::Str(Str::Str(""))]
      )))
    );
  }

  #[test]
  fn test_nested_array() {
    let p = Parser::new();
    assert_eq!(p.array("[[3,4], [4,5], [6]]").1,
      Done("", Rc::new(Array::new(
        vec![
          ArrayValue::new(
            Rc::new(Value::Array(Rc::new(Array::new(
              vec![
                ArrayValue::new(
                  Rc::new(Value::Integer(Str::Str("3"))), Some(WSSep::new_str("", "")),
                  vec![CommentOrNewLines::NewLines(Str::Str(Str::Str(""))]
                ),
                ArrayValue::new(
                  Rc::new(Value::Integer(Str::Str("4"))), None, vec![CommentOrNewLines::NewLines(Str::Str(Str::Str(""))]
                )
              ],
              vec![CommentOrNewLines::NewLines(Str::Str(Str::(""))], vec![CommentOrNewLines::NewLines(Str::Str(Str::Str(""))]
            )))),
            Some(WSSep::new_str("", " ")),
            vec![CommentOrNewLines::NewLines(Str::Str(Str::Str(""))]
          },
          ArrayValue::new(
            Rc::new(Value::Array(Rc::new(Array::new(
              vec![
                ArrayValue::new(
                  Rc::new(Value::Integer(Str::Str("4")))), Some(WSSep::new_str("", ""),
                  vec![CommentOrNewLines::NewLines(Str::Str(Str::Str(""))]
                ),
                ArrayValue::new(
                    Rc::new(Value::Integer(Str::Str("5"))), None, vec![CommentOrNewLines::NewLines(Str::Str(""))]
                )
              ],
              comment_nls1: vec![CommentOrNewLines::NewLines(Str::Str(""))], comment_nls2: vec![CommentOrNewLines::NewLines(Str::Str(""))]
            }))),
            Some(WSSep::new_str("", " ")),
            vec![CommentOrNewLines::NewLines(Str::Str(""))]
          },
          ArrayValue::new(
            Rc::new(Value::Array(Rc::new(Array::new(
              vec![
                ArrayValue::new(
                  Rc::new(Value::Integer(Str::Str("6"))), None, vec![CommentOrNewLines::NewLines(Str::Str(""))]
                }
              ],
             vec![CommentOrNewLines::NewLines(Str::Str(""))], vec![CommentOrNewLines::NewLines(Str::Str(""))]
            )))),
            None, vec![CommentOrNewLines::NewLines(Str::Str(""))]
          }
        ],
        vec![CommentOrNewLines::NewLines(Str::Str(""))], vec![CommentOrNewLines::NewLines(Str::Str(""))]
      }))
    );
  }

  #[test]
  fn test_table_keyval() {
    let p = Parser::new();
    assert_eq!(p.table_keyval("\"Ì WúƲ Húϱƨ!\"\t=\t'Mè ƭôô!' ").1, Done("", TableKeyVal::new(
      KeyVal::new_str(
        key: "\"Ì WúƲ Húϱƨ!\"", Rc::new(Value::String("Mè ƭôô!", StrType::Literal)), keyval_sep: WSSep::new_str(
          ws1: "\t", "\t"
        }
      },
      kv_sep: WSSep::new_str(
        ws1: "", " "
      },
    }));
  }

  #[test]
  fn test_inline_table_keyvals_non_empty() {
    let p = Parser::new();
    assert_eq!(p.inline_table_keyvals_non_empty(" Key =\t54,\"Key2\" = '34.99'\t").1,
      Done("", vec![
        TableKeyVal::new(
          KeyVal::new_str(
            "Key", WSSep::new_str(" ", "\t"),
            Rc::new(Value::Integer("54"))
          ),
          WSSep::new_str{" ", "")
        ),
        TableKeyVal::new(
          KeyVal::new_str(
            "\"Key2\"", WSSep::new_str( " ", " "),
            Rc::new(Value::String(Str::Str("34.99"), StrType::Literal))
          ),
          WSSep::new_str("", "\t")
        )
      ])
    );
  }

  #[test]
  fn test_inline_table() {
    let p = Parser::new();
    assert_eq!(p.inline_table("{\tKey = 3.14E+5 , \"Key2\" = '''New\nLine'''\t}").1,
      Done("", InlineTable::new(
        Some(vec![
          TableKeyVal::new(
            KeyVal::new_str(
              "Key", WSSep::new_str(" ", " "),
              Rc::new(Value::Float(Str::Str("3.14E+5")))
            ),
            WSSep::new_str("", " ")
          ),
          TableKeyVal::new(
            KeyVal::new_str("\"Key2\"", WSSep::new_str(" ", " "),
              Rc::new(Value::String(Str::Str("New\nLine"), StrType::MLLiteral))
            ),
            WSSep::new_str(" ", "\t")
          )
        ]),
        WSSep::new_str("\t", "")
      ))
    );
  }
}
