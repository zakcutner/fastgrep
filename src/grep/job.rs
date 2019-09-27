use std::io;
use std::sync::Once;

use super::ArcPattern;

pub struct Job {
    once: Once,
    chunk: Vec<io::Result<String>>,
    pattern: ArcPattern,
    colour: bool,
    result: Vec<String>,
}

impl Job {
    pub fn new(chunk: Vec<io::Result<String>>, pattern: ArcPattern, colour: bool) -> Self {
        Self {
            once: Once::new(),
            chunk,
            pattern,
            colour,
            result: Vec::new(),
        }
    }

    pub fn execute(&mut self) {
        let chunk = &self.chunk;
        let pattern = &*self.pattern;
        let colour = &self.colour;
        let result = &mut self.result;

        self.once.call_once(move || {
            for line in chunk {
                let line = line.as_ref().unwrap();

                let matches = pattern.find(line);

                if matches.is_empty() {
                    continue;
                }

                if !colour {
                    result.push(line.clone());
                    continue;
                }

                let mut output = String::with_capacity(line.len());
                let mut last_end = 0;

                for (start, end) in matches {
                    output.push_str(unsafe { line.get_unchecked(last_end..start) });
                    push_coloured(&mut output, unsafe { line.get_unchecked(start..end) });
                    last_end = end;
                }

                output.push_str(unsafe { line.get_unchecked(last_end..line.len()) });
                result.push(output);
            }
        });
    }

    pub fn result(&self) -> &Vec<String> {
        &self.result
    }
}

fn push_coloured(dest: &mut String, src: &str) {
    dest.push_str("\x1b[91m");
    dest.push_str(src);
    dest.push_str("\x1b[0m");
}
