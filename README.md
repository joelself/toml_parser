# RUST TOML PARSER

###A TOML file parser and manipulator in Rust using [nom](https://github.com/Geal/nom)

[![ghit.me](https://ghit.me/badge.svg?repo=joelself/toml_parser)](https://ghit.me/repo/joelself/toml_parser)

Based on the official [TOML ABNF](https://github.com/toml-lang/toml/blob/abnf/toml.abnf#L54). Currently a work in progress as it can only parse Newlines, Whitespace, Comments, Key-Value Pairs, Standard Tables, Array Tables, Integers, Floats, Basic Strings, Multiline Basic Strings, Literal Strings, Booleans, Datetimes, and Arrays. Still needs Inline tables, Expressions, allow Inline tables to be vals and the top-level TOML.

All high level structs representing the ABNF are implemented except for (partially) Val, Expression, Inline table and TOML.

To get the source simply ```git clone https://github.com/joelself/toml_parser.git```.
I took a dependencies on `regex_macros` which requires you be on the nightly version of Rust. Fortunately [multirust](https://github.com/brson/multirust) makes this dead simple without forcing all of your Rust evironment to be on Nightly.

Install multirust (you'll have to [uninstall currently installed versions of Rust](https://doc.rust-lang.org/book/installing-rust.html#uninstalling) first:

```shell
curl -sf https://raw.githubusercontent.com/brson/multirust/master/blastoff.sh | sh
```
Change into the toml_parser directory and set that directory (and that directory only) to use Nightly Rust:

```shell
cd toml_parser
multirust override nightly
```

You can always go back to stable or beta with ```multirust override (beta|stable)```.
To make changes fork the repository, then clone it, make changes, and issue a pull request. If you have any problems enter an issue in the issue tracker.

**I would love to hear your feedback. If there's something you would like this project to do then feel free to write up an issue about it.** If you're not comfortable writing an issue out in the open you can email me at <joel@dislocal.com>.
