use std::collections::HashSet;
use std::fmt::{Display, Formatter};
use std::io::{Read, stdin};
use std::path::{Path, PathBuf};

// TODO: Support counting by multibyte characters
//  This may require a lot of reworking, and may result in no longer using a HashSet but rather
//  using a struct of which count modes are on/off, or for chars, how they should be counted.
#[derive(Debug)]
pub struct WordCounter {
    modes: HashSet<CountMode>,
    source: StreamSource,
}

impl Default for WordCounter {
    fn default() -> Self {
        Self {
            modes: HashSet::from([CountMode::Line, CountMode::Word, CountMode::Character]),
            source: StreamSource::Stdin,
        }
    }
}

/// Getters
impl WordCounter {
    pub fn modes(&self) -> &HashSet<CountMode> {
        &self.modes
    }
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
            .map_err(|e| WordCountError::Io(None, e))?;
        let wc = Count::new_bytes(None, &bytes);

        Ok(wc)
    }

    fn count_in_file(&self, path: &Path) -> Result<Count, WordCountError> {
        let bytes =
            std::fs::read(path).map_err(|e| WordCountError::Io(Some(path.to_owned()), e))?;

        let wc = Count::new_bytes(Some(path.into()), &bytes);

        Ok(wc)
    }
}

pub struct Count {
    file_name: Option<PathBuf>,
    char_count: usize,
    word_count: usize,
    line_count: usize,
}

/// Getters
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

/// Constructors
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

    fn new_total(counts: &[Result<Count, WordCountError>]) -> Self {
        let mut total = Self::new(Some("total".into()));
        for maybe_count in counts {
            let Ok(count) = maybe_count else { continue };
            total.char_count += count.char_count;
            total.word_count += count.word_count;
            total.line_count += count.line_count;
        }

        total
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
    Io(Option<PathBuf>, std::io::Error),
    IllegalOption(char),
    Unknown,
}

impl Display for WordCountError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            WordCountError::Io(file_name, e) => write!(
                f,
                "{}: {}",
                file_name
                    .as_ref()
                    .map(|p| p.display().to_string())
                    .unwrap_or("stdin".to_string()),
                e.kind()
            ),
            WordCountError::IllegalOption(char) => write!(f, "illegal option -- {}", char),
            WordCountError::Unknown => write!(f, "unknown error"),
        }?;

        Ok(())
    }
}
