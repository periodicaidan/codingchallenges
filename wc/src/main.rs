mod word_count;

use crate::word_count::{CountMode, WordCount, WordCountError};
use std::collections::HashSet;
use std::env::args;
use std::path::PathBuf;
use word_count::WordCounter;

fn format_line(line: &Result<WordCount, WordCountError>) -> String {
    match line {
        Err(e) => {
            if let WordCountError::FileNotFound(file) = e {
                format!(
                    "wc: {}: open: No such file or directory",
                    file.to_string_lossy()
                )
            } else {
                unreachable!()
            }
        }

        Ok(word_count) => {
            let mut s = String::new();
            if let Some(line_count) = word_count.counts().get(&CountMode::Line) {
                s.push_str(&format!("{line_count:>8}"));
            }

            if let Some(word_count) = word_count.counts().get(&CountMode::Word) {
                s.push_str(&format!("{word_count:>8}"));
            }

            if let Some(char_count) = word_count.counts().get(&CountMode::Character) {
                s.push_str(&format!("{char_count:>8}"));
            }

            if let Some(file_name) = word_count.file_name() {
                s.push_str(&format!(" {}", file_name.to_string_lossy()));
            }

            s.push('\n');

            s
        }
    }
}

fn main() -> Result<(), WordCountError> {
    //                  the first arg is the exe name
    let args = args().skip(1).collect::<Vec<_>>();
    let wc = if args.len() == 0 {
        WordCounter::default()
    } else {
        let mut files: Vec<PathBuf> = Vec::with_capacity(args.len());
        let mut modes: HashSet<CountMode> = HashSet::new();
        let mut reading_opts = true;

        for arg in args {
            if !arg.starts_with('-') {
                reading_opts = false;
            }

            if reading_opts {
                for char in arg.chars().skip(1) {
                    modes.insert(CountMode::from_char(char)?);
                }
            }

            files.push(arg.into());
        }

        if modes.is_empty() {
            modes = HashSet::from([CountMode::Line, CountMode::Word, CountMode::Character]);
        }

        WordCounter::new(&files, modes)
    };

    let output = wc
        .count()
        .iter()
        .map(|c| format_line(c))
        .collect::<String>();

    print!("{output}");

    Ok(())
}
