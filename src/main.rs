mod grep;

use std::io;
use std::process;

use clap::{value_t_or_exit, App, Arg};

use self::grep::Grep;

fn main() {
  let matches = App::new("fastgrep")
    .version("0.1.0")
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
        .help("Number of jobs to run in parallel")
        .required(true),
    )
    .get_matches();

  let threads = value_t_or_exit!(matches, "jobs", usize);

  if threads < 1 {
    eprintln!("Must have at least one worker thread!");
    process::exit(1);
  }

  let stdin = io::stdin();

  let grep = Grep::new(stdin.lock(), matches.value_of("NEEDLE").unwrap().to_owned());
  grep.execute(threads);
}
