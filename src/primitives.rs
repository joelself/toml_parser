use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;
use std::collections::hash_map::Entry;
use ast::structs::{KeyVal, WSSep, Value, ErrorCode,
                   HashValue, TableType, Table,
                   get_last_keys};
use ::types::{Date, Time, DateTime, TimeOffset, TimeOffsetAmount, ParseError, StrType,
             Bool, Children, TOMLValue};
use parser::{Parser, Key, count_lines};
use nom;
use nom::{IResult, InputLength};
// TODO LIST:
// Make sure empty key is accepted
// Allow Date only. Right now we require time and offset for a full date-time
//
#[inline(always)]
fn is_keychar(chr: char) -> bool {
  let uchr = chr as u32;
  uchr >= 0x41 && uchr <= 0x5A || // A-Z
  uchr >= 0x61 && uchr <= 0x7A || // a-z
  uchr >= 0x30 && uchr <= 0x39 || // 0-9
  uchr == 0x2D || uchr == 0x5f    // "-", "_"
}

impl<'a> Parser<'a> {
  pub fn get_key_parent(tables: &RefCell<Vec<Rc<TableType<'a>>>>,
    tables_index: &RefCell<Vec<usize>>) -> String {
    let mut key_parent: String = String::new();
    let last_table: Option<&Table> = None;
    let tables_len = tables_index.borrow().len();
    let mut prev_end = 0;
    for i in 0..tables_len {
      if let &TableType::Array(ref t) = &*tables.borrow()[i] {
        if key_parent.len() > 0 {
          key_parent.push('.');
        }
        let keys = get_last_keys(last_table, t);
        for j in prev_end..keys.len() {
          if keys[j] != "$Root$" {
            key_parent.push_str(&keys[j]);
            if j < keys.len() - 1 {
              key_parent.push('.');
            }
          }
        }
        prev_end = keys.len();
        let index = tables_index.borrow()[i];
        if i < tables_len - 1 {
          key_parent.push_str(&format!("[{}]", index));
        }
      }
    }
    key_parent
  }


  pub fn get_array_table_key(map: &RefCell<&mut HashMap<String, HashValue<'a>>>, tables: &RefCell<Vec<Rc<TableType<'a>>>>,
    tables_index: &RefCell<Vec<usize>>) -> String {
    let mut full_key: String = String::new();
    let last_table: Option<&Table> = None;
    let tables_len = tables_index.borrow().len();
    let mut prev_end = 0;
    for i in 0..tables_len {
      match  &*tables.borrow()[i] {
        &TableType::Array(ref t) => {
          debug!("Array Table: {}", t);
          let keys = get_last_keys(last_table, t);
          let len = keys.len();
          for j in prev_end..len - 1 {
            debug!("keys[{}] = {}", j, keys[j]);
            full_key.push_str(&keys[j]);
            let map_borrow = map.borrow();
            let hash_value_opt = map_borrow.get(&full_key);
            match hash_value_opt {
              Some(hash_value) =>  {
                match hash_value.subkeys {
                  Children::Count(ref c) => {
                    if full_key == "$Root$" {
                      full_key.truncate(0);
                    } else {
                      full_key.push_str(&format!("[{}].", c.get()))
                    }
                  },
                  Children::Keys(ref hs_rf) => {
                    if hs_rf.borrow().contains(&keys[j+1]) {
                      if full_key == "$Root$" {
                        full_key.truncate(0);
                      } else {
                        full_key.push('.');
                      }
                    } else {
                      panic!("Key does not exist in map: \"{}\"", &format!("{}.{}", full_key, &keys[j+1]));
                    }
                  },
                }
              }
              None => panic!("Key does not exist in map: \"{}\"", &full_key),
            }
          }
          prev_end = len;
          full_key.push_str(&keys[len-1]);
          let index = tables_index.borrow()[i];
          if i < tables_len - 1 {
            full_key.push_str(&format!("[{}].", index));
          } else {
            full_key.push_str(&format!("[{}]", index));
          }
        },
        &TableType::Standard(ref t) => {
          // Standard tables can't be nested so this has to be the last table in the vector
          let keys = get_last_keys(last_table, t);
          for j in prev_end..keys.len() - 1 {
            if keys[j] == "$Root$" {
              continue;
            }
            full_key.push_str(&keys[j]);
            full_key.push('.');
          }
          full_key.push_str(&keys[keys.len() - 1]);
        }
      }
    }
    debug!("full_key: {}", full_key);
    full_key
  }

