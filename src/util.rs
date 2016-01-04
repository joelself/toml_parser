use std::fmt;
use std::fmt::{Display};
use ast::structs::{Comment};

// Newline
named!(pub newline<&str, &str>,
  alt!(
    complete!(tag_s!("\r\n")) |
    complete!(tag_s!("\n"))
  )
);

named!(newlines<&str, Vec<&str> >, many1!(newline));

// Whitespace
named!(pub ws<&str, &str>, re_find_static!("^( |\t)*"));

// Comment
fn not_eol(chr: char) -> bool {
  chr as u32 == 0x09 || (chr as u32 >= 0x20 && chr as u32 <= 0x10FFF)
}

named!(pub comment<&str, Comment>,
  chain!(
             re_find_static!("^#")            ~
comment_txt: take_while_s!(not_eol) ,
    ||{
      Comment{
        text: comment_txt
      }
    }
  )
);