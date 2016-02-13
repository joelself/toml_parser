use ::types::Str;

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