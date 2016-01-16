use ast::structs::Comment;
use parser::Parser;

fn not_eol(chr: char) -> bool {
  chr as u32 == 0x09 || (chr as u32 >= 0x20 && chr as u32 <= 0x10FFF)
}

impl<'a> Parser<'a> {
  // Newline
  method!(pub newline<&mut Parser,&str, &str>,
    chain!(
   string: alt!(
        complete!(tag_s!("\r\n")) |
        complete!(tag_s!("\n"))
      ),
      ||{self.line_count += 1; string}
    )
  );

  // Whitespace
  method!(pub ws<&mut Parser,&str, &str>, re_find!("^( |\t)*"));

  // Comment
  method!(pub comment<&mut Parser,&str, Comment>,
    chain!(
               tag_s!("#")            ~
  comment_txt: take_while_s!(self.not_eol) ,
      ||{
        Comment{
          text: comment_txt
        }
      }
    )
  );
}

#[cfg(test)]
mod test {
  use nom::IResult::Done;
  use parser::Parser;
  use ast::structs::Comment;

  #[test]
  fn test_newline() {
    let p = Parser::new();
    assert_eq!(p.newline("\r\n"), Done("", "\r\n"));
    assert_eq!(p.newline("\n"), Done("", "\n"));
  }

  #[test]
  fn test_ws() {
    let p = Parser::new();
    assert_eq!(p.ws(" \t  "), Done("", " \t  "));
  }

  #[test]
  fn test_comment() {
    let p = Parser::new();
    assert_eq!(p.comment("# Hèřè'ƨ ₥¥ çô₥₥èñƭ. -?#word"),
      Done("", Comment{text: " Hèřè'ƨ ₥¥ çô₥₥èñƭ. -?#word"}));
  }
}