# dupsrm

[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)
[![Rust](https://github.com/tf-maam/dupsrm/actions/workflows/rust.yml/badge.svg)](https://github.com/tf-maam/dupsrm/actions/workflows/rust.yml)
![GitHub Release](https://img.shields.io/github/v/release/tf-maam/dupsrm)

Command line tool to remove duplicated files.
It recurses a reference and a root directory, finds file duplicates from the reference directory tree in the root directory tree and removes them.

`dupsrm`: **dup**licate**s** **r**e**m**oval

## Usage

```text
Remove duplicated files in the reference directory that are found in the root directory tree

Usage: dupsrm [OPTIONS] <REFERENCE_DIR> <ROOT_DIR>

Arguments:
  <REFERENCE_DIR>  Reference directory path
  <ROOT_DIR>       Root directory path

Options:
  -d, --dry-run        Perform a dry-run without removing any file
  -r, --regex <REGEX>  Regular expression filtering files in reference directories
  -h, --help           Print help
  -V, --version        Print version

```

## Installation

```bash
cargo build --release
cargo install --path .
```

## Profiling

```bash
rm default_* 
rm dupsrm.profdata
cargo clean
# profile execution
RUSTFLAGS="-C instrument-coverage" cargo build
target/debug/dupsrm test/ .
# or profile tests
RUSTFLAGS="-C instrument-coverage" cargo test --tests

llvm-profdata merge -sparse default_*.profraw -o dupsrm.profdata
llvm-cov report --use-color --ignore-filename-regex='/.cargo/registry' --instr-profile=dupsrm.profdata --object target/debug/dupsrm
llvm-cov show --use-color --ignore-filename-regex='/.cargo/registry' --instr-profile=dupsrm.profdata --object target/debug/dupsrm
```

## TODOs

- [x] Recursively iterate root and reference directories
- [x] Calculate the hash of each file and store them in a list aside from the path
- [x] Create a list of duplicates in the reference directory
- [x] Add command line interface to define reference and root paths \
    See the Rust [CLI book](https://rust-cli.github.io/book/index.html) for further details.
    Use [clap](https://docs.rs/clap/latest/clap/) for command line argument parsing
- [x] Add the method to remove files
- [x] Add the command line flags `-n, --dry-run` to don't remove files as in `git rm`
- [ ] Modularize source code into different files
- [x] Add additional unit tests with an example file structure
- [ ] Create a docker container for running build tests
- [x] Create Github and Gitlab CI
- [ ] Modularize the hash function to allow the usage of other hash algorithms
- [ ] Benchmark implementation
- [x] Parallelize iterators and hashing of files in multiple threads
- [ ] Write documentation with usage examples
- [ ] Extend logger output
- [ ] Use a hashmap to find duplicated hashes decreasing the computational complexity
- [x] Add a filter for file types or regex support
- [ ] Use `PathBuf` instead of `String` for paths
- [ ] Wrap hash type with `&str` or fixed size type
- [ ] Add a flag to not recurse the reference directory or set a maximum depth
- [ ] Provide usage examples with regular expression
