#![allow(dead_code)]
mod ast;
#[macro_use]
extern crate nom;
extern crate regex;
// TOML
// TODO: toml
// TODO: expression

// Newline
named!(newline<&str, &str>, alt!(tag_s!("\n") | tag_s!("\r\n")));

named!(newlines<&str, Vec<&str> >, many1!(newline));

// Whitespace
fn is_space(chr: char) -> bool {
    chr as u32 == 0x20 || chr as u32 == 0x09
}

named!(ws<&str, &str>, take_while_s!(is_space));

// Comment
fn not_eol(chr: char) -> bool {
  chr as u32 == 0x09 || (chr as u32 >= 0x20 && chr as u32 <=0x10FFF)
}

named!(comment<&str, ast::Comment>,
       chain!(
              tag_s!("#") ~
 comment_txt: take_while_s!(not_eol),
              || {ast::Comment{text: comment_txt}}
             )
      );

// Key-Value pairs
fn is_keychar(chr: char) -> bool {
  let uchr = chr as u32;
  uchr >= 0x41 && uchr <= 0x5A || // A-Z
  uchr >= 0x61 && uchr <= 0x7A || // A-Z
  uchr >= 0x30 && uchr <= 0x39 || // 0-9
  uchr == 0x2D || uchr == 0x5f // "-", "_"
}


// named!(basic_unescaped<&str, &str>, re_match!(r" |!|[#-\[]|[\]-􏿿]"));
// named!(escaped<&str, &str>, re_match!("(\\\\\")|(\\\\)|(\\\\/)|(\\b)|(\\f)|(\\n)|(\\r)|(\\t)|(u[0-9A-Z]{4})|(U[0-9A-Z]{8})"));
// named!(basic_char<&str, &str>, alt!(basic_unescaped | escaped));
named!(unquoted_key<&str, &str>, take_while1_s!(is_keychar));
named!(quoted_key<&str, &str>, re_find!("\"( |!|[#-\\[]|[\\]-􏿿]|(\\\\\")|(\\\\)|(\\\\/)|(\\b)|(\\f)|(\\n)|(\\r)|(\\t)|(\\\\u[0-9A-Z]{4})|(\\\\U[0-9A-Z]{8})){1,}\""));

named!(key<&str, &str>, alt!(unquoted_key | quoted_key));
named!(keyval_sep<&str, ast::WSSep>,
       chain!(
         ws1: ws ~
              tag_s!("=") ~
         ws2: ws,
              || {ast::WSSep{ws1: ws1, ws2: ws2}}     
             )
      );
// TODO: named!(val<&str, &str>, ...);
// TODO: named!(keyval<&str, ast::KeyVal>, ...);

// Standard Table
named!(table_sub_key<&str, ast::WSKeySep>,
       chain!(
         ws1: ws ~
              tag_s!(".") ~
         ws2: ws ~
         key: key,
              || {ast::WSKeySep{ws: ast::WSSep{ws1: ws1, ws2: ws2}, key: key}} 
             )
      );
named!(table_sub_keys<&str, Vec<ast::WSKeySep> >, many0!(table_sub_key));
named!(std_table<&str, ast::Table>,
       chain!(
              tag_s!("[") ~
         ws1: ws ~
         key: key ~
     subkeys: table_sub_keys ~
         ws2: ws ~
              tag_s!("]"),
              || {ast::Table{ttype: ast::TableType::Standard,
                             ws: ast::WSSep{ws1: ws1, ws2: ws2},
                             key: key, subkeys: subkeys}}
             )
      );

// Array Table
named!(array_table<&str, ast::Table>,
       chain!(
              tag_s!("[[") ~
         ws1: ws ~
         key: key ~
     subkeys: table_sub_keys ~
         ws2: ws ~
              tag_s!("]]"),
              || {ast::Table{ttype: ast::TableType::Array,
                             ws: ast::WSSep{ws1: ws1, ws2: ws2},
                             key: key, subkeys: subkeys}}
             )
      );

// Integer
named!(integer<&str, &str>, re_find!("(\\+|-)?([1-9](\\d|(_\\d))+|\\d)"));

// Float
named!(float<&str, &str>,
       re_find!("(\\+|-)?([1-9](\\d|(_\\d))+|\\d)((\\.\\d(\\d|(_\\d))*)((e|E)(\\+|-)?([1-9](\\d|(_\\d))+|\\d))|(\\.\\d(\\d|(_\\d))*)|((e|E)(\\+|-)?([1-9](\\d|(_\\d))+|\\d)))"));

// String
// TODO: named!(string<&str, &str>, alt!(basic_string | ml_basic_string | literal_string | ml_literal_string));

// Basic String
named!(basic_string<&str, &str>,
       re_find!("\"( |!|[#-\\[]|[\\]-􏿿]|(\\\\\")|(\\\\)|(\\\\/)|(\\b)|(\\f)|(\\n)|(\\r)|(\\t)|(\\\\u[0-9A-Z]{4})|(\\\\U[0-9A-Z]{8})){0,}\""));

// Multiline Basic String
named!(ml_basic_string<&str, &str>,
       re_find!("\"\"\"([ -\\[]|[\\]-􏿿]|(\\\\\")|(\\\\)|(\\\\/)|(\\b)|(\\f)|(\\n)|(\\r)|(\\t)|(\\\\u[0-9A-Z]{4})|(\\\\U[0-9A-Z]{8})|\n|(\r\n)|(\\\\(\n|(\r\n))))*\"\"\""));

// Literal String
named!(literal_string<&str, &str>,
       re_find!("'(	|[ -&]|[\\(-􏿿])*'"));

//Multiline Literal String
// TODO: finish this function
named!(ml_literal_string<&str, &str>, 
	   re_find("'''(	|)*'''"));
fn main() {
    let s = r#""""
             HOH\\nA#[]
             """"#;
    let r = ml_basic_string(s);
    println!("{:?}", r);
}

#[cfg(test)]
mod test {
	use nom::IResult::{Done};
	use ::literal_string;
	#[test]
	fn test_literal_string() {
		assert_eq!(literal_string("'Abc џ'"), Done("", "'Abc џ'"));	
	}
}