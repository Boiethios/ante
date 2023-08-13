//! error/mod.rs - Defines the error, warning, and note macros
//! used to issue compiler errors. There is also an ErrorMessage type
//! for storing messages that may be issued later. Note that all issuing
//! an error does is print it to stderr and update the global ERROR_COUNT.
//!
//! Compiler passes are expected to continue even after issuing errors so
//! that as many can be issued as possible. A possible future improvement
//! would be to implement poisoning so that repeated errors are hidden.
mod error;
pub mod location;
mod note;
mod styling;
mod warning;

pub use self::{error::CompilationError, note::CompilationNote, styling::Styling, warning::CompilationWarning};

use self::location::OwnedLocation;
use crate::{cache::ModuleCache, error::location::Location};
use owo_colors::OwoColorize as _;
use std::{fmt, path::Path};

/// Reads the given file, returning all of its contents
/// TODO: read from the cache instead:
fn read_file_or_panic(path: &Path) -> String {
    use std::fs::File;
    use std::io::{BufReader, Read};

    let file = File::open(path).unwrap();
    let mut reader = BufReader::new(file);
    let mut contents = String::new();
    reader.read_to_string(&mut contents).unwrap();
    contents
}

#[derive(Debug)]
pub struct CompilationMessage {
    message: MessageType,
    location: OwnedLocation,
}

#[derive(Debug)]
pub enum MessageType {
    Error(CompilationError),
    Warning(CompilationWarning),
    Note(CompilationNote),
}

pub struct DisplayableMessage<'a, 'c> {
    message: &'a CompilationMessage,
    cache: &'a ModuleCache<'c>,
    styling: &'a Styling,
}

impl CompilationMessage {
    pub fn new(location: Location, message: MessageType) -> Self {
        let location = location.as_owned();
        CompilationMessage { location, message }
    }

    pub fn display<'a, 'c>(&'a self, cache: &'a ModuleCache<'c>, styling: &'a Styling) -> DisplayableMessage<'a, 'c> {
        DisplayableMessage { message: self, cache, styling }
    }

    pub fn is_error(&self) -> bool {
        matches!(self.message, MessageType::Error(_))
    }
}

/// Prints a message for the user, with:
/// - the error filename, line and column;
/// - an error explanation;
/// - a visual indicator of the error location.
impl fmt::Display for DisplayableMessage<'_, '_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self { message: CompilationMessage { location, message }, cache, styling } = self;

        let file_contents = read_file_or_panic(&location.filename);
        let line = file_contents.lines().nth(location.start.line.saturating_sub(1) as usize).unwrap_or("");

        let start_column = location.start.column.saturating_sub(1) as usize;
        let location_len = location.length().min(line.len() - start_column);
        let end_column = start_column + location_len;

        writeln!(f, "{} | {} ", location.style(styling.location), "error:".style(styling.header_error))?;

        match message {
            MessageType::Error(error) => writeln!(f, "{}", error.display(cache, styling)),
            MessageType::Warning(warning) => writeln!(f, "{}", warning.display(cache, styling)),
            MessageType::Note(note) => writeln!(f, "{}", note.display(cache, styling)),
        }?;

        writeln!(
            f,
            "{}{}{}",
            &line[..start_column],
            (&line[start_column..end_column]).style(styling.line_wrong_part),
            &line[end_column..],
        )?;

        if styling.underline || location_len == 0 {
            writeln!(f, "{:>width$}", "^".repeat(location_len), width = end_column)?;
        }

        Ok(())
    }
}

/// Format the path in an OS-agnostic way. By default rust uses "/" on Unix
/// and "\" on windows as the path separator. This makes testing more
/// difficult and isn't needed for error reporting so we implement our own
/// path-Displaying here that is roughly the same as printing Unix paths.
struct OsAgnosticPath<'a>(&'a Path);

impl fmt::Display for OsAgnosticPath<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut display_separator = false;

        for component in self.0.components() {
            use std::path::Component;

            // Use / as the separator regardless of the host OS so
            // we can use the same tests for Linux/Mac/Windows
            if display_separator {
                write!(f, "/")?;
            }

            match component {
                Component::CurDir => write!(f, "."),
                Component::Normal(s) => write!(f, "{}", AsRef::<Path>::as_ref(s).display()),
                Component::ParentDir => write!(f, ".."),
                Component::Prefix(_) => write!(f, ""),
                Component::RootDir => write!(f, "/"),
            }?;
            display_separator = component != Component::RootDir;
        }

        Ok(())
    }
}

impl<'a> fmt::Display for Location<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}:{}:{}", OsAgnosticPath(self.filename), self.start.line, self.start.column)
    }
}

impl fmt::Display for OwnedLocation {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}:{}:{}", OsAgnosticPath(&self.filename), self.start.line, self.start.column)
    }
}

impl From<CompilationError> for MessageType {
    fn from(error: CompilationError) -> Self {
        MessageType::Error(error)
    }
}

impl From<CompilationWarning> for MessageType {
    fn from(warning: CompilationWarning) -> Self {
        MessageType::Warning(warning)
    }
}

impl From<CompilationNote> for MessageType {
    fn from(note: CompilationNote) -> Self {
        MessageType::Note(note)
    }
}
