extern crate tomllib;
//use std::rc::Rc;
use tomllib::parser::Parser;
//use tomllib::types::{StrType, TOMLValue, TimeOffsetAmount, TimeOffset,
//                     DateTime, Date, Time, Str};

fn main() {
	let parser = Parser::new();
// 	let mut parser = parser.parse(r#"# This is a TOML document.

// title = "TOML Example"

// [owner]
// name = "Tom Preston-Werner"
// dob = 1979-05-27T07:32:00-08:00 # First class dates

// [database]
// server = "192.168.1.1"
// ports = [ 8001, 8003, [5, 6] ]
// connection_max = 5000
// enabled = true

// [servers]

//   # Indentation (tabs and/or spaces) is allowed but not required
//   [servers.alpha]
//   ip = "10.0.0.1"
//   dc = "eqdc10"

//   [servers.beta]
//   ip = "10.0.0.2"
//   dc = "eqdc10"
// [[flower]]
// color = "blue"
// type = "rose"
// [[flower]]
// color = "red"
// type = "carnation"
//   [flower.props]
//     stuff = true
//     junk = 4.35
//   [[flower.power]]
//   foo = "bar"
  
// [clients]
// data = [ ["gamma", "delta"], [1, 2] ]

// # Line breaks are OK when inside arrays
// hosts = [
//   "alpha",
//   "omega"
// ]"#);
let parser = parser.parse(
r#"[[products]]
name = "Hammer"
sku = 738594937

[[products]]

[[products]]
name = "Nail"
sku = 284758393
color = "gray"

[[fruit]]
  name = "apple"

  [fruit.physical.fizzy.phys]]
    color = "red"
    shape = "round"

  [[fruit.variety]]
    name = "red delicious"

  [[fruit.variety]]
    name = "granny smith"

[[fruit]]
  name = "banana"

  [[fruit.variety]]
    name = "plantain"
"#);
  // let mut new_owner = String::new();
  // new_owner.push_str("Joel Self");
	// println!("{}", parser.get_value("database.server".to_string()).unwrap());
 //  parser.set_value("database.server".to_string(),
 //    TOMLValue::String(Str::String("localhost".to_string()), StrType::Basic));
 //  println!("{}", parser.get_value("database.server".to_string()).unwrap());
 //  parser.set_value("database.connection_max".to_string(),
 //    TOMLValue::String(Str::String("100".to_string()), StrType::Literal));
 //  parser.set_value("title".to_string(), TOMLValue::Float(Str::String("4.567".to_string())));
 //  parser.set_value("owner.dob".to_string(), TOMLValue::DateTime(DateTime::new(
 //    Date::new_string("1981".to_string(), "04".to_string(), "15".to_string()),
 //    Some(Time::new_string("08".to_string(), "08".to_string(), "00".to_string(), Some("005".to_string()),
 //      Some(TimeOffset::Time(TimeOffsetAmount::new_string(
 //        "+".to_string(), "05".to_string(), "30".to_string()
 //      )))
 //    )
 //  ))));
 //  parser.set_value("database.ports".to_string(), TOMLValue::Array(
 //    Rc::new(vec![
 //      TOMLValue::Float(Str::String("1.23".to_string())),
 //      TOMLValue::Float(Str::String("4.56".to_string())),
 //      TOMLValue::Array(
 //        Rc::new(vec![
 //          TOMLValue::String(Str::String("foobar".to_string()), StrType::MLBasic),
 //          TOMLValue::String(Str::String("barfoo".to_string()), StrType::MLLiteral)
 //        ])
 //      ),
 //    ])
 //  ));
  parser.print_keys_and_values();
  println!("{}", parser);
  // let (tmp, new_str) = parser.add_string(new_owner);
  // let parser = tmp;
  //parser.set_value("owner.name".to_string(),
    //TOMLValue::String(new_str, StrType::Basic));
 }