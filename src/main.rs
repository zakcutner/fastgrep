mod grep;

use std::env;
use std::io;
use std::process;

use self::grep::Grep;

const DEFAULT_THREADS: usize = 2;

struct Args {
  threads: usize,
  needle: String,
}

fn main() {
  let mut args = env::args();

  // First argument is program name, should be discarded.
  args.next();

  let args = parse_args(&mut args).unwrap_or_else(|| {
    eprintln!("Usage: fastgrep [-t threads] needle");
    process::exit(1);
  });

  if args.threads < 2 {
    eprintln!("Must have at least one worker thread!");
    process::exit(1);
  }

  let stdin = io::stdin();

  let grep = Grep::new(stdin.lock(), args.needle);
  grep.execute(args.threads - 1);
}

fn parse_args(args: &mut env::Args) -> Option<Args> {
  match args.next()?.as_ref() {
    "-t" => Some(Args {
      threads: match args.next()?.parse() {
        Ok(threads) => threads,
        Err(_) => return None,
      },
      needle: parse_args(args)?.needle,
    }),
    arg => Some(Args {
      threads: DEFAULT_THREADS,
      needle: arg.to_owned(),
    }),
  }
}
