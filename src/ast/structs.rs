#[derive(Debug)]
pub struct LineEnd<'a> {
    pub ws: &'a str,
    pub comment: &'a str,
    pub nl: &'a str,
}
#[derive(Debug)]
pub struct Comment<'a> {
	pub text: &'a str,
}
#[derive(Debug)]
pub struct WSSep<'a> {
	pub ws1: &'a str,
	pub ws2: &'a str,
}
#[derive(Debug)]
pub struct KeyVal<'a> {
	pub key: &'a str,
	pub keyval_sep: WSSep<'a>,
	pub val: &'a str,
}
#[derive(Debug)]
pub struct WSKeySep<'a> {
	pub ws: WSSep<'a>,
	pub key: &'a str,
}
#[derive(Debug)]
pub enum TableType {
	Standard,
	Array,
}
#[derive(Debug)]
pub struct Table<'a> {
    pub ttype: TableType,
	pub ws: WSSep<'a>, // opening whitespace and closing whitespace
	pub key: &'a str,
	pub subkeys: Vec<WSKeySep<'a>>,
}