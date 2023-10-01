use std::io::{BufRead, BufReader, Lines, Read};

use fancy_regex::Regex;

pub(crate) struct FilterIter<Reader> {
    lines: Lines<BufReader<Reader>>,
    includes: Vec<Regex>,
    excludes: Vec<Regex>,
}

impl<Reader> FilterIter<Reader>
where
    Reader: Read,
{
    pub fn new(reader: Reader, included_lines: Vec<Regex>, excluded_lines: Vec<Regex>) -> Self {
        let buffer = std::io::BufReader::new(reader);
        let lines = buffer.lines();
        Self {
            lines,
            includes: included_lines,
            excludes: excluded_lines,
        }
    }
}

impl<Reader> Iterator for FilterIter<Reader>
where
    Reader: Read,
{
    type Item = std::io::Result<String>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let next_line = self.lines.next();

            if let Some(Ok(line)) = next_line {
                if !is_included(&line, &self.includes) {
                    continue;
                }
                if is_ignored(&line, &self.excludes) {
                    continue;
                }
                return Some(Ok(line));
            } else {
                return next_line;
            }
        }
    }
}

fn is_ignored(line: &str, ignores: &Vec<Regex>) -> bool {
    for rgx in ignores {
        if let Ok(Some(_)) = rgx.captures(line) {
            return true;
        }
    }
    false
}

fn is_included(line: &str, includes: &Vec<Regex>) -> bool {
    for rgx in includes {
        if let Ok(Some(_)) = rgx.captures(line) {
            return true;
        }
    }
    includes.is_empty()
}
