use std::collections::{HashMap, HashSet};
use std::fmt::Display;
use std::fs::File;
use std::io::{BufRead, BufReader, ErrorKind, Read, stdin};
use std::path::{Path, PathBuf};

pub struct WordCount {
    counts: HashMap<CountMode, usize>,
    file_name: Option<PathBuf>,
}

impl WordCount {
    fn new(name: Option<PathBuf>, modes: HashSet<CountMode>) -> Self {
        let mut counts: HashMap<CountMode, usize> = HashMap::new();
        for mode in modes {
            counts.insert(mode, 0);
        }

        Self {
            counts,
            file_name: name,
        }
    }

    fn new_total(
        word_counts: &[Result<WordCount, WordCountError>],
        modes: HashSet<CountMode>,
    ) -> Self {
        let mut total = Self::new(Some("total".into()), modes);
        for word_count in word_counts {
            let Ok(word_count) = word_count else { continue };
            for (mode, count) in total.counts.iter_mut() {
                *count += word_count.counts.get(mode).unwrap_or(&0);
            }
        }

        total
    }

    pub fn file_name(&self) -> Option<&Path> {
        self.file_name.as_ref().map(|p| p.as_path())
    }

    pub fn counts(&self) -> &HashMap<CountMode, usize> {
        &self.counts
    }

    fn count(mut self, bytes: &[u8]) -> Self {
        self.counts
            .entry(CountMode::Line)
            .and_modify(|count| *count = bytes.split(|c| *c == b'\n').count());

        self.counts
            .entry(CountMode::Word)
            .and_modify(|count| *count = bytes.split(|c| *c == b' ').count());

        self.counts
            .entry(CountMode::Character)
            .and_modify(|count| *count = bytes.len());

        self
    }
}

#[derive(Debug)]
pub struct WordCounter {
    modes: HashSet<CountMode>,
    source: StreamSource,
}

#[derive(Debug, Hash, PartialEq, Eq, Copy, Clone)]
pub enum CountMode {
    Character,
    Line,
    Word,
}

impl CountMode {
    pub fn from_char(c: char) -> Result<CountMode, WordCountError> {
        match c {
            'c' | 'm' => Ok(CountMode::Character),
            'l' => Ok(CountMode::Line),
            'w' => Ok(CountMode::Word),
            c => Err(WordCountError::IllegalOption(c)),
        }
    }
}

#[derive(Debug)]
pub enum StreamSource {
    Files(Vec<PathBuf>),
    Stdin,
}

#[derive(Debug)]
pub enum WordCountError {
    FileNotFound(PathBuf),
    IllegalOption(char),
    Unknown,
}

impl WordCounter {
    pub fn new(files: &[PathBuf], modes: HashSet<CountMode>) -> Self {
        Self {
            source: if files.is_empty() {
                StreamSource::Stdin
            } else {
                StreamSource::Files(files.to_owned())
            },

            modes,
        }
    }

    pub fn count(&self) -> Vec<Result<WordCount, WordCountError>> {
        match &self.source {
            StreamSource::Stdin => {
                vec![self.count_in_stdin()]
            }

            StreamSource::Files(files) => {
                let mut output: Vec<Result<WordCount, WordCountError>> =
                    Vec::with_capacity(files.len() + 1);
                for file in files {
                    output.push(self.count_in_file(file));
                }

                if files.len() > 1 {
                    // Append a total
                    output.push(Ok(WordCount::new_total(&output, self.modes.clone())));
                }

                output
            }
        }
    }

    fn init_count(&self, file_name: Option<PathBuf>) -> WordCount {
        WordCount::new(file_name, self.modes.clone())
    }

    fn count_in_stdin(&self) -> Result<WordCount, WordCountError> {
        let mut bytes = Vec::new();
        stdin()
            .lock()
            .read_to_end(&mut bytes)
            .map_err(|_e| WordCountError::Unknown)?;
        let wc = self.init_count(None).count(&bytes);

        Ok(wc)
    }

    fn count_in_file(&self, path: &Path) -> Result<WordCount, WordCountError> {
        let bytes = std::fs::read(path).map_err(|e| {
            if e.kind() == ErrorKind::NotFound {
                WordCountError::FileNotFound(path.to_owned())
            } else {
                WordCountError::Unknown
            }
        })?;

        let wc = self.init_count(Some(path.into())).count(&bytes);

        Ok(wc)
    }
}

impl Default for WordCounter {
    fn default() -> Self {
        Self {
            modes: HashSet::from([CountMode::Line, CountMode::Word, CountMode::Character]),
            source: StreamSource::Stdin,
        }
    }
}
