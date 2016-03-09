extern crate tomllib;
#[macro_use] extern crate log;
extern crate env_logger;
use std::rc::Rc;
use tomllib::parser::Parser;
use tomllib::types::{StrType, TOMLValue, TimeOffsetAmount, TimeOffset,
                     DateTime, Date, Time, Str};

fn main() {
  env_logger::init().unwrap();
	let parser = Parser::new();
	let mut parser = parser.parse(r#"# This is a TOML document.

title = "TOML Example"

[owner]
name = "Tom Preston-Werner"
dob = 1979-05-27T07:32:00-08:00 # First class dates

[database]
server = "192.168.1.1"
ports = [   8001, 8003  ,  [8004  , 8005] ]
servers = {main = "juice", failover1 = "soda", failover2 = { something = 25   , nothing = 2.5 }    }
connection_max = 5000
enabled = true
"#);
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
// let mut parser = parser.parse(
// r#"[[products]]
// name = { first = "Bob", last = "Jones" }
// sku = 738594937

// [[products]]

// [[products]]
// name = "Nail"
// sku = 284758393
// colors = { one = "green", two = { some = "thing", any = [1.1, {deeply = "nested", twoplustwo = 5.5 }]}, three = 8.9 }

// [[fruit]]
//   name = [4, 5]

//   [fruit.physical.fizzy.phys]
//     color = ["tree", {lego = "block", fun = [6.6, [7.7]]}]"#);
//     shape = "round"

//   [[fruit.variety]]
//     name = "red delicious"

//   [[fruit.variety]]
//     name = "granny smith"

// [[fruit]]
//   name = "banana"

//   [[fruit.variety]]
//     name = "plantain"
//"#);
// let parser = parser.parse(
// r#"[computers]
// room_A = [[1], [[2, 5], [3, 4]]]
// "#);
  // let mut new_owner = String::new();
  // new_owner.push_str("Joel Self");
  // println!("{}", parser.get_value("fruit[0].name[1]".to_string()).unwrap());
  // println!("{}", parser.get_value("fruit[0].physical.fizzy.phys.color".to_string()).unwrap());
  // println!("{}", parser.get_value("fruit[0].physical.fizzy.phys.color[1].fun[0]".to_string()).unwrap());
  // println!("{}", parser.get_value("products[2].colors.two.some".to_string()).unwrap());

  // println!("{}", parser.set_value("fruit[0].name[1]".to_string(),
  //   TOMLValue::String(Str::String("brains".to_string()), StrType::MLLiteral)));
  // println!("{}", parser.set_value("fruit[0].physical.fizzy.phys.color".to_string(),
  //   TOMLValue::Integer(Str::String("99".to_string()))));
  // println!("{}", parser.set_value("fruit[0].physical.fizzy.phys.color[1].fun[0]".to_string(),
  //   TOMLValue::DateTime(DateTime::new(
  //   Date::new_string("1981".to_string(), "04".to_string(), "15".to_string()),
  //     Some(Time::new_string("08".to_string(), "08".to_string(), "00".to_string(), Some("005".to_string()),
  //       Some(TimeOffset::Time(TimeOffsetAmount::new_string(
  //         "+".to_string(), "05".to_string(), "30".to_string()
  //       )))
  //     ))
  //   ))
  // ));
  // println!("{}", parser.set_value("products[2].colors.two.some".to_string(),
  //   TOMLValue::Float(Str::String("99.999".to_string()))));


  // println!("{}", parser.get_value("fruit[0].name[1]".to_string()).unwrap());
  // println!("{}", parser.get_value("fruit[0].physical.fizzy.phys.color".to_string()).unwrap());
  // println!("{}", parser.get_value("fruit[0].physical.fizzy.phys.color[1].fun[0]".to_string()).unwrap());
  // println!("{}", parser.get_value("products[2].colors.two.some".to_string()).unwrap());
  // println!("{:?}", parser.get_children("fruit[0]".to_string()).unwrap());
  // println!("{:?}", parser.get_children("fruit[0].physical.fizzy.phys.color".to_string()).unwrap());
  // println!("{:?}", parser.get_children("products[2].colors".to_string()).unwrap());
  // println!("{:?}", parser.get_children("products[2].colors.two.any".to_string()).unwrap());
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
  parser.print_keys_and_values_debug();
  
  
  println!("before: {}", parser.get_value("title".to_string()).unwrap());
  println!("before: {}", parser.get_value("owner.name".to_string()).unwrap());
  println!("before: {}", parser.get_value("owner.dob".to_string()).unwrap());
  println!("before: {}", parser.get_value("database.connection_max".to_string()).unwrap());
  println!("before: {}", parser.get_value("database.ports".to_string()).unwrap());
  println!("before: {}", parser.get_value("database.servers".to_string()).unwrap());
  
  parser.set_value("title".to_string(), TOMLValue::Float(Str::String("7.62".to_string())));
  parser.set_value("owner.name".to_string(), TOMLValue::String(Str::String("Joel Self".to_string()), StrType::MLLiteral));
  parser.set_value("owner.dob".to_string(), TOMLValue::Integer(Str::String("567".to_string())));
  parser.set_value("database.connection_max".to_string(), TOMLValue::DateTime(
    DateTime::new(Date::new_string("03".to_string(), "04".to_string(), "2016".to_string()),
      Some(Time::new_string("09".to_string(), "34".to_string(), "15".to_string(),
        Some("7891".to_string()), Some(TimeOffset::Time(
          TimeOffsetAmount::new_string("-".to_string(), "09".to_string(), "30".to_string())
        ))
      )
    ))
  ));
  
// ports = [   8001, 8003  ,  [8004  , 8005] ]
// servers = {main = "juice", failover1 = "soda", failover2 = { something = 25   , nothing = 2.5 }    }
  parser.set_value("database.ports".to_string(), TOMLValue::Array(Rc::new(vec![
    TOMLValue::Array(Rc::new(vec![
      TOMLValue::String(Str::String("1 thousand".to_string()), StrType::MLLiteral),
      TOMLValue::String(Str::String("3 hundred 4".to_string()), StrType::MLBasic)
    ])),
    TOMLValue::Array(Rc::new(vec![
      TOMLValue::String(Str::String("five 100 7".to_string()), StrType::Basic),
      TOMLValue::Integer(Str::String("100".to_string()))
    ])),
    TOMLValue::String(Str::String("element".to_string()), StrType::Basic),
    TOMLValue::Integer(Str::String("99".to_string())),
    TOMLValue::String(Str::String("index".to_string()), StrType::Basic),
    TOMLValue::Integer(Str::String("98".to_string())),
  ])));
  
  parser.set_value("database.servers".to_string(), TOMLValue::InlineTable(Rc::new(vec![
    (Str::String("woo".to_string()), TOMLValue::Integer(Str::String("1928".to_string()))),
    (Str::String("hoo".to_string()), TOMLValue::Float(Str::String("321.987".to_string()))),
    (Str::String("key1".to_string()), TOMLValue::InlineTable(Rc::new(vec![
      (Str::String("key2".to_string()), TOMLValue::String(Str::String("new value".to_string()), StrType::Literal)),
      (Str::String("key3".to_string()), TOMLValue::String(Str::String("another new value".to_string()), StrType::Basic)),
      (Str::String("key4".to_string()), TOMLValue::String(Str::String("A third new value".to_string()), StrType::Literal)),
      (Str::String("key5".to_string()), TOMLValue::String(Str::String("fourth value".to_string()), StrType::Basic))
    ]))),
    (Str::String("key6".to_string()), TOMLValue::Integer(Str::String("2010".to_string()))),
    (Str::String("key7".to_string()), TOMLValue::Float(Str::String("3.14159".to_string())))
  ])));
  
  println!("after: {}", parser.get_value("title".to_string()).unwrap());
  println!("after: {}", parser.get_value("owner.name".to_string()).unwrap());
  println!("after: {}", parser.get_value("owner.dob".to_string()).unwrap());
  println!("after: {}", parser.get_value("database.connection_max".to_string()).unwrap());
  println!("after: {}", parser.get_value("database.ports".to_string()).unwrap());
  println!("after: {}", parser.get_value("database.servers".to_string()).unwrap());
  
  
  println!("{}", parser);
  // let (tmp, new_str) = parser.add_string(new_owner);
  // let parser = tmp;
  //parser.set_value("owner.name".to_string(),
    //TOMLValue::String(new_str, StrType::Basic));
 }