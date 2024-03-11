# dupsrm

Command line tool to remove duplicated files.
It recurses a reference and a root directory, finds file duplicates from the reference directory tree in the root directory tree and removes them.

## TODOs

- [ ] Add command line interface to define reference and root paths
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
