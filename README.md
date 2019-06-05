# md-links-rs

Rust implementation of [Laboratoria's bootcamp project `md-links`](https://github.com/Laboratoria/curricula-js/tree/master/projects/04-md-links).

:construction: WIP

## Installation

```sh
git clone git@github.com:lupomontero/md-links-rs.git
cd md-links-rs
cargo install --path .
```

## Usage

```sh
$ md-links-rs --help
md-links 0.1.0
Lupo Montero <lupomontero@gmail.com>
Check links in MarkDown files.

USAGE:
    md-links-rs [FLAGS] <path>

FLAGS:
    -h, --help        Prints help information
    -j, --json        Show output in JSON format
    -s, --stats       Show stats instead of individual matches
    -v, --validate    Validate links (send HTTP requests)
    -V, --version     Prints version information

ARGS:
    <path>    The path to the file to read
```

## Build

```sh
# Dev build...
cargo build

# Release build...
cargo build --release
```

## Tests

```sh
cargo test
```
