# RUST TOML PARSER

###A TOML file parser and manipulator in Rust using [nom](https://github.com/Geal/nom)

[![ghit.me](https://ghit.me/badge.svg?repo=joelself/toml_parser)](https://ghit.me/repo/joelself/toml_parser)

Based on the official [TOML ABNF](https://github.com/toml-lang/toml/blob/abnf/toml.abnf#L54). Currently a work in progress as it can only parse Newlines, Whitespace, Comments, (partially) Key-Value Pairs, Standard Tables, Array Tables, Integers, Floats, Basic Strings, and Multiline Basic Strings. Still needs the rest of Key-Value Pairs, Literal Strings, Booleans, Datetimes, Array, Inline Tables, Expressions, and TOML (the top level).

Needs structs to represent various high-level pieces, probably Arrays, Inline Tables, Expressions, and TOML as well as accept methods for traversing the structure. Also needs tests for each named function.

To get the source simply ```git clone https://github.com/joelself/toml_parser.git```. To make changes fork the repository, then clone it and make changes.
