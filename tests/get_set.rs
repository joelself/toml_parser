extern crate tomllib;
extern crate env_logger;
use tomllib::parser::TOMLParser;
use tomllib::types::{ParseResult, Value, ParseError};
use std::rc::Rc;



  #[test]
  fn test_output_after_set() {
    let _ = env_logger::init();
    let p = TOMLParser::new();
    let (mut p, _) = p.parse(r#"animal = "bear"

[[car.owners]]
Name = """Bob Jones"""
Age = 25
[[car.owners]]
Name = 'Jane Doe'
Age = 44

[car.interior.seats]
type = '''fabric'''
count = 5

[car]
model = "Civic"
"ωλèèℓƨ" = 4
"ƭôƥ ƨƥèèδ" = 124.56
"Date of Manufacture" = 2007-05-16T10:12:13.2324+04:00
drivers = ["Bob", "Jane", "John", "Michael", { disallowed = "Chris", banned="Sally"}]
properties = { color = "red", "plate number" = "ABC 345",
               accident_dates = [2008-09-29, 2011-01-16, 2014-11-30T03:13:54]}
"#);
    p.set_value("car.interior.seats.type", Value::basic_string("leather").unwrap());
    p.set_value("car.interior.seats.type", Value::basic_string("vinyl").unwrap());
    p.set_value("car.owners[0].Age", Value::float_from_str("19.5").unwrap());
    p.set_value("car.owners[1].Name", Value::ml_basic_string("Steve Parker").unwrap());
    p.set_value("car.drivers[4].banned", Value::datetime_from_int(2013, 9, 23, 17, 34, 2).unwrap());
    p.set_value("car.properties.color", Value::int(19));
    p.set_value("car.properties.accident_dates[2]", Value::float(3443.34));
    p.set_value("car.drivers[1]", Value::ml_literal_string("Mark").unwrap());
    p.set_value("car.properties", Value::InlineTable(Rc::new(
      vec![("make".into(), Value::literal_string("Honda").unwrap()),
           ("transmission".into(), Value::bool(true))]
    )));
    p.set_value("car.drivers", Value::Array(Rc::new(
      vec![Value::basic_string("Phil").unwrap(), Value::basic_string("Mary").unwrap()]
    )));
    p.set_value("car.properties", Value::InlineTable(Rc::new(
      vec![("prop1".into(), Value::bool_from_str("TrUe").unwrap()),
           ("prop2".into(), Value::bool_from_str("FALSE").unwrap()),
           ("prop3".into(), Value::bool_from_str("truE").unwrap()),
           ("prop4".into(), Value::bool_from_str("false").unwrap())]
    )));
    p.set_value("car.drivers", Value::Array(Rc::new(
      vec![Value::int(1), Value::int(2), Value::int(3), Value::int(4),
      Value::int(5), Value::int(6), Value::int(7), Value::int(8)]
    )));
    p.set_value("car.model", Value::literal_string("Accord").unwrap());
    p.set_value("animal", Value::ml_basic_string("shark").unwrap());
    assert_eq!(r#"animal = """shark"""

[[car.owners]]
Name = """Bob Jones"""
Age = 19.5
[[car.owners]]
Name = """Steve Parker"""
Age = 44

[car.interior.seats]
type = "vinyl"
count = 5

[car]
model = 'Accord'
"ωλèèℓƨ" = 4
"ƭôƥ ƨƥèèδ" = 124.56
"Date of Manufacture" = 2007-05-16T10:12:13.2324+04:00
drivers = [1, 2, 3, 4, 5, 6, 7, 8]
properties = { prop1 = true, prop2 = false, prop3 = true, prop4 = false }
"#, format!("{}", p));
  }

struct TT;
impl TT {
  fn get<'a>() -> &'a str {
    return r#"fish = "halibut"
[[foo."bar"]]
baz = 12345
qux = 1981-04-15
[[foo."bar"]]
baz = 'something'
qux = '''other'''
[[foo.quality]]
furniture = "chair"
[foo.quality.machine.parts.service]
"ƥèřïôδ" = 24.7
"inline table" = { drink = 5.5, meal = [ 6, { start = 1980-05-14, end = 2002-10-19 } ], dessert = '''cake''' }
[[foo.quality.labor]]
Name = 'Rïçλářδ'
[[foo.quality.labor]]
Name = '§ƭèřℓïñϱ Âřçλèř'
[[foo.quality]]
money = 789.0123
[[foo."bar"]]
baz = 2016-03-10T12:31:02+07:30
qux = """ƒáβúℓôúƨ δïñôƨáúř"""
array = [1, 2, {you = ["good", """bye"""], fire = "truck"}, 3]
[foo]
"δïáϱñôƨïƨ" = true
"ƥřôϱñôƨïƨ" = "not good"
hypnosis = 987654321
"#;
  }
}

#[test]
fn test_mixed_tables() {
  let _ = env_logger::init();

  let parser = TOMLParser::new();
  let (parser, result) = parser.parse(TT::get());
  let mut get_errors = false;
  let mut leftover = "".to_string();
  match result {
    ParseResult::FullError => {get_errors = true;},
    ParseResult::Partial(l,_,_) => {leftover = l.into_owned();},
    ParseResult::PartialError(l,_,_) => {get_errors = true; leftover = l.into_owned();},
    ParseResult::Failure(line,_) => {panic!("failed at line number: {}", line);},
    _ => (),
  }
  let mut full_panic = "".to_string();
  if get_errors {
    let errors = parser.get_errors();
    for error in errors.borrow().iter() {
      match error {
        &ParseError::MixedArray(ref key, _) => full_panic.push_str(&format!("MixedArray error: {}\n", key)),
        &ParseError::DuplicateKey(ref key, _, _) => full_panic.push_str(&format!("Duplicate key error: {}\n", key)),
        &ParseError::InvalidTable(ref key, _, _) => full_panic.push_str(&format!("Invalid table error: {}\n", key)),
        &ParseError::InvalidDateTime(ref key, _) => full_panic.push_str(&format!("Invalid datetime error: {}\n", key)),
      }
    }
  }
  if leftover != "" {
    full_panic.push_str(&format!("$$ Leftover =\n\"{}\"", leftover));
  }
  parser.print_keys_and_values_debug();
  panic!(full_panic);
}

#[test]
fn test_basic_get_on_mixed_tables() {
  let _ = env_logger::init();

  let parser = TOMLParser::new();
  let (parser, _) = parser.parse(TT::get());
  
  // test getting bare key
  assert_eq!(Value::basic_string("halibut").unwrap(), parser.get_value("fish").unwrap());
}