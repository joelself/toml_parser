extern crate tomllib;
use tomllib::parser::Parser;
fn main() {
    //println!("Hello in English: {}", tomllib::ast::structs::hello());
    let mut p = Parser::new();
    p.parse(r#"key=[43]"#);
	println!("{}", p);
}