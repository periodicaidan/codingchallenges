mod word_count;

use crate::word_count::{CountMode, Count, WordCountError};
use std::collections::HashSet;
use std::env::args;
use std::path::PathBuf;
use word_count::WordCounter;

fn format_line(line: &Result<Count, WordCountError>, modes: &HashSet<CountMode>) {
    match line {
        Err(e) => {
            if let WordCountError::FileNotFound(file) = e {
                println!(
                    "wc: {}: open: No such file or directory",
                    file.to_string_lossy()
                )
            } else {
                unreachable!()
            }
        }

        Ok(word_count) => {
            if modes.contains(&CountMode::Line) {
                print!("{:>8}", word_count.line_count());
            }

            if modes.contains(&CountMode::Word) {
                print!("{:>8}", word_count.word_count());
            }

            if modes.contains(&CountMode::Character) {
                print!("{:>8}", word_count.char_count());
            }

            if let Some(file_name) = word_count.file_name() {
                print!(" {}", file_name.to_string_lossy());
            }

            println!();
        }
    }
}

fn main() -> Result<(), WordCountError> {
    //           the first arg is the exe name
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

    wc.count().iter().for_each(|counts| format_line(counts, wc.modes()));


    Ok(())
}
