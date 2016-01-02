# RUST TOML PARSER

###A TOML file parser and manipulator in Rust using [nom](https://github.com/Geal/nom)

[![ghit.me](https://ghit.me/badge.svg?repo=joelself/toml_parser)](https://ghit.me/repo/joelself/toml_parser)

Based on the [my version](https://github.com/joelself/toml/blob/abnf/toml.abnf) of the official [TOML ABNF](https://github.com/toml-lang/toml/blob/abnf/toml.abnf#L54) (at least until they merge my changes). Currently can parse entire TOML files and reconstruct them into a perfect copy, preserving order and all whitespace. Tested with perfect output on the toml README example, and the regular and hard example in the toml test directory.

Next steps are for unit tests for all parsers that don't have them yet. A test method that will iterate through each toml file in the assets directory, parse the file, then reconstruct it and compare it to the original.

After testing shows the parser to be working I'll add functionality found in other toml packages such as
* Value look-up/modification
* Adding and removing elements
* Type conversion

The difference will be that I'll preserve the exact formatting at all times unless told not to.

Some other things I might add:
* Strip extraneous spaces and newlines
* Reorder elements
* For Cargo.toml files, updating dependency versions to the latest on crates.io
* For Cargo.toml files, change ranges on dependency versions
* Conversion to JSON and YAML

To get the source simply ```git clone https://github.com/joelself/toml_parser.git```.
I took a dependency on `regex_macros` which requires you be on the nightly version of Rust. Fortunately [multirust](https://github.com/brson/multirust) makes this dead simple without forcing all of your Rust evironment to be on Nightly.

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
