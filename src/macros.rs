use types::{Str, TOMLValue};
use ast::structs::Value;
use parser::Parser;
#[macro_export]
macro_rules! str (
    ($s:expr) => (
        match $s {
        	Str::Str(s) => s,
        	Str::String(ref s) => s,
        };
    );
);

#[macro_export]
macro_rules! string (
    ($s:expr) => (
        match $s {
        	Str::Str(s) => s.to_string(),
        	Str::String(ref s) => s.clone(),
        };
    );
);

#[macro_export]
macro_rules! to_tval(
    ($tval:expr) => (
     match $tval {
         &Value::Integer(ref v) => TOMLValue::Integer(v.clone()),
         &Value::Float(ref v) => TOMLValue::Float(v.clone()),
         &Value::Boolean(v) => TOMLValue::Boolean(v),
         &Value::DateTime(ref v) => TOMLValue::DateTime(v.clone()),
         &Value::Array(ref arr) => Parser::sanitize_array(arr.clone()),
         &Value::String(ref s, t) => TOMLValue::String(s.clone(), t.clone()),
         &Value::InlineTable(ref it) => Parser::sanitize_inline_table(it.clone()),
     }
    );
);