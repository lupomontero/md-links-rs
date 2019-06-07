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

### CLI

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

### Rust

The Rust library exposes one function:

`md_links::from_path(path: &PathBuf, validate: bool) -> Vec<Link>`

Example:

```rust
extern crate md_links;

let path = PathBuf::from("./some/dir");
let links = md_links::from_path(&path, false);

for link in links {
  println!("{:?}", link);
}
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

***

## TODO

* Learn Rust 🦀
* Error handling :see_no_evil:
* Mock http requests and filesystem in tests?
* Replace `println!` with a _Writer_ and stream output instead of buffering to a
  string and printing at the end.
* [Clippy](https://github.com/rust-lang/rust-clippy)?
* Code coverage?
* Add Travis CI build
* Run requests in multiple threads?
* Request concurrency and async?
* Reorganize code in more Rust-like manner
* Persistent cache (file based)?
* Progress bar?
* Validate _relative_ and _fragment_ links?
