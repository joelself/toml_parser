use ast::structs::{Time, FullDate, KeyVal, WSSep, Value, StrType, ErrorCode};
use ::types::{DateTime, TimeOffset, TimeOffsetAmount};
use parser::{Parser, count_lines};
use nomplusplus;
use nomplusplus::{IResult, InputLength};

fn is_keychar(chr: char) -> bool {
  let uchr = chr as u32;
  uchr >= 0x41 && uchr <= 0x5A || // A-Z
  uchr >= 0x61 && uchr <= 0x7A || // a-z
  uchr >= 0x30 && uchr <= 0x39 || // 0-9
  uchr == 0x2D || uchr == 0x5f    // "-", "_"
}

impl<'a> Parser<'a> {
  // Integer
  method!(integer<Parser<'a>, &'a str, &'a str>, self, re_find!("^((\\+|-)?(([1-9](\\d|(_\\d))+)|\\d))")) ;

  // Float
  method!(float<Parser<'a>, &'a str, &'a str>, self,
         re_find!("^(\\+|-)?([1-9](\\d|(_\\d))+|\\d)((\\.\\d(\\d|(_\\d))*)((e|E)(\\+|-)?([1-9](\\d|(_\\d))+|\\d))|(\\.\\d(\\d|(_\\d))*)|((e|E)(\\+|-)?([1-9](\\d|(_\\d))+|\\d)))"));

  // String
  // TODO: method!(string<&'a str, &'a str>, alt!(basic_string | ml_basic_string | literal_string | ml_literal_string));

  // Basic String
  method!(raw_basic_string<Parser<'a>, &'a str, &'a str>, self,
    re_find!("^\"( |!|[#-\\[]|[\\]-􏿿]|(\\\\\")|(\\\\)|(\\\\/)|(\\b)|(\\f)|(\\n)|(\\r)|(\\t)|(\\\\u[0-9A-Z]{4})|(\\\\U[0-9A-Z]{8}))*?\""));
  // Multiline Basic String
  // TODO: Convert this to take_while_s using a function that increments self.linecount
  method!(raw_ml_basic_string<Parser<'a>, &'a str, &'a str>, self,
    chain!(
   string: re_find!("^\"\"\"([ -\\[]|[\\]-􏿿]|(\\\\\")|(\\\\)|(\\\\/)|(\\b)|(\\f)|(\\n)|(\\r)|(\t)|(\\\\u[0-9A-Z]{4})|(\\\\U[0-9A-Z]{8})|\n|(\r\n)|(\\\\(\n|(\r\n))))*?\"\"\""),
      ||{self.line_count.set(self.line_count.get() + count_lines(string)); string}
    )
  );
  // Literal String
  method!(raw_literal_string<Parser<'a>, &'a str, &'a str>, self, re_find!("^'(	|[ -&]|[\\(-􏿿])*?'"));
  // Multiline Literal String
  // TODO: Convert to take_while_s using a function that increments self.linecount
  method!(raw_ml_literal_string<Parser<'a>, &'a str, &'a str>, self,
    chain!(
   string: re_find!("^'''(	|[ -􏿿]|\n|(\r\n))*?'''"),
      ||{self.line_count.set(self.line_count.get() + count_lines(string)); string}
    )
  );


  fn ml_basic_string(mut self: Parser<'a>, input: &'a str) -> (Parser<'a>, nomplusplus::IResult<&'a str, &'a str>) {
    let (tmp, raw) = self.raw_ml_basic_string(input);
    self = tmp;
    let r = match raw {
      IResult::Done(i, o) => IResult::Done(i, &o["\"\"\"".input_len()..o.input_len()-"\"\"\"".input_len()]),
      IResult::Error(_) => IResult::Error(nomplusplus::Err::Code(nomplusplus::ErrorKind::Custom(ErrorCode::MLLiteralString as u32))),
      IResult::Incomplete(i) => IResult::Incomplete(i),
    };
    (self, r)
  }

  fn basic_string(mut self: Parser<'a>, input: &'a str) -> (Parser<'a>, nomplusplus::IResult<&'a str, &'a str>) {
    let (tmp, raw) = self.raw_basic_string(input);
    self = tmp;
    let r = match raw {
      IResult::Done(i, o) => IResult::Done(i, &o["\"".input_len()..o.input_len()-"\"".input_len()]),
      IResult::Error(_) => IResult::Error(nomplusplus::Err::Code(nomplusplus::ErrorKind::Custom(ErrorCode::MLLiteralString as u32))),
      IResult::Incomplete(i) => IResult::Incomplete(i),
    };
    (self, r)
  }

  fn ml_literal_string(mut self: Parser<'a>, input: &'a str) -> (Parser<'a>, nomplusplus::IResult<&'a str, &'a str>) {
    let (tmp, raw) = self.raw_ml_literal_string(input);
    self = tmp;
    let r = match raw {
      IResult::Done(i, o) => IResult::Done(i, &o["'''".input_len()..o.input_len()-"'''".input_len()]),
      IResult::Error(_) => IResult::Error(nomplusplus::Err::Code(nomplusplus::ErrorKind::Custom(ErrorCode::MLLiteralString as u32))),
      IResult::Incomplete(i) => IResult::Incomplete(i),
    };
    (self, r)
  }

  fn literal_string(mut self: Parser<'a>, input: &'a str) -> (Parser<'a>, nomplusplus::IResult<&'a str, &'a str>) {
    let (tmp, raw) = self.raw_literal_string(input);
    self = tmp;
    let r = match raw {
      IResult::Done(i, o) => IResult::Done(i, &o["'".input_len()..o.input_len()-"'".input_len()]),
      IResult::Error(_) => IResult::Error(nomplusplus::Err::Code(nomplusplus::ErrorKind::Custom(ErrorCode::MLLiteralString as u32))),
      IResult::Incomplete(i) => IResult::Incomplete(i),
    };
    (self, r)
  }

  method!(string<Parser<'a>, &'a str, Value>, mut self,
    alt!(
      complete!(call_m!(self.ml_literal_string))  => {|ml| Value::String(ml, StrType::MLLiteral)}  |
      complete!(call_m!(self.ml_basic_string))    => {|mb| Value::String(mb, StrType::MLBasic)}  |
      complete!(call_m!(self.basic_string))       => {|b| Value::String(b, StrType::Basic)}    |
      complete!(call_m!(self.literal_string))     => {|l| Value::String(l, StrType::Literal)}
    )
  );


  // Boolean
  method!(boolean<Parser<'a>, &'a str, &'a str>, self, alt!(complete!(tag_s!("false")) | complete!(tag_s!("true"))));


  // Datetime
  // I use re_capture here because I only want the number without the dot. It captures the entire match
  // in the 0th position and the first capture group in the 1st position
  method!(fractional<Parser<'a>, &'a str, Vec<&'a str> >, self, re_capture!("^\\.([0-9]+)"));

  method!(time<Parser<'a>, &'a str, Time>, mut self,
    chain!(
      hour: re_find!("^[0-9]{2}")   ~
            tag_s!(":")             ~
    minute: re_find!("^[0-9]{2}")   ~
            tag_s!(":")             ~
    second: re_find!("^[0-9]{2}")   ~
   fraction: complete!(call_m!(self.fractional)) ? ,
      ||{
        Time{
          hour: hour, minute: minute, second: second, fraction: match fraction {
            Some(ref x) => x[1],
            None        => "",
          }
        }
      }
    )
  );

  method!(time_offset_amount<Parser<'a>, &'a str, TimeOffsetAmount>, self,
    chain!(
  pos_neg: alt!(complete!(tag_s!("+")) | complete!(tag_s!("-")))  ~
     hour: re_find!("^[0-9]{2}")                                                                      ~
           tag_s!(":")                                                                                      ~
  minute: re_find!("^[0-9]{2}")                                                                       ,
      ||{
        TimeOffsetAmount{
          pos_neg: pos_neg, hour: hour, minute: minute
        }
      }
    )
  );

  method!(time_offset<Parser<'a>, &'a str, TimeOffset>, mut self,
    alt!(
      complete!(tag_s!("Z"))                       => {|_|       TimeOffset::Z} |
      complete!(call_m!(self.time_offset_amount))  => {|offset|  TimeOffset::Time(offset)}
    )
  );

  method!(full_date<Parser<'a>, &'a str, FullDate>, self,
    chain!(
     year: re_find!("^([0-9]{4})") ~
           tag_s!("-") ~
    month: re_find!("^([0-9]{2})") ~
           tag_s!("-") ~
      day: re_find!("^([0-9]{2})"),
      ||{
        FullDate{
          year: year, month: month, day: day
        }
      }
    )
  );

  method!(date_time<Parser<'a>, &'a str, DateTime>, mut self,
    chain!(
     date: call_m!(self.full_date)  ~
           tag_s!("T")~
     time: call_m!(self.time)       ~
   offset: call_m!(self.time_offset),
        ||{
        DateTime{
          year: date.year, month: date.month, day: date.day,
          hour: time.hour, minute: time.minute, second: time.second,
          fraction: time.fraction, offset: offset
        }
      }
    )
  );

  // Key-Value pairs
  method!(unquoted_key<Parser<'a>, &'a str, &'a str>, self, take_while1_s!(is_keychar));
  method!(quoted_key<Parser<'a>, &'a str, &'a str>, self, re_find!("^\"( |!|[#-\\[]|[\\]-􏿿]|(\\\\\")|(\\\\\\\\)|(\\\\/)|(\\\\b)|(\\\\f)|(\\\\n)|(\\\\r)|(\\\\t)|(\\\\u[0-9A-Z]{4})|(\\\\U[0-9A-Z]{8}))+\""));

  method!(pub key<Parser<'a>, &'a str, &'a str>, mut self, alt!(complete!(call_m!(self.quoted_key)) | complete!(call_m!(self.unquoted_key))));

  method!(keyval_sep<Parser<'a>, &'a str, WSSep>, mut self,
    chain!(
      ws1: call_m!(self.ws) ~
           tag_s!("=")      ~
      ws2: call_m!(self.ws) ,
      ||{
        WSSep{
          ws1: ws1, ws2: ws2
        }
      }     
    )
  );

  method!(pub val<Parser<'a>, &'a str, Value>, mut self,
    alt!(
      complete!(call_m!(self.array))        => {|arr|   Value::Array(Box::new(arr))}      |
      complete!(call_m!(self.inline_table)) => {|it|    Value::InlineTable(Box::new(it))} |
      complete!(call_m!(self.date_time))    => {|dt|    Value::DateTime(dt)}              |
      complete!(call_m!(self.float))        => {|flt|   Value::Float(flt)}                |
      complete!(call_m!(self.integer))      => {|int|   Value::Integer(int)}              |
      complete!(call_m!(self.boolean))      => {|b|     Value::Boolean(b)}               |
      complete!(call_m!(self.string))       => {|s|     s}
    )
  );

  method!(pub keyval<Parser<'a>, &'a str, KeyVal>, mut self,
    chain!(
      key: call_m!(self.key)        ~
       ws: call_m!(self.keyval_sep) ~
      val: call_m!(self.val)        ,
      || {
        KeyVal{
          key: key, keyval_sep: ws, val: val
        }
      }
    )
  );
}

#[cfg(test)]
mod test {
  use nomplusplus::IResult::Done;
  use ast::structs::{Time, FullDate, WSSep, Array, ArrayValue, KeyVal,
                     InlineTable, TableKeyVal, Value, StrType};
  use ::types::{DateTime, TimeOffsetAmount, TimeOffset};
  use parser::Parser;

  #[test]
  fn test_integer() {
    let p = Parser::new();
    assert_eq!(p.integer("345_12_678").1, Done("", "345_12_678"));
  }

  #[test]
  fn test_float() {
    let p = Parser::new();
    assert_eq!(p.float("98_7.2_34e-8_8").1, Done("", "98_7.2_34e-8_8"));
  }

  #[test]
  fn test_basic_string() {
    let p = Parser::new();
    assert_eq!(p.basic_string("\"Tλïƨ ïƨ á βáƨïç ƨƭřïñϱ.\"").1, Done("", "Tλïƨ ïƨ á βáƨïç ƨƭřïñϱ."));
  }

  #[test]
  fn test_ml_basic_string() {
    let p = Parser::new();
    assert_eq!(p.ml_basic_string("\"\"\"£ïñè Óñè
£ïñè Tωô
£ïñè Tλřèè\"\"\"").1, Done("", r#"£ïñè Óñè
£ïñè Tωô
£ïñè Tλřèè"# ));
  }

  #[test]
  fn test_literal_string() {
    let p = Parser::new();
    assert_eq!(p.literal_string("'Abc џ'").1, Done("", "Abc џ")); 
  }

  #[test]
  fn test_ml_literal_string() {
    let p = Parser::new();
    assert_eq!(p.ml_literal_string(r#"'''
                                    Abc џ
                                    '''"#).1,
      Done("", r#"
                                    Abc џ
                                    "#));
  }

  #[test]
  fn test_string() {
    let mut p = Parser::new();
    assert_eq!(p.string("\"βáƨïç_ƨƭřïñϱ\"").1, Done("", Value::String("βáƨïç_ƨƭřïñϱ", StrType::Basic)));
    p = Parser::new();
    assert_eq!(p.string(r#""""₥ℓ_βáƨïç_ƨƭřïñϱ
ñú₥βèř_ƭωô
NÛMßÉR-THRÉÉ
""""#).1, Done("", Value::String(r#"₥ℓ_βáƨïç_ƨƭřïñϱ
ñú₥βèř_ƭωô
NÛMßÉR-THRÉÉ
"#, StrType::MLBasic)));
    p = Parser::new();
    assert_eq!(p.string("'£ÌTÉRÂ£§TRïNG'").1, Done("", Value::String("£ÌTÉRÂ£§TRïNG", StrType::Literal)));
    p = Parser::new();
    assert_eq!(p.string(r#"'''§ƥřïƭè
Çôƙè
Þèƥƨï
'''"#).1,
      Done("", Value::String(r#"§ƥřïƭè
Çôƙè
Þèƥƨï
"#, StrType::MLLiteral)));

  }

  #[test]
  fn test_boolean() {
    let mut p = Parser::new();
    assert_eq!(p.boolean("true").1, Done("", "true"));
    p = Parser::new();
    assert_eq!(p.boolean("false").1, Done("", "false"));
  }

  #[test]
  fn test_fractional() {
    let p = Parser::new();
    assert_eq!(p.fractional(".03856").1, Done("", vec![".03856", "03856"]));
  }

  #[test]
  fn test_time() {
    let mut p = Parser::new();
    assert_eq!(p.time("11:22:33.456").1,
      Done("", Time{
        hour: "11",
        minute: "22",
        second: "33",
        fraction: "456"
      })
    );
    p = Parser::new();
    assert_eq!(p.time("04:05:06").1,
      Done("", Time{
        hour: "04",
        minute: "05",
        second: "06",
        fraction: ""
      })
    );
  }

  #[test]
  fn test_time_offset_amount() {
    let p = Parser::new();
    assert_eq!(p.time_offset_amount("+12:34").1,
      Done("", TimeOffsetAmount{
        pos_neg: "+",
        hour: "12",
        minute: "34"
      })
    );
  }

  #[test]
  fn test_time_offset() {
    let mut p = Parser::new();
    assert_eq!(p.time_offset("+12:34").1,
      Done("", TimeOffset::Time(TimeOffsetAmount{
        pos_neg: "+",
        hour: "12",
        minute: "34"
      }))
    );
    p = Parser::new();
    assert_eq!(p.time_offset("Z").1, Done("", TimeOffset::Z));
  }

  #[test]
  fn test_full_date() {
    let p = Parser::new();
    assert_eq!(p.full_date("1942-12-07").1,
      Done("", FullDate{
        year: "1942", month: "12", day: "07"
      })
    );
  }

  #[test]
  fn test_date_time() {
    let mut p = Parser::new();
    assert_eq!(p.date_time("1999-03-21T20:15:44.5-07:00").1,
      Done("", DateTime{
        year: "1999", month: "03", day: "21",
        hour: "20", minute: "15", second: "44", fraction: "5",
        offset: TimeOffset::Time(TimeOffsetAmount{
          pos_neg: "-",
          hour: "07",
          minute: "00"
        })
      })
    );
  }

  #[test]
  fn test_unquoted_key() {
    let p = Parser::new();
    assert_eq!(p.unquoted_key("Un-Quoted_Key").1, Done("", "Un-Quoted_Key"));
  }

  #[test]
  fn test_quoted_key() {
    let p = Parser::new();
    assert_eq!(p.quoted_key("\"QúôƭèδKè¥\"").1, Done("", "\"QúôƭèδKè¥\""));
  }

  #[test]
  fn test_key() {
    let mut p = Parser::new();
    assert_eq!(p.key("\"Gřáƥèƒřúïƭ\"").1, Done("", "\"Gřáƥèƒřúïƭ\""));
    p = Parser::new();
    assert_eq!(p.key("_is-key").1, Done("", "_is-key"));
  }

  #[test]
  fn test_keyval_sep() {
    let p = Parser::new();
    assert_eq!(p.keyval_sep("\t \t= \t").1, Done("", WSSep{ws1: "\t \t", ws2: " \t"}));
  }

  #[test]
  fn test_val() {
    let mut p = Parser::new();
    assert_eq!(p.val("[4,9]").1, Done("",
      Value::Array(Box::new(Array{
        values: vec![
          ArrayValue{
            val: Value::Integer("4"), array_sep: Some(WSSep{
              ws1: "", ws2: ""
            }),
            comment_nl: None
          },
          ArrayValue{
            val: Value::Integer("9"), array_sep: None,
            comment_nl: None
          },
        ],
        ws: WSSep{ws1: "", ws2: ""}
      }
    ))));
    p = Parser::new();
    assert_eq!(p.val("{\"§ô₥è Þïϱ\"='Táƨƭ¥ Þôřƙ'}").1, Done("",
      Value::InlineTable(Box::new(InlineTable{
        keyvals: Some(vec![
          TableKeyVal{
            keyval: KeyVal{
              key: "\"§ô₥è Þïϱ\"", keyval_sep: WSSep{
                ws1: "", ws2: ""
              },
              val: Value::String("Táƨƭ¥ Þôřƙ", StrType::Literal)
            },
            kv_sep: WSSep{ws1: "", ws2: ""}
          }
        ]),
        ws: WSSep{
          ws1: "", ws2: ""
        }
    }))));
    p = Parser::new();
    assert_eq!(p.val("2112-09-30T12:33:01.345-11:30").1, Done("", Value::DateTime(DateTime{
                              year: "2112", month: "09", day: "30",
                              hour: "12", minute: "33", second: "01", fraction: "345",
                              offset: TimeOffset::Time(TimeOffsetAmount{
                                pos_neg: "-", hour: "11", minute: "30"
                              })
                            })));
    p = Parser::new();
    assert_eq!(p.val("3487.3289E+22").1, Done("", Value::Float("3487.3289E+22")));
    p = Parser::new();
    assert_eq!(p.val("8932838").1, Done("", Value::Integer("8932838")));
    p = Parser::new();
    assert_eq!(p.val("false").1, Done("", Value::Boolean("false")));
    p = Parser::new();
    assert_eq!(p.val("true").1, Done("", Value::Boolean("true")));
    p = Parser::new();
    assert_eq!(p.val("'§ô₥è §ƭřïñϱ'").1, Done("", Value::String("§ô₥è §ƭřïñϱ", StrType::Literal)));
  }

  #[test]
  fn test_keyval() {
    let p = Parser::new();
    assert_eq!(p.keyval("Boolean = 84.67").1, Done("", KeyVal{
      key: "Boolean", keyval_sep: WSSep{
        ws1: " ", ws2: " "
      },
      val: Value::Float("84.67")
    }));
  }
}