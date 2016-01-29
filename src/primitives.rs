use ast::structs::{Time, FullDate, KeyVal, WSSep, Value, StrType, ErrorCode};
use ::types::{DateTime, TimeOffset, TimeOffsetAmount};
use parser::{Parser, ParseData, count_lines};
use nom;
use nom::IResult;

fn is_keychar(chr: char) -> bool {
  let uchr = chr as u32;
  uchr >= 0x41 && uchr <= 0x5A || // A-Z
  uchr >= 0x61 && uchr <= 0x7A || // a-z
  uchr >= 0x30 && uchr <= 0x39 || // 0-9
  uchr == 0x2D || uchr == 0x5f    // "-", "_"
}

impl<'a> Parser {
  // Integer
  named!(integer<&'a str, &'a str, &mut ParseData<'a> >, data, re_find!("^((\\+|-)?(([1-9](\\d|(_\\d))+)|\\d))")) ;

  // Float
  named!(float<&'a str, &'a str, &mut ParseData<'a> >, data,
         re_find!("^(\\+|-)?([1-9](\\d|(_\\d))+|\\d)((\\.\\d(\\d|(_\\d))*)((e|E)(\\+|-)?([1-9](\\d|(_\\d))+|\\d))|(\\.\\d(\\d|(_\\d))*)|((e|E)(\\+|-)?([1-9](\\d|(_\\d))+|\\d)))"));

  // String
  // TODO: named!(string<&'a str, &'a str>, alt!(basic_string | ml_basic_string | literal_string | ml_literal_string));

  // Basic String
  named!(raw_basic_string<&'a str, &'a str, &mut ParseData<'a> >, data,
    re_find!("^\"( |!|[#-\\[]|[\\]-􏿿]|(\\\\\")|(\\\\)|(\\\\/)|(\\b)|(\\f)|(\\n)|(\\r)|(\\t)|(\\\\u[0-9A-Z]{4})|(\\\\U[0-9A-Z]{8}))*?\""));
  // Multiline Basic String
  named!(raw_ml_basic_string<&'a str, &'a str, &mut ParseData<'a> >, data,
    chain!(
   string: re_find!("^\"\"\"([ -\\[]|[\\]-􏿿]|(\\\\\")|(\\\\)|(\\\\/)|(\\b)|(\\f)|(\\n)|(\\r)|(\t)|(\\\\u[0-9A-Z]{4})|(\\\\U[0-9A-Z]{8})|\n|(\r\n)|(\\\\(\n|(\r\n))))*?\"\"\""),
      ||{data.line_count.set(data.line_count.get() + count_lines(string)); string}
    )
  );
  // Literal String
  named!(raw_literal_string<&'a str, &'a str, &mut ParseData<'a> >, data, re_find!("^'(	|[ -&]|[\\(-􏿿])*?'"));
  // Multiline Literal String
  named!(raw_ml_literal_string<&'a str, &'a str, &mut ParseData<'a> >, data,
    chain!(
   string: re_find!("^'''(	|[ -􏿿]|\n|(\r\n))*?'''"),
      ||{data.line_count.set(data.line_count.get() + count_lines(string)); string}
    )
  );


  fn ml_basic_string(input: &'a str, data: &mut ParseData<'a> ) -> nom::IResult<&'a str, &'a str> {
    let raw = Parser::raw_ml_basic_string(input, data);
    match &raw {
      &IResult::Done(i, o) => IResult::Done(i, &o["\"\"\"".len()..o.len()-"\"\"\"".len()]),
      &IResult::Error(_) => IResult::Error(nom::Err::Code(nom::ErrorKind::Custom(ErrorCode::MLLiteralString as u32))),
      &IResult::Incomplete(i) => IResult::Incomplete(i),
    }
  }

  fn basic_string(input: &'a str, data: &mut ParseData<'a> ) -> nom::IResult<&'a str, &'a str> {
    let raw = Parser::raw_basic_string(input, data);
    match &raw {
      &IResult::Done(i, o) => IResult::Done(i, &o["\"".len()..o.len()-"\"".len()]),
      &IResult::Error(_) => IResult::Error(nom::Err::Code(nom::ErrorKind::Custom(ErrorCode::MLLiteralString as u32))),
      &IResult::Incomplete(i) => IResult::Incomplete(i),
    }
  }

  fn ml_literal_string(input: &'a str, data: &mut ParseData<'a> ) -> nom::IResult<&'a str, &'a str> {
    let raw = Parser::raw_ml_literal_string(input, data);
    match &raw {
      &IResult::Done(i, o) => IResult::Done(i, &o["'''".len()..o.len()-"'''".len()]),
      &IResult::Error(_) => IResult::Error(nom::Err::Code(nom::ErrorKind::Custom(ErrorCode::MLLiteralString as u32))),
      &IResult::Incomplete(i) => IResult::Incomplete(i),
    }
  }

  fn literal_string(input: &'a str, data: &mut ParseData<'a> ) -> nom::IResult<&'a str, &'a str> {
    let raw = Parser::raw_literal_string(input, data);
    match &raw {
      &IResult::Done(i, o) => IResult::Done(i, &o["'".len()..o.len()-"'".len()]),
      &IResult::Error(_) => IResult::Error(nom::Err::Code(nom::ErrorKind::Custom(ErrorCode::MLLiteralString as u32))),
      &IResult::Incomplete(i) => IResult::Incomplete(i),
    }
  }

  named!(string<&'a str, Value<'a>, &mut ParseData<'a> >, data,
    alt!(
      complete!(call_d!(Parser::ml_literal_string, data))  => {|ml| Value::String(ml, StrType::MLLiteral)}  |
      complete!(call_d!(Parser::ml_basic_string, data))    => {|mb| Value::String(mb, StrType::MLBasic)}  |
      complete!(call_d!(Parser::basic_string, data))       => {|b| Value::String(b, StrType::Basic)}    |
      complete!(call_d!(Parser::literal_string, data))     => {|l| Value::String(l, StrType::Literal)}
    )
  );

  // Boolean
  named!(boolean<&'a str, &'a str, &mut ParseData<'a> >, data, alt!(complete!(tag_s!("false")) | complete!(tag_s!("true"))));


  // Datetime
  // I use re_capture here because I only want the number without the dot. It captures the entire match
  // in the 0th position and the first capture group in the 1st position
  named!(fractional<&'a str, Vec<&'a str>, &mut ParseData<'a> >, data, re_capture!("^\\.([0-9]+)"));

  named!(time<&'a str, Time<'a>, &mut ParseData<'a> >, data,
    chain!(
      hour: re_find!("^[0-9]{2}")   ~
            tag_s!(":")             ~
    minute: re_find!("^[0-9]{2}")   ~
            tag_s!(":")             ~
    second: re_find!("^[0-9]{2}")   ~
   fraction: complete!(call_d!(Parser::fractional, data)) ? ,
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

  named!(time_offset_amount<&'a str, TimeOffsetAmount<'a>, &mut ParseData<'a> >, data,
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

  named!(time_offset<&'a str, TimeOffset<'a>, &mut ParseData<'a> >, data,
    alt!(
      complete!(tag_s!("Z"))                       => {|_|       TimeOffset::Z} |
      complete!(call_d!(Parser::time_offset_amount, data))  => {|offset|  TimeOffset::Time(offset)}
    )
  );

  named!(full_date<&'a str, FullDate<'a>, &mut ParseData<'a> >, data,
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

  named!(date_time<&'a str, DateTime<'a>, &mut ParseData<'a> >, data,
    chain!(
     date: call_d!(Parser::full_date, data)  ~
           tag_s!("T")~
     time: call_d!(Parser::time, data)       ~
   offset: call_d!(Parser::time_offset, data),
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
  named!(unquoted_key<&'a str, &'a str, &mut ParseData<'a> >, data, take_while1_s!(is_keychar));
  named!(quoted_key<&'a str, &'a str, &mut ParseData<'a> >, data, re_find!("^\"( |!|[#-\\[]|[\\]-􏿿]|(\\\\\")|(\\\\\\\\)|(\\\\/)|(\\\\b)|(\\\\f)|(\\\\n)|(\\\\r)|(\\\\t)|(\\\\u[0-9A-Z]{4})|(\\\\U[0-9A-Z]{8}))+\""));

  named!(pub key<&'a str, &'a str, &mut ParseData<'a> >, data, alt!(complete!(call_d!(Parser::quoted_key, data)) | complete!(call_d!(Parser::unquoted_key, data))));

  named!(keyval_sep<&'a str, WSSep<'a>, &mut ParseData<'a> >, data,
    chain!(
      ws1: call_d!(Parser::ws, data) ~
           tag_s!("=")      ~
      ws2: call_d!(Parser::ws, data) ,
      ||{
        WSSep{
          ws1: ws1, ws2: ws2
        }
      }     
    )
  );

  named!(pub val< &'a str, Value<'a>, &mut ParseData<'a> >, data,
    alt!(
      complete!(call_d!(Parser::array, data))        => {|arr|   Value::Array(Box::new(arr))}      |
      complete!(call_d!(Parser::inline_table, data)) => {|it|    Value::InlineTable(Box::new(it))} |
      complete!(call_d!(Parser::date_time, data))    => {|dt|    Value::DateTime(dt)}              |
      complete!(call_d!(Parser::float, data))        => {|flt|   Value::Float(flt)}                |
      complete!(call_d!(Parser::integer, data))      => {|int|   Value::Integer(int)}              |
      complete!(call_d!(Parser::boolean, data))      => {|b|     Value::Boolean(b)}                |
      complete!(call_d!(Parser::string, data))       => {|s|     s}
    )
  );

  named!(pub keyval<&'a str, KeyVal<'a>, &mut ParseData<'a> >, data,
    chain!(
      key: call_d!(Parser::key, data)        ~
       ws: call_d!(Parser::keyval_sep, data) ~
      val: call_d!(Parser::val, data)        ,
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
  use nom::IResult::Done;
  use ast::structs::{Time, FullDate, WSSep, Array, ArrayValue, KeyVal,
                     InlineTable, TableKeyVal, Value, StrType};
  use ::types::{DateTime, TimeOffsetAmount, TimeOffset};
  use Parser;

  #[test]
  fn test_integer() {
    let p = new();
    assert_eq!(p.integer("345_12_678"), Done("", "345_12_678"));
  }

  #[test]
  fn test_float() {
    let p = new();
    assert_eq!(p.float("98_7.2_34e-8_8"), Done("", "98_7.2_34e-8_8"));
  }

  #[test]
  fn test_basic_string() {
    let p = new();
    assert_eq!(p.basic_string("\"Tλïƨ ïƨ á βáƨïç ƨƭřïñϱ.\""), Done("", "Tλïƨ ïƨ á βáƨïç ƨƭřïñϱ."));
  }

  #[test]
  fn test_ml_basic_string() {
    let p = new();
    assert_eq!(p.ml_basic_string("\"\"\"£ïñè Óñè
£ïñè Tωô
£ïñè Tλřèè\"\"\""), Done("", r#"£ïñè Óñè
£ïñè Tωô
£ïñè Tλřèè"# ));
  }

  #[test]
  fn test_literal_string() {
    let p = new();
    assert_eq!(p.literal_string("'Abc џ'"), Done("", "Abc џ")); 
  }

  #[test]
  fn test_ml_literal_string() {
    let p = new();
    assert_eq!(p.ml_literal_string(r#"'''
                                    Abc џ
                                    '''"#),
      Done("", r#"
                                    Abc џ
                                    "#));
  }

  #[test]
  fn test_string() {
    let p = new();
    assert_eq!(p.string("\"βáƨïç_ƨƭřïñϱ\""), Done("", Value::String("βáƨïç_ƨƭřïñϱ", StrType::Basic)));
assert_eq!(p.string(r#""""₥ℓ_βáƨïç_ƨƭřïñϱ
ñú₥βèř_ƭωô
NÛMßÉR-THRÉÉ
""""#), Done("", Value::String(r#"₥ℓ_βáƨïç_ƨƭřïñϱ
ñú₥βèř_ƭωô
NÛMßÉR-THRÉÉ
"#, StrType::MLBasic)));
    assert_eq!(p.string("'£ÌTÉRÂ£§TRïNG'"), Done("", Value::String("£ÌTÉRÂ£§TRïNG", StrType::Literal)));
    assert_eq!(p.string(r#"'''§ƥřïƭè
Çôƙè
Þèƥƨï
'''"#),
      Done("", Value::String(r#"§ƥřïƭè
Çôƙè
Þèƥƨï
"#, StrType::MLLiteral)));

  }

  #[test]
  fn test_boolean() {
    let p = new();
    assert_eq!(p.boolean("true"), Done("", "true"));
    assert_eq!(p.boolean("false"), Done("", "false"));
  }

  #[test]
  fn test_fractional() {
    let p = new();
    assert_eq!(p.fractional(".03856"), Done("", vec![".03856", "03856"]));
  }

  #[test]
  fn test_time() {
    let p = new();
    assert_eq!(p.time("11:22:33.456"),
      Done("", Time{
        hour: "11",
        minute: "22",
        second: "33",
        fraction: "456"
      })
    );
    assert_eq!(p.time("04:05:06"),
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
    let p = new();
    assert_eq!(p.time_offset_amount("+12:34"),
      Done("", TimeOffsetAmount{
        pos_neg: "+",
        hour: "12",
        minute: "34"
      })
    );
  }

  #[test]
  fn test_time_offset() {
    let p = new();
    assert_eq!(p.time_offset("+12:34"),
      Done("", TimeOffset::Time(TimeOffsetAmount{
        pos_neg: "+",
        hour: "12",
        minute: "34"
      }))
    );
    assert_eq!(p.time_offset("Z"), Done("", TimeOffset::Z));
  }

  #[test]
  fn test_full_date() {
    let p = new();
    assert_eq!(p.full_date("1942-12-07"),
      Done("", FullDate{
        year: "1942", month: "12", day: "07"
      })
    );
  }

  #[test]
  fn test_date_time() {
    let p = new();
    assert_eq!(p.date_time("1999-03-21T20:15:44.5-07:00"),
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
    let p = new();
    assert_eq!(p.unquoted_key("Un-Quoted_Key"), Done("", "Un-Quoted_Key"));
  }

  #[test]
  fn test_quoted_key() {
    let p = new();
    assert_eq!(p.quoted_key("\"QúôƭèδKè¥\""), Done("", "\"QúôƭèδKè¥\""));
  }

  #[test]
  fn test_key() {
    let p = new();
    assert_eq!(p.key("\"Gřáƥèƒřúïƭ\""), Done("", "\"Gřáƥèƒřúïƭ\""));
    assert_eq!(p.key("_is-key"), Done("", "_is-key"));
  }

  #[test]
  fn test_keyval_sep() {
    let p = new();
    assert_eq!(p.keyval_sep("\t \t= \t"), Done("", WSSep{ws1: "\t \t", ws2: " \t"}));
  }

  #[test]
  fn test_val() {
    let p = new();
    assert_eq!(p.val("[4,9]"), Done("",
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

    assert_eq!(p.val("{\"§ô₥è Þïϱ\"='Táƨƭ¥ Þôřƙ'}"), Done("",
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

    assert_eq!(p.val("2112-09-30T12:33:01.345-11:30"), Done("", Value::DateTime(DateTime{
                              year: "2112", month: "09", day: "30",
                              hour: "12", minute: "33", second: "01", fraction: "345",
                              offset: TimeOffset::Time(TimeOffsetAmount{
                                pos_neg: "-", hour: "11", minute: "30"
                              })
                            })));
    assert_eq!(p.val("3487.3289E+22"), Done("", Value::Float("3487.3289E+22")));
    assert_eq!(p.val("8932838"), Done("", Value::Integer("8932838")));
    assert_eq!(p.val("false"), Done("", Value::Boolean("false")));
    assert_eq!(p.val("true"), Done("", Value::Boolean("true")));
    assert_eq!(p.val("'§ô₥è §ƭřïñϱ'"), Done("", Value::String("§ô₥è §ƭřïñϱ", StrType::Literal)));
  }

  #[test]
  fn test_keyval() {
    let p = new();
    assert_eq!(p.keyval("Boolean = 84.67"), Done("", KeyVal{
      key: "Boolean", keyval_sep: WSSep{
        ws1: " ", ws2: " "
      },
      val: Value::Float("84.67")
    }));
  }
}