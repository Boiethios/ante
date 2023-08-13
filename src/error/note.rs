use crate::{
    cache::ModuleCache,
    error::{location::Location, Styling},
    types::Type,
};
use owo_colors::OwoColorize as _;
use std::fmt;

#[derive(Debug)]
pub enum CompilationNote {
    Todo,
}

impl CompilationNote {
    pub fn display<'a, 'c>(&'a self, cache: &'a ModuleCache<'c>, styling: &'a Styling) -> DisplayableNote<'a, 'c> {
        DisplayableNote { note: self, cache, styling }
    }

    pub fn todo(message: &str) -> Self {
        todo!("{message}")
    }
}

pub struct DisplayableNote<'a, 'c> {
    note: &'a CompilationNote,
    cache: &'a ModuleCache<'c>,
    styling: &'a Styling,
}

impl fmt::Display for DisplayableNote<'_, '_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self { note, cache, styling } = self;

        match note {
            Todo => todo!(),
        }
    }
}
