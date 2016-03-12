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

#[macro_export]
macro_rules! call_s(
  ($i:expr, $method:path) => ( $method( $i ) );
);

#[macro_export]
macro_rules! res2opt(
  ($i:expr) => (
    match $i {
      Result::Ok(t) => Some(t),
      Result::Err(_) => None,  
    }
  );
);