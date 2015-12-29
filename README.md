# RUST TOML PARSER

###A TOML file parser and manipulator in Rust using [nom](https://github.com/Geal/nom)

Currently a work in progress as it can only parse comments, end-of-lines, and key_names.

To modify the source simply ```git clone https://github.com/joelself/toml_parser.git```.
TOML Parser uses the beta channel of Rust because it uses ```Vector.extend_from_slice```.
This is most easily managed using [multirust](https://github.com/brson/multirust).
Install multirust:

```shell
curl -sf https://raw.githubusercontent.com/brson/multirust/master/blastoff.sh | sh
```

Change to the ```toml_parser``` directory:

```shell
cd toml_parser
```

Then run:

```shell
multirust override beta
cargo build
```

This will set only the ```toml_parser``` directory to the Rust beta channel and build the binary. Everywhere else Rust will be on the default channel (stable).
