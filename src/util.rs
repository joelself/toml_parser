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

fn eol(chr: char) -> bool {
  (chr as u32) < 0x09 || ((chr as u32) > 0x09 && (chr as u32) < 0x20) || (chr as u32) > 0x10FFF
}

named!(pub comment<&str, Comment>,
  chain!(
             re_find_static!("^#")  ~
comment_txt: take_while_s!(not_eol) ~
         nl: newline                ,
    ||{
      Comment{
        text: comment_txt, nl: nl
      }
    }
  )
);

mod test {
  use nom::IResult::Done;
  use super::{newline, newlines, ws, comment};
  use ast::structs::Comment;

  #[test]
  fn test_newline() {
    assert_eq!(newline("\r\n"), Done("", "\r\n"));
    assert_eq!(newline("\n"), Done("", "\n"));
  }

  #[test]
  fn test_newlines() {
    assert_eq!(newlines("\n\r\n\n"), Done("", vec!["\n", "\r\n", "\n"]));
  }

  #[test]
  fn test_ws() {
    assert_eq!(ws(" \t  "), Done("", " \t  "));
  }

  #[test]
  fn test_comment() {
    assert_eq!(comment("# Hèřè'ƨ ₥¥ çô₥₥èñƭ. -?#word\n"),
      Done("", Comment{text: " Hèřè'ƨ ₥¥ çô₥₥èñƭ. -?#word", nl: "\n"}));
  }
}