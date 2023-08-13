use crate::{
    cache::ModuleCache,
    error::{location::Location, Styling},
    types::Type,
};
use owo_colors::OwoColorize as _;
use std::fmt;

#[derive(Debug)]
pub enum CompilationWarning {
    Todo,
}

impl CompilationWarning {
    pub fn display<'a, 'c>(&'a self, cache: &'a ModuleCache<'c>, styling: &'a Styling) -> DisplayableWarning<'a, 'c> {
        DisplayableWarning { warning: self, cache, styling }
    }

    pub fn todo(message: &str) -> Self {
        todo!("{message}")
    }
}

pub struct DisplayableWarning<'a, 'c> {
    warning: &'a CompilationWarning,
    cache: &'a ModuleCache<'c>,
    styling: &'a Styling,
}

/// Prints a message for the user, with:
/// - the error filename, line and column;
/// - an error explanation;
/// - a visual indicator of the error location.
impl fmt::Display for DisplayableWarning<'_, '_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self { warning, cache, styling } = self;

        match warning {
            Todo => todo!(),
        }
    }
}
