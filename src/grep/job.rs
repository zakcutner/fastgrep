use std::io;
use std::sync::{Arc, Once};

pub struct Job {
  once: Once,
  chunk: Vec<io::Result<String>>,
  needle: Arc<String>,
  result: Vec<String>,
}

impl Job {
  pub fn new(chunk: Vec<io::Result<String>>, needle: Arc<String>) -> Self {
    Job {
      once: Once::new(),
      chunk,
      needle,
      result: Vec::new(),
    }
  }

  pub fn execute(&mut self) {
    let chunk = &self.chunk;
    let needle = &*self.needle;
    let result = &mut self.result;

    self.once.call_once(move || {
      for line in chunk {
        let line = line.as_ref().unwrap();

        let matches = line.match_indices(needle);
        let mut matches = matches.peekable();

        if matches.peek().is_none() {
          continue;
        }

        let mut output = String::new();
        let mut last_end = 0;

        for (start, _) in matches {
          output.push_str(unsafe { line.get_unchecked(last_end..start) });
          push_coloured(&mut output, needle);
          last_end = start + needle.len();
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
