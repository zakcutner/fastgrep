use regex::Regex;

pub type Match = (usize, usize);

pub trait Pattern {
    fn find(&self, haystack: &str) -> Vec<Match>;
}

impl Pattern for String {
    fn find(&self, haystack: &str) -> Vec<Match> {
        haystack
            .match_indices(self)
            .map(|(start, text)| (start, start + text.len()))
            .collect()
    }
}

impl Pattern for Regex {
    fn find(&self, haystack: &str) -> Vec<Match> {
        self.find_iter(haystack)
            .map(|m| (m.start(), m.end()))
            .collect()
    }
}
