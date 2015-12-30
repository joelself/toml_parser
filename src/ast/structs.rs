
#[derive(Debug)]
pub enum TableType {
	Standard,
	Array,
}

// #<text>
#[derive(Debug)]
pub struct Comment<'a> {
	pub text: &'a str,
}

#[derive(Debug)]
pub struct WSSep<'a> {
	pub ws1: &'a str,
	pub ws2: &'a str,
}

// <key><keyval_sep.ws1>=<keyval_sep.ws2><val>
#[derive(Debug)]
pub struct KeyVal<'a> {
	pub key: &'a str,
	pub keyval_sep: WSSep<'a>,
	pub val: &'a str,
}

// <ws.ws1>.<ws.ws2><key>
#[derive(Debug)]
pub struct WSKeySep<'a> {
	pub ws: WSSep<'a>,
	pub key: &'a str,
}

// Standard: [<ws.ws1><key><subkeys*><ws.ws2>]
// Array: [[<ws.ws1><key><subkeys*><ws.ws2>]]
#[derive(Debug)]
pub struct Table<'a> {
    pub ttype: TableType,
	pub ws: WSSep<'a>, // opening whitespace and closing whitespace
	pub key: &'a str,
	pub subkeys: Vec<WSKeySep<'a>>,
}

// <hour>:<minute>:<second>(.<fraction>)?
#[derive(Debug)]
#[derive(Eq)]
pub struct PartialTime<'a> {
    pub hour: &'a str,
	pub minute: &'a str,
	pub second: &'a str,
	pub fraction: &'a str, // should only have one element if it is Some
}

impl<'a> PartialEq for PartialTime<'a> {
	fn eq(&self, other: &PartialTime<'a>) -> bool {
		self.hour == other.hour &&
		self.minute == other.minute &&
		self.second == other.second &&
		self.fraction == other.fraction
	}
}
