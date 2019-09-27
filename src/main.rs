mod grep;
mod positive;

use std::io;

use clap::*;
use regex::Regex;

use self::grep::Grep;
use self::positive::Positive;

fn main() {
    let matches = app_from_crate!()
        .arg(
            Arg::with_name("regex")
                .short("r")
                .long("regex")
                .help("Enables regular expression patterns"),
        )
        .arg(
            Arg::with_name("jobs")
                .short("j")
                .long("jobs")
                .takes_value(true)
                .value_name("NUMBER")
                .help("Sets number of jobs to run in parallel, must be at least one")
                .required(true),
        )
        .arg(
            Arg::with_name("size")
                .short("s")
                .long("size")
                .takes_value(true)
                .value_name("NUMBER")
                .help("Sets number of lines to include in each job, must be at least one")
                .required(true),
        )
        .arg(
            Arg::with_name("colour")
                .short("c")
                .long("colour")
                .takes_value(true)
                .require_equals(true)
                .value_name("WHEN")
                .possible_values(&["auto", "always", "never"])
                .hide_possible_values(true)
                .help(
                    "Sets when the output should be coloured, can be `auto', `always' or `never'",
                ),
        )
        .arg(
            Arg::with_name("PATTERN")
                .help("Pattern to grep for within the given input")
                .required(true),
        )
        .setting(AppSettings::ArgRequiredElseHelp)
        .get_matches();

    let stdin = io::stdin();
    let stdout = io::stdout();

    let mut grep = Grep::new(stdin.lock());

    if matches.is_present("regex") {
        let pattern = value_t_or_exit!(matches, "PATTERN", Regex);
        grep.set_pattern(pattern);
    } else {
        let pattern = value_t_or_exit!(matches, "PATTERN", String);
        grep.set_pattern(pattern);
    }

    let threads = value_t_or_exit!(matches, "jobs", Positive).into();
    grep.set_threads(threads);

    let size = value_t_or_exit!(matches, "size", Positive).into();
    grep.set_size(size);

    let colour = matches.value_of("colour").unwrap_or("auto");
    match colour {
        "auto" => grep.set_colour(atty::is(atty::Stream::Stdout)),
        "always" => grep.set_colour(true),
        "never" => grep.set_colour(false),
        _ => panic!("unexpected colour value"),
    }

    grep.execute(stdout);
}
