mod grep;
mod positive;

use std::io;

use clap::{value_t_or_exit, App, Arg};

use self::grep::Grep;
use self::positive::Positive;

fn main() {
  let matches = App::new("fastgrep")
    .version("0.2.2")
    .about("Grep, but FAST! Uses multi-threading to grep very large files")
    .arg(
      Arg::with_name("NEEDLE")
        .help("Substring to grep for within the given input")
        .required(true),
    )
    .arg(
      Arg::with_name("jobs")
        .short("j")
        .long("jobs")
        .value_name("N")
        .help("Sets number of jobs to run in parallel")
        .required(true),
    )
    .arg(
      Arg::with_name("size")
        .short("s")
        .long("size")
        .value_name("N")
        .help("Sets number of lines to include in each job")
        .required(true),
    )
    .get_matches();

  let threads = value_t_or_exit!(matches, "jobs", Positive).into();
  let size = value_t_or_exit!(matches, "size", Positive).into();

  let needle = matches.value_of("NEEDLE").unwrap().to_owned();

  let stdin = io::stdin();
  let stdout = io::stdout();

  let grep = Grep::new(stdin.lock(), needle);
  grep.execute(stdout, threads, size);
}
