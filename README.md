## fastgrep

**🚀 Grep, but _FAST_! Uses multi-threading to grep very large files**


### Installation

1. Run `cargo build --release` to build the binary.
2. Execute binary at `./target/release/fastgrep`.
3. Add to your `PATH` by running `cp target/release/fastgrep /usr/local/bin/`.


### Usage

```
fastgrep 0.1.0
Grep, but FAST! Uses multi-threading to grep very large files

USAGE:
    fastgrep <NEEDLE> --jobs <N>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -j, --jobs <N>    Number of jobs to run in parallel

ARGS:
    <NEEDLE>    Substring to grep for within the given input
```


### Appendix

- Requires Rust to be installed, can be obtained from [here](https://www.rust-lang.org/).
- Please open an issue for any questions!
