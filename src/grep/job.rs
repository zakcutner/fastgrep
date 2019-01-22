use std::io;
use std::sync::{Arc, Once};

pub struct Job {
  once: Once,
  line: String,
  needle: Arc<String>,
  result: Option<String>,
}

impl Job {
  pub fn new(line: io::Result<String>, needle: Arc<String>) -> Self {
    Job {
      once: Once::new(),
      line: line.unwrap(),
      needle,
      result: None,
    }
  }

  pub fn execute(&mut self) {
    let line = &self.line;
    let needle = &*self.needle;
    let result = &mut self.result;

    self.once.call_once(move || {
      let matches = line.match_indices(needle);
      let mut matches = matches.peekable();

      if matches.peek().is_none() {
        return;
      }

      let mut output = String::new();
      let mut last_end = 0;

      for (start, _) in matches {
        output.push_str(unsafe { line.get_unchecked(last_end..start) });
        push_coloured(&mut output, needle);
        last_end = start + needle.len();
      }

      output.push_str(unsafe { line.get_unchecked(last_end..line.len()) });
      *result = Some(output);
    });
  }

  pub fn result(&self) -> &Option<String> {
    &self.result
  }
}

fn push_coloured(dest: &mut String, src: &str) {
  dest.push_str("\x1b[91m");
  dest.push_str(src);
  dest.push_str("\x1b[0m");
}
