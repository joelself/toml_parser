extern crate tomllib;
use tomllib::ast;
fn main() {
    //println!("Hello in English: {}", tomllib::ast::structs::hello());
	println!("{}", ast::PartialTime{hour: "09", minute: "30", second: "22", fraction: "733"});
}