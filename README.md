## *TOML_PARSER is now TOMLLIB and TOMLKIT!*

## Tomllib is a parser, modifier, and generator for TOML files ***that doesn't judge you***! 

######It is written in Rust using [nom](https://github.com/Geal/nom). Tomlkit is the command line tool that has the same functionality as the library  (but doesn't actually exist yet, but then again neither does the library).

[![ghit.me](https://ghit.me/badge.svg?repo=joelself/toml_parser)](https://ghit.me/repo/joelself/toml_parser)

####What does it mean that it doesn't judge me?###

Toml lib respects your crazy indentation and whitespace scheme. It respects the order you place things. It doesn't try to reformat the file based on *somebody else's* views on file format purity. It only makes changes that you tell it to make and leaves the rest alone. Want 20 tabs between every key and `=` in a key value pair? Have at it! Want to put a comment and 5 newlines after every array value? We won't try to change your mind! Randomly placed tables and nested tables? It's your file! Do whatever you want and as long as it is within spec; we won't try to change it.

###Tomllib###

Based on [my version](https://github.com/joelself/toml/blob/abnf/toml.abnf) of the official [TOML ABNF](https://github.com/toml-lang/toml/blob/abnf/toml.abnf#L54) (at least until they merge my changes). Currently can parse entire Unicode TOML files and reconstruct them into a perfect copy, preserving order and all whitespace. Tested with perfect output on the toml README example, the regular and hard example in the toml test directory and [my own Unicode version of the hard example](https://github.com/joelself/toml/blob/master/tests/hard_example_unicode.toml).

Next steps for the first release are:
* Might back off to use beta Rust or, less likely, stable Rust
* Unit tests for all parsers that don't have them yet
* A test method that will iterate through each toml file in the assets directory, parse the file, then reconstruct it and compare it to the original.
* Content validation (currently the parser doesn't complain about heterogeneous arrays or duplicate keys, because it wants to give you a chance to correct them rather than immediately fail and force you to fix it by hand.)
* Value look-up/modification
* Key modification
* Element addition and deletion
* Type conversion

The difference between this toml library and others is that I'll preserve the exact formatting at all times unless told not to.

Some other things I might add:
* Strip extraneous spaces and newlines
* User defined re-indentation
* Element re-ordering
* For Cargo.toml files, updating dependency versions to the latest version on crates.io
* For Cargo.toml files, change ranges on dependency versions
* Conversion to JSON and YAML

To get the source simply ```git clone https://github.com/joelself/toml_parser.git```.
I took a dependency on `regex_macros` which requires you be on the nightly version of Rust. Fortunately [multirust](https://github.com/brson/multirust) makes this dead simple without forcing all of your Rust evironment to be on Nightly.

Install multirust (you'll have to [uninstall currently installed versions of Rust](https://doc.rust-lang.org/book/installing-rust.html#uninstalling)) first:

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