  fn get_keychain_key(keychain: &RefCell<Vec<Key<'a>>>) -> (String, String) {
    let len = keychain.borrow().len();
    let mut key = String::new();
    let mut parent_key = String::new();
    for i in 0..len {
      match &keychain.borrow()[i] {
        &Key::Str(ref str_str) => {
          if key.len() > 0 {
            key.push('.');
          }
          key.push_str(str_str)
        },
        &Key::Index(ref i) => key.push_str(&format!("[{}]", i.get())),
      }
      if len > 1 && i == len - 2 {
        parent_key = key.clone();
      }
    }
    return (key, parent_key);
  }

  pub fn get_full_key(map: &RefCell<&mut HashMap<String, HashValue<'a>>>,
    tables: &RefCell<Vec<Rc<TableType<'a>>>>, tables_index: &RefCell<Vec<usize>>,
    keychain: &RefCell<Vec<Key<'a>>>) -> (String, String) {

    let array_key = Parser::get_array_table_key(map, tables, tables_index);
    let (chain_key, parent_chain_key) = Parser::get_keychain_key(keychain);
    debug!("array_key: {}, chain_key: {}, parent_chain_key: {}", array_key, chain_key, parent_chain_key);
    let mut full_key = String::new();
    let mut parent_key = String::new();
    if array_key.len() > 0 {
      full_key.push_str(&array_key);
      full_key.push_str(".");
      parent_key.push_str(&array_key);
      if parent_chain_key.len() > 0 {
        parent_key.push_str(".");
      }
    }
    full_key.push_str(&chain_key);
    parent_key.push_str(&parent_chain_key);
    debug!("full_key: {}, parent_key: {}", full_key, parent_key);
    return (full_key, parent_key);
  }

  pub fn insert_keyval_into_map(&mut self, val: Rc<RefCell<Value<'a>>>) {
    debug!("Insert val: {}", *(*val).borrow());
    let map = RefCell::new(&mut self.map);
    let mut insert = false;
    let mut error = false;
    let mut setvalue = false;
    let full_key: String;
    let mut parent_key: String;
    match &self.last_table {
      // If the last table is None
      //  If the key exists
      //    If the value is empty insert the value
      //    If the value in non-empty add the key/val to the error list
      //  If the key doesn't exist, insert it
      &None => {
        let tuple = Parser::get_keychain_key(&self.keychain);
        full_key = tuple.0;
        parent_key = tuple.1;
        if parent_key == "" {
          parent_key.push_str("$Root$");
        }
        let map_borrow = map.borrow();
        let hv_opt = map_borrow.get(&full_key);
        if let Some(hv) = hv_opt {
          if let Some(_) = hv.value {
            error = true;
          } else {
            setvalue = true;
          }
        } else {
          insert = true;
        }
      },
      // If the last table was a StandardTable or ArrayTable:
      //  If the key exists
      //    If the value is empty, insert the value
      //    If the value is non-empty add the key/val pair to the error list
      //    If the key is for an ArrayOfTables add the key/val to the error list
      //  If the key doesn't exist add the key/value pair to the hash table
      &Some(ref ttype) => {
        match **ttype {
          TableType::Standard(_) => {
            self.last_array_tables.borrow_mut().push(ttype.clone());
            let tuple = Parser::get_full_key(&map, &self.last_array_tables,
              &self.last_array_tables_index, &self.keychain);
            full_key = tuple.0;
            parent_key = tuple.1;
            self.last_array_tables.borrow_mut().pop();
            let map_borrow = map.borrow();
            let hv_opt = map_borrow.get(&full_key);
            if let Some(hv) = hv_opt {
              if let Some(_) = hv.value {
                error = true;
              } else {
                setvalue = true;
              }
            } else {
              insert = true;
            }
          },
          TableType::Array(_) => {
            let tuple = Parser::get_full_key(&map, &self.last_array_tables,
              &self.last_array_tables_index, &self.keychain);
            full_key = tuple.0;
            parent_key = tuple.1;
            let map_borrow = map.borrow();
            let hv_opt = map_borrow.get(&full_key);
            if let Some(hv) = hv_opt {
              if let Some(_) = hv.value {
                error = true;
              } else {
                setvalue = true;
              }
            } else {
              insert = true;
            }
          },
        }
      }
    }

    if error {
      debug!("Error: {}", *(*val).borrow());
      self.errors.borrow_mut().push(ParseError::DuplicateKey(
        full_key, self.line_count.get() ,to_tval!(&*val.borrow())
      ));
    } else if setvalue  || insert {
      if setvalue {
        debug!("Set existing hash value. full_key: {}, parent_key: {}, val: {}", full_key, parent_key, *(*val).borrow());
        let mut borrow = map.borrow_mut();
        let entry = borrow.entry(full_key.clone());
        match entry {
          Entry::Occupied(mut o) => {
            debug!("Children: {:?}", &o.get_mut().subkeys);
            o.get_mut().value = Some(val.clone());
          },
          _ => panic!("Unreachable! Set existing hash value has no exisiting hash value."),
        }
      } else if insert {
        debug!("Insert full_key: {}, parent_key: {}, val: {}", full_key, parent_key, *(*val).borrow());
        match *val.borrow() {
          Value::InlineTable(_) => map.borrow_mut().insert(full_key.clone(), HashValue::new_keys(val.clone())),
          _                     => map.borrow_mut().insert(full_key.clone(), HashValue::new_count(val.clone())),
        };
      }

      // in either case update the parent and possibly grandparent
      let mut borrow = map.borrow_mut();
      {
        let entry = borrow.entry(parent_key.clone());
        match entry {
          Entry::Occupied(mut o) => {
            debug!("Children: {:?}", &o.get_mut().subkeys);
            match &o.get_mut().subkeys {
              &Children::Count(ref c) => { debug!("parent inc to {}", c.get() + 1); c.set(c.get() + 1) },
              &Children::Keys(ref vec_rf) => {
                if let Key::Str(ref s) = self.keychain.borrow()[self.keychain.borrow().len() - 1] {
                  Parser::insert(vec_rf,s.clone().into_owned());
                }
              },
            }
          },
          Entry::Vacant(v) => {
            debug!("vacant parent");
            if let Key::Index(_) = self.keychain.borrow()[self.keychain.borrow().len() - 1] {
              debug!("initialize to 1");
              v.insert(HashValue::one_count());
            } else if let Key::Str(ref s) = self.keychain.borrow()[self.keychain.borrow().len() - 1] {
              debug!("initialize to string: {}", s);
              v.insert(HashValue::one_keys(s.clone().into_owned()));
            }
          },
        }
      }
    }
  }

