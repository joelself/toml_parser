
#[derive(Debug)]
pub enum TableType {
	Standard,
	Array,
}

#[derive(Debug, Eq)]
pub enum TimeOffset<'a> {
	Z,
	Time(TimeOffsetAmount<'a>),
}

impl<'a> PartialEq for TimeOffset<'a> {
	fn eq(&self, other: &TimeOffset<'a>) -> bool {
		match (self, other) {
			(&TimeOffset::Z, &TimeOffset::Z) => true,
			(&TimeOffset::Time(ref i), &TimeOffset::Time(ref j)) if(i == j) => true,
			_ => false
		}
	}
}

#[derive(Debug, Eq)]
pub enum PosNeg {
	Pos,
	Neg,
}

impl PartialEq for PosNeg {
	fn eq(&self, other: &PosNeg) -> bool {
		match (self, other) {
			(&PosNeg::Pos, &PosNeg::Pos) => true,
			(&PosNeg::Neg, &PosNeg::Neg) => true,
			_ => false
		}
	}
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
#[derive(Debug, Eq)]
pub struct PartialTime<'a> {
    pub hour: &'a str,
	pub minute: &'a str,
	pub second: &'a str,
	pub fraction: &'a str,
}

impl<'a> PartialEq for PartialTime<'a> {
	fn eq(&self, other: &PartialTime<'a>) -> bool {
		self.hour == other.hour &&
		self.minute == other.minute &&
		self.second == other.second &&
		self.fraction == other.fraction
	}
}

// (+|-)<hour>:<minute>
#[derive(Debug, Eq)]
pub struct TimeOffsetAmount<'a> {
	pub pos_neg: PosNeg,
	pub hour: &'a str,
	pub minute: &'a str,
}

impl<'a> PartialEq for TimeOffsetAmount<'a> {
	fn eq(&self, other: &TimeOffsetAmount<'a>) -> bool {
		self.pos_neg == other.pos_neg &&
		self.hour == other.hour &&
		self.minute == other.minute
	}
}

// <partial_time><time_offset>
#[derive(Debug, Eq)]
pub struct FullTime<'a> {
    pub partial_time: PartialTime<'a>,
    pub time_offset: TimeOffset<'a>,
}

impl<'a> PartialEq for FullTime<'a> {
	fn eq(&self, other: &FullTime<'a>) -> bool {
		self.partial_time == other.partial_time &&
		self.time_offset == other.time_offset
	}
}

// <year>-<month>-<day>
#[derive(Debug, Eq)]
pub struct FullDate<'a> {
	pub year: &'a str,
	pub month: &'a str,
	pub day: &'a str,
}

impl<'a> PartialEq for FullDate<'a> {
	fn eq(&self, other: &FullDate<'a>) -> bool {
		self.year == other.year &&
		self.month == other.month &&
		self.day == other.day
	}
}

// <date>T<time>
#[derive(Debug, Eq)]
pub struct DateTime<'a> {
	pub date: FullDate<'a>,
	pub time: FullTime<'a>,
}

impl<'a> PartialEq for DateTime<'a> {
	fn eq(&self, other: &DateTime<'a>) -> bool {
		self.date == other.date &&
		self.time == other.time
	}
}
