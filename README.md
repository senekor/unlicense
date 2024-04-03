# unlicense

Access the text of the Unlicense. <https://unlicense.org>

## Installation

Using [`cargo-binstall`](https://github.com/cargo-bins/cargo-binstall):

```sh
cargo binstall unlicense
```

Using `cargo`:
  
```sh
cargo install unlicense
```

You can also grab a binary from the [releases page](https://github.com/senekor/unlicense/releases).

## Usage

```
$ unlicense # write to ./UNLICENSE
$ unlicense | cat # print to stdout

$ unlicense --help
Access the text of the Unlicense. <https://unlicense.org>

When used interactively, writes to ./UNLICENSE.
Otherwise (e.g. in a pipe), writes to stdout.

When given any arguments, displays this help page.
```

## Library

This project also exposes a library you can use in your own Rust code.
Add it as a dependency with:

```sh
cargo add unlicense
```
