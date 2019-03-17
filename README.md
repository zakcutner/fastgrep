## fastgrep

**:rocket: Grep, but _FAST_! Uses multi-threading to grep very large files**


### Installation

1. Run `cargo build --release` to build the binary.
2. Execute binary at `./target/release/fastgrep`.
3. Add to your `PATH` by running `cp target/release/fastgrep /usr/local/bin/`.


### Usage

```
fastgrep 0.3.0
Zak Cutner (https://zakcutner.uk)
Grep, but FAST! Uses multi-threading to grep very large files

USAGE:
    fastgrep [FLAGS] [OPTIONS] <PATTERN> --jobs <NUMBER> --size <NUMBER>

FLAGS:
    -h, --help       Prints help information
    -r, --regex      Enables regular expression patterns
    -V, --version    Prints version information

OPTIONS:
    -c, --colour=<WHEN>    Sets when the output should be coloured, can be `auto', `always' or `never'
    -j, --jobs <NUMBER>    Sets number of jobs to run in parallel, must be at least one
    -s, --size <NUMBER>    Sets number of lines to include in each job, must be at least one

ARGS:
    <PATTERN>    Pattern to grep for within the given input
```


### Appendix

- Requires Rust to be installed, can be obtained from [here](https://www.rust-lang.org/).
- Greater job sizes will result in more lines being loaded into memory at once.
- Please open an issue for any questions!
