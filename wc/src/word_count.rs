use std::collections::{HashMap, HashSet};
use std::fmt::Display;
use std::io::{ErrorKind, Read, stdin};
use std::path::{Path, PathBuf};

pub struct Count {
    file_name: Option<PathBuf>,
    char_count: usize,
    word_count: usize,
    line_count: usize,
}

impl Count {
    pub fn file_name(&self) -> Option<&Path> {
        self.file_name.as_ref().map(|p| p.as_path())
    }

    pub fn char_count(&self) -> usize {
        self.char_count
    }

    pub fn word_count(&self) -> usize {
        self.word_count
    }

    pub fn line_count(&self) -> usize {
        self.line_count
    }
}

impl Count {
    fn new(name: Option<PathBuf>) -> Self {
        Self {
            file_name: name,
            char_count: 0,
            word_count: 0,
            line_count: 0,
        }
    }

    fn new_bytes(name: Option<PathBuf>, bytes: &[u8]) -> Self {
        let mut counts = Self::new(name);
        for b in bytes {
            counts.char_count += 1;
            if *b == b'\n' {
                counts.word_count += 1;
                counts.line_count += 1;
            } else if b.is_ascii_whitespace() {
                counts.word_count += 1;
            }
        }

        counts
    }

    fn new_total(
        counts: &[Result<Count, WordCountError>],
    ) -> Self {
        let mut total = Self::new(Some("total".into()));
        for maybe_count in counts {
            let Ok(count) = maybe_count else { continue };
            total.char_count += count.char_count;
            total.word_count += count.word_count;
            total.line_count += count.line_count;
        }

        total
    }

    fn count_bytes(mut self, bytes: &[u8]) -> Self {
        for b in bytes {
            self.char_count += 1;
            if *b == b'\n' {
                self.word_count += 1;
                self.line_count += 1;
            } else if b.is_ascii_whitespace() {
                self.word_count += 1;
            }
        }

        self
    }
}

#[derive(Debug)]
pub struct WordCounter {
    modes: HashSet<CountMode>,
    source: StreamSource,
}

impl WordCounter {
    pub fn modes(&self) -> &HashSet<CountMode> {
        &self.modes
    }
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

    pub fn count(&self) -> Vec<Result<Count, WordCountError>> {
        match &self.source {
            StreamSource::Stdin => {
                vec![self.count_in_stdin()]
            }

            StreamSource::Files(files) => {
                let mut output: Vec<Result<Count, WordCountError>> =
                    Vec::with_capacity(files.len() + 1);
                for file in files {
                    output.push(self.count_in_file(file));
                }

                if files.len() > 1 {
                    // Append a total
                    output.push(Ok(Count::new_total(&output)));
                }

                output
            }
        }
    }

    fn count_in_stdin(&self) -> Result<Count, WordCountError> {
        let mut bytes = Vec::new();
        stdin()
            .lock()
            .read_to_end(&mut bytes)
            .map_err(|_e| WordCountError::Unknown)?;
        let wc = Count::new(None).count_bytes(&bytes);

        Ok(wc)
    }

    fn count_in_file(&self, path: &Path) -> Result<Count, WordCountError> {
        let bytes = std::fs::read(path).map_err(|e| {
            if e.kind() == ErrorKind::NotFound {
                WordCountError::FileNotFound(path.to_owned())
            } else {
                WordCountError::Unknown
            }
        })?;

        let wc = Count::new(Some(path.into())).count_bytes(&bytes);

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
