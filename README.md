# dupsrm

Command line tool to remove duplicated files.
It recurses a reference and a root directory, finds file duplicates from the reference directory tree in the root directory tree and removes them.

## TODOs

- [x] Recursively iterate root and reference directories
- [x] Calculate the hash of each file and store them in a list aside from the path
- [x] Create a list of duplicates in the reference directory
- [ ] Add command line interface to define reference and root paths \
    See the Rust [CLI book](https://rust-cli.github.io/book/index.html) for further details.
    Use [clap](https://docs.rs/clap/latest/clap/) for command line argument parsing
- [ ] Add the method to remove files
- [ ] Add the command line flags `-n, --dry-run` to don't remove files as in `git rm`
- [ ] Modularize source code into different files
- [ ] Add additional unit tests with an example file structure
- [ ] Create a docker container for running build tests
- [ ] Create Github and Gitlab CI
- [ ] Modularize the hash function to allow the usage of other hash algorithms
- [ ] Benchmark implementation
- [ ] Parallelize iterators and hashing of files in multiple threads
- [ ] Write documentation with usage examples
- [ ] Extend logger output
- [ ] Use a hashmap to find duplicated hashes decreasing the computational complexity
- [ ] Add a filter for file types or regex support
