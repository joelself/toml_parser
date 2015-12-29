#![allow(dead_code)]
mod ast;
#[macro_use]
extern crate nom;
use std::str;

fn is_space(chr: u8) -> bool {
    chr as u8 == 0x20 || chr as u8 == 0x09
}

/*fn is_digit(chr: u8) -> bool {
  chr as u8 >= 0x30 && chr as u8 <= 0x39
}

fn is_escape_char(chr: u8) -> bool {
    chr as char == '0' || chr as char == 't' || chr as char == 'n' || chr as char == 'r' || chr as char == '"' || chr as char == '\\'
}*/

named!(space<&[u8], &[u8]>, alt!( tag!(" ") | tag!("\t") )); // space
named!(whitespace<&[u8], &[u8]>, take_while!(is_space)); // ws
named!(newline<&[u8], &[u8]>, alt!(tag!("\r\n") | tag!("\n"))); //nl
named!(key_segment<&[u8], &[u8]>, take_until!("[].")); // key_segment
named!(dotted_key_segment<&[u8], &[u8]>,
       chain!(
           tag!(".") ~
  segment: key_segment,
           || {segment}
       )
);
named!(dotted_key_segments< Vec<&[u8]> >, many0!(dotted_key_segment));
named!(key_name<&[u8], Vec<&[u8]> >,
       chain!(
  segment: key_segment ~
 segments: dotted_key_segments,
           || {let mut v = vec![segment];
               v.extend_from_slice(&*segments);
               v}
       )
);

named!(comment<&[u8], &[u8]>,
       chain!(
           tag!("#") ~
   result: alt!(take_until!("\r\n") |
                take_until!("\n")),
           || {result}
       )
); // comment

named!(ignorable<&[u8], &[u8]>, alt!( comment | space | newline ));
named!(ignore<&[u8], Vec<&[u8]> >, many0!(ignorable)); // ignore
named!(line_end<&[u8], ast::LineEnd>,
       chain!(
       ws: whitespace ~
  comment: comment? ~
       nl: newline ,
           || {ast::LineEnd{ws: str::from_utf8(ws).unwrap(),
                           comment: str::from_utf8(comment.unwrap()).unwrap(),
                           nl: str::from_utf8(nl).unwrap()}}
       )
); // line_end

fn main() {
    let r = line_end(b" \t #this is comment\r\n");
    println!("{:?}", r);
    //assert_eq!(r, Done(b"X", true));
}
