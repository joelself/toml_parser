extern crate tomllib;
#[macro_use] extern crate log;
extern crate env_logger;
use std::rc::Rc;
use tomllib::parser::Parser;
use tomllib::types::{StrType, TOMLValue, TimeOffsetAmount, TimeOffset,
                     DateTime, Date, Time};

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
  // println!("{}", parser.get_value("fruit[0].name[1]").unwrap());
  // println!("{}", parser.get_value("fruit[0].physical.fizzy.phys.color").unwrap());
  // println!("{}", parser.get_value("fruit[0].physical.fizzy.phys.color[1].fun[0]").unwrap());
  // println!("{}", parser.get_value("products[2].colors.two.some").unwrap());

  // println!("{}", parser.set_value("fruit[0].name[1]",
  //   TOMLValue::String("brains"), StrType::MLLiteral)));
  // println!("{}", parser.set_value("fruit[0].physical.fizzy.phys.color",
  //   TOMLValue::Integer("99"))));
  // println!("{}", parser.set_value("fruit[0].physical.fizzy.phys.color[1].fun[0]",
  //   TOMLValue::DateTime(DateTime::new(
  //   Date::new_string("1981", "04", "15"),
  //     Some(Time::new_string("08", "08", "00", Some("005"),
  //       Some(TimeOffset::Time(TimeOffsetAmount::new_string(
  //         "+", "05", "30"
  //       )))
  //     ))
  //   ))
  // ));
  // println!("{}", parser.set_value("products[2].colors.two.some",
  //   TOMLValue::Float("99.999"))));


  // println!("{}", parser.get_value("fruit[0].name[1]").unwrap());
  // println!("{}", parser.get_value("fruit[0].physical.fizzy.phys.color").unwrap());
  // println!("{}", parser.get_value("fruit[0].physical.fizzy.phys.color[1].fun[0]").unwrap());
  // println!("{}", parser.get_value("products[2].colors.two.some").unwrap());
  // println!("{:?}", parser.get_children("fruit[0]").unwrap());
  // println!("{:?}", parser.get_children("fruit[0].physical.fizzy.phys.color").unwrap());
  // println!("{:?}", parser.get_children("products[2].colors").unwrap());
  // println!("{:?}", parser.get_children("products[2].colors.two.any").unwrap());
 //  parser.set_value("database.server",
 //    TOMLValue::String("localhost"), StrType::Basic));
 //  println!("{}", parser.get_value("database.server").unwrap());
 //  parser.set_value("database.connection_max",
 //    TOMLValue::String("100"), StrType::Literal));
 //  parser.set_value("title", TOMLValue::Float("4.567")));
 //  parser.set_value("owner.dob", TOMLValue::DateTime(DateTime::new(
 //    Date::new_string("1981", "04", "15"),
 //    Some(Time::new_string("08", "08", "00", Some("005"),
 //      Some(TimeOffset::Time(TimeOffsetAmount::new_string(
 //        "+", "05", "30"
 //      )))
 //    )
 //  ))));
 //  parser.set_value("database.ports", TOMLValue::Array(
 //    Rc::new(vec![
 //      TOMLValue::Float("1.23")),
 //      TOMLValue::Float("4.56")),
 //      TOMLValue::Array(
 //        Rc::new(vec![
 //          TOMLValue::String("foobar"), StrType::MLBasic),
 //          TOMLValue::String("barfoo"), StrType::MLLiteral)
 //        ])
 //      ),
 //    ])
 //  ));
  parser.print_keys_and_values_debug();
  
  
  println!("before: {}", parser.get_value("title").unwrap());
  println!("before: {}", parser.get_value("owner.name").unwrap());
  println!("before: {}", parser.get_value("owner.dob").unwrap());
  println!("before: {}", parser.get_value("database.connection_max").unwrap());
  println!("before: {}", parser.get_value("database.ports").unwrap());
  println!("before: {}", parser.get_value("database.servers").unwrap());
  
  parser.set_value("title", TOMLValue::Float("7.62".into()));
  parser.set_value("owner.name", TOMLValue::String("Joel Self".into(), StrType::MLLiteral));
  parser.set_value("owner.dob", TOMLValue::Integer("567".into()));
  parser.set_value("database.connection_max", TOMLValue::DateTime(
    DateTime::new(Date::from_str("03", "04", "2016"),
      Some(Time::from_str("09", "34", "15",
        Some("7891"), Some(TimeOffset::Time(
          TimeOffsetAmount::from_str("-", "09", "30")
        ))
      )
    ))
  ));
  
// ports = [   8001, 8003  ,  [8004  , 8005] ]
// servers = {main = "juice", failover1 = "soda", failover2 = { something = 25   , nothing = 2.5 }    }
  parser.set_value("database.ports", TOMLValue::Array(Rc::new(vec![
    TOMLValue::Array(Rc::new(vec![
      TOMLValue::String("1 thousand".into(), StrType::MLLiteral),
      TOMLValue::String("3 hundred 4".into(), StrType::MLBasic)
    ])),
    TOMLValue::Array(Rc::new(vec![
      TOMLValue::String("five 100 7".into(), StrType::Basic),
      TOMLValue::Integer("100".into())
    ])),
    TOMLValue::String("element".into(), StrType::Basic),
    TOMLValue::Integer("99".into()),
    TOMLValue::String("index".into(), StrType::Basic),
    TOMLValue::Integer("98".into()),
  ])));
  
  parser.set_value("database.servers", TOMLValue::InlineTable(Rc::new(vec![
    ("woo".into(), TOMLValue::Integer("1928".into())),
    ("hoo".into(), TOMLValue::Float("321.987".into())),
    ("key1".into(), TOMLValue::InlineTable(Rc::new(vec![
      ("key2".into(), TOMLValue::String("new value".into(), StrType::Literal)),
      ("key3".into(), TOMLValue::String("another new value".into(), StrType::Basic)),
      ("key4".into(), TOMLValue::String("A third new value".into(), StrType::Literal)),
      ("key5".into(), TOMLValue::String("fourth value".into(), StrType::Basic))
    ]))),
    ("key6".into(), TOMLValue::Integer("2010".into())),
    ("key7".into(), TOMLValue::Float("3.14159".into()))
  ])));
  
  println!("after: {}", parser.get_value("title").unwrap());
  println!("after: {}", parser.get_value("owner.name").unwrap());
  println!("after: {}", parser.get_value("owner.dob").unwrap());
  println!("after: {}", parser.get_value("database.connection_max").unwrap());
  println!("after: {}", parser.get_value("database.ports").unwrap());
  println!("after: {}", parser.get_value("database.servers").unwrap());
  
  
  println!("{}", parser);
  // let (tmp, new_str) = parser.add_string(new_owner);
  // let parser = tmp;
  //parser.set_value("owner.name",
    //TOMLValue::String(new_str, StrType::Basic));
 }