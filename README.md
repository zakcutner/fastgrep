## fastgrep

**:rocket: Grep, but _FAST_! Uses multi-threading to grep very large files**


### Installation

1. Run `cargo build --release` to build the binary.
2. Execute binary at `./target/release/fastgrep`.
3. Add to your `PATH` by running `cp target/release/fastgrep /usr/local/bin/`.


### Usage

```
fastgrep 0.2.3
Grep, but FAST! Uses multi-threading to grep very large files

USAGE:
    fastgrep <NEEDLE> --jobs <N> --size <N>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -j, --jobs <N>    Sets number of jobs to run in parallel
    -s, --size <N>    Sets number of lines to include in each job

ARGS:
    <NEEDLE>    Substring to grep for within the given input
```


### Appendix

- Requires Rust to be installed, can be obtained from [here](https://www.rust-lang.org/).
- Greater job sizes will result in more lines being loaded into memory at once.
- Please open an issue for any questions!