  // Integer
  method!(pub integer<Parser<'a>, &'a str, &'a str>, self, re_find!("^((\\+|-)?(([1-9](\\d|(_\\d))+)|\\d))")) ;

  // Float
  method!(pub float<Parser<'a>, &'a str, &'a str>, self,
         re_find!("^(\\+|-)?([1-9](\\d|(_\\d))+|\\d)((\\.\\d(\\d|(_\\d))*)((e|E)(\\+|-)?([1-9](\\d|(_\\d))+|\\d))|(\\.\\d(\\d|(_\\d))*)|((e|E)(\\+|-)?([1-9](\\d|(_\\d))+|\\d)))"));

  // Basic String
  named!(pub quoteless_basic_string<&'a str, &'a str>,
    re_find!("^( |!|[#-\\[]|[\\]-􏿿]|(\\\\\")|(\\\\)|(\\\\/)|(\\b)|(\\f)|(\\n)|(\\r)|(\\t)|(\\\\u[0-9A-Z]{4})|(\\\\U[0-9A-Z]{8}))*?"));
  // Multiline Basic String
  // TODO: Convert this to take_while_s using a function that increments self.linecount
  named!(pub quoteless_ml_basic_string<&'a str, &'a str>,
    re_find!("^([ -\\[]|[\\]-􏿿]|(\\\\\")|(\\\\)|(\\\\/)|(\\b)|(\\f)|(\\n)|(\\r)|(\t)|(\\\\u[0-9A-Z]{4})|(\\\\U[0-9A-Z]{8})|\n|(\r\n)|(\\\\(\n|(\r\n))))*?"));
  // Literal String
  named!(pub quoteless_literal_string<&'a str, &'a str>, re_find!("^(	|[ -&]|[\\(-􏿿])*?"));
  // Multiline Literal String
  // TODO: Convert to take_while_s using a function that increments self.linecount
  named!(pub quoteless_ml_literal_string<&'a str, &'a str>, re_find!("^(	|[ -􏿿]|\n|(\r\n))*?"));

  // Basic String
  method!(raw_basic_string<Parser<'a>, &'a str, &'a str>, self,
    re_find!("^\"( |!|[#-\\[]|[\\]-􏿿]|(\\\\\")|(\\\\)|(\\\\/)|(\\b)|(\\f)|(\\n)|(\\r)|(\\t)|(\\\\u[0-9A-Z]{4})|(\\\\U[0-9A-Z]{8}))*?\""));
  // Multiline Basic String
  // TODO: Convert this to take_while_s using a function that increments self.linecount
  method!(raw_ml_basic_string<Parser<'a>, &'a str, &'a str>, self,
    chain!(
   string: re_find!("^\"\"\"([ -\\[]|[\\]-􏿿]|(\\\\\")|(\\\\)|(\\\\/)|(\\b)|(\\f)|(\\n)|(\\r)|(\t)|(\\\\u[0-9A-Z]{4})|(\\\\U[0-9A-Z]{8})|\n|(\r\n)|(\\\\(\n|(\r\n))))*?\"\"\""),
      ||{self.line_count.set(self.line_count.get() + count_lines(string)); string}
    )
  );
  // Literal String
  method!(raw_literal_string<Parser<'a>, &'a str, &'a str>, self, re_find!("^'(	|[ -&]|[\\(-􏿿])*?'"));
  // Multiline Literal String
  // TODO: Convert to take_while_s using a function that increments self.linecount
  method!(raw_ml_literal_string<Parser<'a>, &'a str, &'a str>, self,
    chain!(
   string: re_find!("^'''(	|[ -􏿿]|\n|(\r\n))*?'''"),
      ||{self.line_count.set(self.line_count.get() + count_lines(string)); string}
    )
  );

  fn ml_basic_string(mut self: Parser<'a>, input: &'a str) -> (Parser<'a>, nom::IResult<&'a str, &'a str>) {
    let (tmp, raw) = self.raw_ml_basic_string(input);
    self = tmp;
    let r = match raw {
      IResult::Done(i, o) => IResult::Done(i, &o["\"\"\"".input_len()..o.input_len()-"\"\"\"".input_len()]),
      IResult::Error(_) => IResult::Error(nom::Err::Code(nom::ErrorKind::Custom(ErrorCode::MLLiteralString as u32))),
      IResult::Incomplete(i) => IResult::Incomplete(i),
    };
    (self, r)
  }

  fn basic_string(mut self: Parser<'a>, input: &'a str) -> (Parser<'a>, nom::IResult<&'a str, &'a str>) {
    let (tmp, raw) = self.raw_basic_string(input);
    self = tmp;
    let r = match raw {
      IResult::Done(i, o) => IResult::Done(i, &o["\"".input_len()..o.input_len()-"\"".input_len()]),
      IResult::Error(_) => IResult::Error(nom::Err::Code(nom::ErrorKind::Custom(ErrorCode::MLLiteralString as u32))),
      IResult::Incomplete(i) => IResult::Incomplete(i),
    };
    (self, r)
  }

  fn ml_literal_string(mut self: Parser<'a>, input: &'a str) -> (Parser<'a>, nom::IResult<&'a str, &'a str>) {
    let (tmp, raw) = self.raw_ml_literal_string(input);
    self = tmp;
    let r = match raw {
      IResult::Done(i, o) => IResult::Done(i, &o["'''".input_len()..o.input_len()-"'''".input_len()]),
      IResult::Error(_) => IResult::Error(nom::Err::Code(nom::ErrorKind::Custom(ErrorCode::MLLiteralString as u32))),
      IResult::Incomplete(i) => IResult::Incomplete(i),
    };
    (self, r)
  }

  fn literal_string(mut self: Parser<'a>, input: &'a str) -> (Parser<'a>, nom::IResult<&'a str, &'a str>) {
    let (tmp, raw) = self.raw_literal_string(input);
    self = tmp;
    let r = match raw {
      IResult::Done(i, o) => IResult::Done(i, &o["'".input_len()..o.input_len()-"'".input_len()]),
      IResult::Error(_) => IResult::Error(nom::Err::Code(nom::ErrorKind::Custom(ErrorCode::MLLiteralString as u32))),
      IResult::Incomplete(i) => IResult::Incomplete(i),
    };
    (self, r)
  }

  method!(string<Parser<'a>, &'a str, Value>, mut self,
    alt!(
      complete!(call_m!(self.ml_literal_string))  => {|ml: &'a str| Value::String(ml.into(), StrType::MLLiteral)}  |
      complete!(call_m!(self.ml_basic_string))    => {|mb: &'a str| Value::String(mb.into(), StrType::MLBasic)}  |
      complete!(call_m!(self.basic_string))       => {|b: &'a str| Value::String(b.into(), StrType::Basic)}    |
      complete!(call_m!(self.literal_string))     => {|l: &'a str| Value::String(l.into(), StrType::Literal)}
    )
  );

  // TODO: Allow alternate casing, but report it as an error
  // Boolean
  method!(boolean<Parser<'a>, &'a str, Bool>, self, alt!(complete!(tag_s!("false")) => {|_| Bool::False} |
                                                         complete!(tag_s!("true"))  => {|_| Bool::True}));


  // Datetime
  // I use re_capture here because I only want the number without the dot. It captures the entire match
  // in the 0th position and the first capture group in the 1st position
  method!(fractional<Parser<'a>, &'a str, Vec<&'a str> >, self, re_capture!("^\\.([0-9]+)"));

  method!(time<Parser<'a>, &'a str, Time>, mut self,
    chain!(
           tag_s!("T")                            ~
     hour: re_find!("^[0-9]{2}")                  ~
            tag_s!(":")                           ~
   minute: re_find!("^[0-9]{2}")                  ~
            tag_s!(":")                           ~
   second: re_find!("^[0-9]{2}")                  ~
 fraction: complete!(call_m!(self.fractional)) ?  ~
   offset: complete!(call_m!(self.time_offset)) ? ,
      ||{
        
        Time::new_str(hour, minute, second, match fraction {
            Some(ref x) => Some(x[1]),
            None        => None,
          },
          offset
        )
      }
    )
  );

  method!(time_offset_amount<Parser<'a>, &'a str, TimeOffsetAmount >, self,
    chain!(
  pos_neg: alt!(complete!(tag_s!("+")) | complete!(tag_s!("-")))  ~
     hour: re_find!("^[0-9]{2}")                                  ~
           tag_s!(":")                                            ~
   minute: re_find!("^[0-9]{2}")                                  ,
      ||{
        TimeOffsetAmount::new_str(pos_neg, hour, minute)
      }
    )
  );

  method!(time_offset<Parser<'a>, &'a str, TimeOffset>, mut self,
    alt!(
      complete!(tag_s!("Z"))                       => {|_|       TimeOffset::Zulu} |
      complete!(call_m!(self.time_offset_amount))  => {|offset|  TimeOffset::Time(offset)}
    )
  );

  method!(date<Parser<'a>, &'a str, Date>, self,
    chain!(
     year: re_find!("^([0-9]{4})") ~
           tag_s!("-") ~
    month: re_find!("^([0-9]{2})") ~
           tag_s!("-") ~
      day: re_find!("^([0-9]{2})"),
      ||{
        Date::new_str(year, month, day)
      }
    )
  );

  method!(pub date_time<Parser<'a>, &'a str, DateTime>, mut self,
    chain!(
     date: call_m!(self.date)       ~
     time: call_m!(self.time) ?     ,
        ||{
          let res = DateTime::new(date, time);
          if !res.validate() {
            self.errors.borrow_mut().push(ParseError::InvalidDateTime(
              Parser::get_full_key(&RefCell::new(& mut self.map), &self.last_array_tables,
                &self.last_array_tables_index, &self.keychain
              ).0, self.line_count.get()
            ));
          }
          res
        }
    )
  );

  // Key-Value pairs
  method!(unquoted_key<Parser<'a>, &'a str, &'a str>, self, take_while1_s!(is_keychar));
  method!(quoted_key<Parser<'a>, &'a str, &'a str>, self,
    re_find!("^\"( |!|[#-\\[]|[\\]-􏿿]|(\\\\\")|(\\\\\\\\)|(\\\\/)|(\\\\b)|(\\\\f)|(\\\\n)|(\\\\r)|(\\\\t)|(\\\\u[0-9A-Z]{4})|(\\\\U[0-9A-Z]{8}))+\""));

  method!(pub key<Parser<'a>, &'a str, &'a str>, mut self, alt!(
    complete!(call_m!(self.quoted_key))   =>  {|k: &'a str| {
      self.keychain.borrow_mut().push(Key::Str(k.into())); k
    }}|
    complete!(call_m!(self.unquoted_key)) =>  {|k: &'a str| {
      self.keychain.borrow_mut().push(Key::Str(k.into())); k}}
  ));

  method!(keyval_sep<Parser<'a>, &'a str, WSSep>, mut self,
    chain!(
      ws1: call_m!(self.ws) ~
           tag_s!("=")      ~
      ws2: call_m!(self.ws) ,
      ||{
        WSSep::new_str(ws1, ws2)
      }     
    )
  );

  method!(pub val<Parser<'a>, &'a str, Rc<RefCell<Value>> >, mut self,
    alt!(
      complete!(call_m!(self.array))        => {|arr|           Rc::new(RefCell::new(Value::Array(arr)))}             |
      complete!(call_m!(self.inline_table)) => {|it|            Rc::new(RefCell::new(Value::InlineTable(it)))}        |
      complete!(call_m!(self.date_time))    => {|dt|            Rc::new(RefCell::new(Value::DateTime(dt)))}           |
      complete!(call_m!(self.float))        => {|flt: &'a str|  Rc::new(RefCell::new(Value::Float(flt.into())))}   |
      complete!(call_m!(self.integer))      => {|int: &'a str|  Rc::new(RefCell::new(Value::Integer(int.into())))} |
      complete!(call_m!(self.boolean))      => {|b|             Rc::new(RefCell::new(Value::Boolean(b)))}             |
      complete!(call_m!(self.string))       => {|s|             Rc::new(RefCell::new(s))}
    )
  );

  method!(pub keyval<Parser<'a>, &'a str, KeyVal>, mut self,
    chain!(
      key: call_m!(self.key)        ~
       ws: call_m!(self.keyval_sep) ~
      val: call_m!(self.val)        ,
      || {
        let res = KeyVal::new_str(key, ws, val);
        if self.array_error.get() {
          debug!("array_error");
          let err = self.errors.borrow_mut().pop().unwrap();
          if let ParseError::InvalidTable(_, _, ref map) = err {
            debug!("InvalidTable");
            map.borrow_mut().insert(res.key.to_string(), to_tval!(&*res.val.borrow()));
          }
          self.errors.borrow_mut().push(err);
        } else {
          match *res.val.borrow() {
            _ => self.insert_keyval_into_map(res.val.clone()),
          }
        }
        self.keychain.borrow_mut().pop();
        res
      }
    )
  );
}

#[cfg(test)]
mod test {
  use nom::IResult::Done;
  use ast::structs::{WSSep, Array, ArrayValue, KeyVal,
                     InlineTable, TableKeyVal, Value,
                     CommentOrNewLines};
  use ::types::{DateTime, Time, Date, TimeOffsetAmount, TimeOffset, StrType, Bool, Str};
  use parser::Parser;
  use std::rc::Rc;
  use std::cell::RefCell;

  #[test]
  fn test_integer() {
    let p = Parser::new();
    assert_eq!(p.integer("345_12_678").1, Done("", "345_12_678"));
  }

  #[test]
  fn test_float() {
    let p = Parser::new();
    assert_eq!(p.float("98_7.2_34e-8_8").1, Done("", "98_7.2_34e-8_8"));
  }

  #[test]
  fn test_basic_string() {
    let p = Parser::new();
    assert_eq!(p.basic_string("\"TÎ»Ã¯Æ¨ Ã¯Æ¨ Ã¡ Î²Ã¡Æ¨Ã¯Ã§ Æ¨Æ­Å™Ã¯Ã±Ï±.\"").1, Done("", "TÎ»Ã¯Æ¨ Ã¯Æ¨ Ã¡ Î²Ã¡Æ¨Ã¯Ã§ Æ¨Æ­Å™Ã¯Ã±Ï±."));
  }

  #[test]
  fn test_ml_basic_string() {
    let p = Parser::new();
    assert_eq!(p.ml_basic_string("\"\"\"Â£Ã¯Ã±Ã¨ Ã“Ã±Ã¨
Â£Ã¯Ã±Ã¨ TÏ‰Ã´
Â£Ã¯Ã±Ã¨ TÎ»Å™Ã¨Ã¨\"\"\"").1, Done("", r#"Â£Ã¯Ã±Ã¨ Ã“Ã±Ã¨
Â£Ã¯Ã±Ã¨ TÏ‰Ã´
Â£Ã¯Ã±Ã¨ TÎ»Å™Ã¨Ã¨"# ));
  }

  #[test]
  fn test_literal_string() {
    let p = Parser::new();
    assert_eq!(p.literal_string("'Abc ÑŸ'").1, Done("", "Abc ÑŸ")); 
  }

  #[test]
  fn test_ml_literal_string() {
    let p = Parser::new();
    assert_eq!(p.ml_literal_string(r#"'''
                                    Abc ÑŸ
                                    '''"#).1,
      Done("", r#"
                                    Abc ÑŸ
                                    "#));
  }

  #[test]
  fn test_string() {
    let mut p = Parser::new();
    assert_eq!(p.string("\"Î²Ã¡Æ¨Ã¯Ã§_Æ¨Æ­Å™Ã¯Ã±Ï±\"").1, Done("", Value::String(Str::Str("Î²Ã¡Æ¨Ã¯Ã§_Æ¨Æ­Å™Ã¯Ã±Ï±"), StrType::Basic)));
    p = Parser::new();
    assert_eq!(p.string(r#""""â‚¥â„“_Î²Ã¡Æ¨Ã¯Ã§_Æ¨Æ­Å™Ã¯Ã±Ï±
Ã±Ãºâ‚¥Î²Ã¨Å™_Æ­Ï‰Ã´
NÃ›MÃŸÃ‰R-THRÃ‰Ã‰
""""#).1, Done("", Value::String(Str::Str(r#"â‚¥â„“_Î²Ã¡Æ¨Ã¯Ã§_Æ¨Æ­Å™Ã¯Ã±Ï±
Ã±Ãºâ‚¥Î²Ã¨Å™_Æ­Ï‰Ã´
NÃ›MÃŸÃ‰R-THRÃ‰Ã‰
"#), StrType::MLBasic)));
    p = Parser::new();
    assert_eq!(p.string("'Â£ÃŒTÃ‰RÃ‚Â£Â§TRÃ¯NG'").1, Done("", Value::String(Str::Str("Â£ÃŒTÃ‰RÃ‚Â£Â§TRÃ¯NG"), StrType::Literal)));
    p = Parser::new();
    assert_eq!(p.string(r#"'''Â§Æ¥Å™Ã¯Æ­Ã¨
Ã‡Ã´Æ™Ã¨
ÃžÃ¨Æ¥Æ¨Ã¯
'''"#).1,
      Done("", Value::String(Str::Str(r#"Â§Æ¥Å™Ã¯Æ­Ã¨
Ã‡Ã´Æ™Ã¨
ÃžÃ¨Æ¥Æ¨Ã¯
"#), StrType::MLLiteral)));
  }

  #[test]
  fn test_boolean() {
    let mut p = Parser::new();
    assert_eq!(p.boolean("true").1, Done("", Bool::True));
    p = Parser::new();
    assert_eq!(p.boolean("false").1, Done("", Bool::False));
  }

  #[test]
  fn test_fractional() {
    let p = Parser::new();
    assert_eq!(p.fractional(".03856").1, Done("", vec![".03856", "03856"]));
  }

  #[test]
  fn test_time() {
    let mut p = Parser::new();
    assert_eq!(p.time("T11:22:33.456").1,
      Done("", Time::new_str("11", "22", "33", Some("456"), None)));
    p = Parser::new();
    assert_eq!(p.time("T04:05:06").1,
      Done("", Time::new_str("04", "05", "06", None, None)));
  }

  #[test]
  fn test_time_offset_amount() {
    let p = Parser::new();
    assert_eq!(p.time_offset_amount("+12:34").1,
      Done("", TimeOffsetAmount::new_str("+", "12", "34"))
    );
  }

  #[test]
  fn test_time_offset() {
    let mut p = Parser::new();
    assert_eq!(p.time_offset("+12:34").1,
      Done("", TimeOffset::Time(TimeOffsetAmount::new_str("+", "12", "34")))
    );
    p = Parser::new();
    assert_eq!(p.time_offset("Z").1, Done("", TimeOffset::Zulu));
  }

  #[test]
  fn test_full_date() {
    let p = Parser::new();
    assert_eq!(p.date("1942-12-07").1,
      Done("", Date::new_str("1942", "12", "07"))
    );
  }

  #[test]
  fn test_date_time() {
    let p = Parser::new();
    assert_eq!(p.date_time("1999-03-21T20:15:44.5-07:00").1,
      Done("", DateTime::new(Date::new_str("1999", "03", "21"),
        Some(Time::new_str("20", "15", "44", Some("5"),
          Some(TimeOffset::Time(TimeOffsetAmount::new_str("-", "07", "00")))
      ))))
    );
  }

  #[test]
  fn test_unquoted_key() {
    let p = Parser::new();
    assert_eq!(p.unquoted_key("Un-Quoted_Key").1, Done("", "Un-Quoted_Key"));
  }

  #[test]
  fn test_quoted_key() {
    let p = Parser::new();
    assert_eq!(p.quoted_key("\"QÃºÃ´Æ­Ã¨Î´KÃ¨Â¥\"").1, Done("", "\"QÃºÃ´Æ­Ã¨Î´KÃ¨Â¥\""));
  }

  #[test]
  fn test_key() {
    let mut p = Parser::new();
    assert_eq!(p.key("\"GÅ™Ã¡Æ¥Ã¨Æ’Å™ÃºÃ¯Æ­\"").1, Done("", "\"GÅ™Ã¡Æ¥Ã¨Æ’Å™ÃºÃ¯Æ­\""));
    p = Parser::new();
    assert_eq!(p.key("_is-key").1, Done("", "_is-key"));
  }

  #[test]
  fn test_keyval_sep() {
    let p = Parser::new();
    assert_eq!(p.keyval_sep("\t \t= \t").1, Done("", WSSep::new_str("\t \t", " \t")));
  }

  #[test]
  fn test_val() {
    let mut p = Parser::new();
    assert_eq!(p.val("[4,9]").1, Done("",
      Rc::new(RefCell::new(Value::Array(Rc::new(RefCell::new(Array::new(
        vec![
          ArrayValue::new(
            Rc::new(RefCell::new(Value::Integer(Str::Str("4")))), Some(WSSep::new_str("", "")),
            vec![CommentOrNewLines::NewLines(Str::Str(""))]
          ),
          ArrayValue::new(
            Rc::new(RefCell::new(Value::Integer(Str::Str("9")))), None,
            vec![CommentOrNewLines::NewLines(Str::Str(""))]
          ),
        ],
        vec![CommentOrNewLines::NewLines(Str::Str(""))], vec![CommentOrNewLines::NewLines(Str::Str(""))]
      ))
    ))))));
    p = Parser::new();
    assert_eq!(p.val("{\"Â§Ã´â‚¥Ã¨ ÃžÃ¯Ï±\"='TÃ¡Æ¨Æ­Â¥ ÃžÃ´Å™Æ™'}").1, Done("",
      Rc::new(RefCell::new(Value::InlineTable(Rc::new(RefCell::new(InlineTable::new(
        vec![
          TableKeyVal::new(
            KeyVal::new_str(
              "\"Â§Ã´â‚¥Ã¨ ÃžÃ¯Ï±\"", WSSep::new_str("", ""),
              Rc::new(RefCell::new(Value::String(Str::Str("TÃ¡Æ¨Æ­Â¥ ÃžÃ´Å™Æ™"), StrType::Literal)))
            ),
            None,
            vec![]
          )
        ],
        WSSep::new_str("", "")
    ))))))));
    p = Parser::new();
    assert_eq!(p.val("2112-09-30T12:33:01.345-11:30").1, Done("", Rc::new(RefCell::new(Value::DateTime(
      DateTime::new(Date::new_str("2112", "09", "30"), Some(Time::new_str("12", "33", "01", Some("345"),
        Some(TimeOffset::Time(TimeOffsetAmount::new_str("-", "11", "30"))
    )))))))));
    p = Parser::new();
    assert_eq!(p.val("3487.3289E+22").1, Done("", Rc::new(RefCell::new(Value::Float(Str::Str("3487.3289E+22"))))));
    p = Parser::new();
    assert_eq!(p.val("8932838").1, Done("", Rc::new(RefCell::new(Value::Integer(Str::Str("8932838"))))));
    p = Parser::new();
    assert_eq!(p.val("false").1, Done("", Rc::new(RefCell::new(Value::Boolean(Bool::False)))));
    p = Parser::new();
    assert_eq!(p.val("true").1, Done("", Rc::new(RefCell::new(Value::Boolean(Bool::True)))));
    p = Parser::new();
    assert_eq!(p.val("'Â§Ã´â‚¥Ã¨ Â§Æ­Å™Ã¯Ã±Ï±'").1, Done("", Rc::new(RefCell::new(Value::String(Str::Str("Â§Ã´â‚¥Ã¨ Â§Æ­Å™Ã¯Ã±Ï±"), StrType::Literal)))));
  }

  #[test]
  fn test_keyval() {
    let p = Parser::new();
    assert_eq!(p.keyval("Boolean = 84.67").1, Done("", KeyVal::new_str(
      "Boolean", WSSep::new_str(" ", " "),
      Rc::new(RefCell::new(Value::Float(Str::Str("84.67"))))
    )));
  }
}