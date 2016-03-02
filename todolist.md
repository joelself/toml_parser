* Everywhere
	- [x] Switch back to using nom now that the new version has been released

* primitives.rs
	- [x] Implement get_keychain_key
	- [x] Implement get_full_key
	- [x] Implement get_key_parent
	- [x] Change Key::Str to just hold an &'a str again
	- [x] Fix DateTime to allow only Date, only DateTime (no fractional seconds), only DateTime (with fractional seconds), Full DateTime with offset
	- [x] Change TimeOffset::Z to TimeOffset::Zulu?
	- [x] Change '+'/'-' to enum
	- [x] Inserting a value insert's its key in it's parent's children 
	- [x] Re-implement get_array_table_key to take into account implicit tables are always standard tables

* ast/structs.rs
	- [x] Re-implement HashValue to have a list of children or max index of children
	- [x] Fix DateTime to allow only Date, only DateTime (no fractional seconds), only DateTime (with fractional seconds), Full DateTime with offset
	- [x] Change TimeOffset::Z to TimeOffset::Zulu?
	- [x] Change '+'/'-' to enum

* objects.rs
	- [x] In array_table when adding to existing table get_key_parent and add the new index as a child in the map, then add full_key to the map with None value
	- [x] Fix add_implicit_tables
	- [x] In array_table if table keys imply subtables that don't exist, add the implied tables as std_tables to the map with None value and add add their subkeys as children (partially done)
	- [x] In std_table if table keys imply subtables that don't exist, add the implied tables as std_tables to the map with None value and add add their subkeys as children
	- [x] In array_table if get_key_parent exists and has no indexed children, then it is an error (see toml-test/invalid/table_array_implicit)
	- [x] In array_table when encountering a new table that isn't a subtable of the last table, rebuild last_array_tables and last_array_tables_index by starting at the first subkey, looking up it's children and so-on, if the array_table already exists
	- [x] In array_table always add new table to map with None value
	- [x] In std_table always add new table to map with None value
	- [x] In array_value insert_key_val_into_map

* parser.rs
	- [x] Change Key::Str to just hold a Str
	- [ ] Implement reconstruct_inline_table
	- [ ] Need to rehash keys values when reconstituting tables and arrays when their keys or structure has changed and also change their parent's children
		- [ ] For this version when changing an array or inline table, compare the structure of the original and new structures, if they are the same then just change values in the hash table
		- [ ] If the structure of a new inline table or array is different than what it is replacing then traverse the hash table and remove the existing structure entirely, this will be easier to implement than trying to preserve the what structure we can
			- [ ] Add the inline table or array with new structure as a fresh table/array with default formatting
	- [ ] Implement get_errors
	- [x] Implement get_children
	- [ ] Add unit tests for getting values
	- [ ] Add unit tests for setting values
	- [ ] Add unit tests to check the map to make sure removed keys are gone

* tests/assets.rs
	- [ ] Add failure/error tests for invalid toml-test's
	- [ ] Add toml/examples/example-v0.4.0.toml to success tests

* tests/parser_tests.rs
	- [ ] Add integration tests for parser, like unit tests, but load a larger document -> validate, do a bunch of gets -> validate, do a bunch of sets -> validate, then do a bunch of gets
