extern crate tomlmodlib;
use tomlmodlib::parser::inline_table;
fn main() {
  println!("{:?}", inline_table("{A_Key\t= \"A Value\", \"Another Key\"=3.45e+3 }"));
}