use ast::structs::Comment;
use parser::LINE_COUNT;
// Newline
named!(pub newline<&str, &str>,
  chain!(
 string: alt!(
      complete!(tag_s!("\r\n")) |
      complete!(tag_s!("\n"))
    ),
    ||{LINE_COUNT.with(|f| *f.borrow_mut() = *f.borrow() + 1); string}
  )
);

// Whitespace
named!(pub ws<&str, &str>, re_find!("^( |\t)*"));

// Comment
fn not_eol(chr: char) -> bool {
  chr as u32 == 0x09 || (chr as u32 >= 0x20 && chr as u32 <= 0x10FFF)
}

named!(pub comment<&str, Comment>,
  chain!(
             tag_s!("#")            ~
comment_txt: take_while_s!(not_eol) ,
    ||{
      Comment{
        text: comment_txt
      }
    }
  )
);

#[cfg(test)]
mod test {
  use nom::IResult::Done;
  use super::{newline, ws, comment};
  use ast::structs::Comment;

  #[test]
  fn test_newline() {
    assert_eq!(newline("\r\n"), Done("", "\r\n"));
    assert_eq!(newline("\n"), Done("", "\n"));
  }

  #[test]
  fn test_ws() {
    assert_eq!(ws(" \t  "), Done("", " \t  "));
  }

  #[test]
  fn test_comment() {
    assert_eq!(comment("# Hèřè'ƨ ₥¥ çô₥₥èñƭ. -?#word"),
      Done("", Comment{text: " Hèřè'ƨ ₥¥ çô₥₥èñƭ. -?#word"}));
  }
}