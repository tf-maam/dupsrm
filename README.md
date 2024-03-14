# dupsrm

[![Rust](https://github.com/tf-maam/dupsrm/actions/workflows/rust.yml/badge.svg)](https://github.com/tf-maam/dupsrm/actions/workflows/rust.yml)

Command line tool to remove duplicated files.
It recurses a reference and a root directory, finds file duplicates from the reference directory tree in the root directory tree and removes them.

## Usage

```text
Remove duplicated files in the reference directory that are found in the root directory tree

Usage: dupsrm [OPTIONS] <REFERENCE_DIR> <ROOT_DIR>

Arguments:
  <REFERENCE_DIR>  Reference directory path
  <ROOT_DIR>       Root directory path

Options:
  -d, --dry-run  Perform a dry-run without removing any file
  -h, --help     Print help
  -V, --version  Print version
```

## Installation

```bash
cargo build --release
cargo install --path .
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
- [ ] Add a filter for file types or regex support
- [ ] Use `PathBuf` instead of `String`
- [ ] Wrap hash type with `&str`
