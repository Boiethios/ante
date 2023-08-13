use crate::{
    cache::ModuleCache,
    error::{location::Location, Styling},
    types::{FunctionType, Type},
};
use owo_colors::OwoColorize as _;
use std::{borrow::Borrow, fmt};

#[derive(Debug)]
pub enum CompilationError {
    MismatchedParameters { got: Type, expected: Type },
    RefRequiredForAssignment { got: Type },
    CannotAssignToRef { got: Type, expected: Type },
    ValueIsNotAFunction { got: Type },
    InvalidNumberOfParameters { function: FunctionType, got: usize, expected: usize },
}

impl CompilationError {
    pub fn display<'a, 'c>(&'a self, cache: &'a ModuleCache<'c>, styling: &'a Styling) -> DisplayableError<'a, 'c> {
        DisplayableError { error: self, cache, styling }
    }

    pub fn todo(message: &str) -> Self {
        todo!("{message}")
    }

    pub fn mismatched_parameters(expected: impl Borrow<Type>, got: impl Borrow<Type>) -> Self {
        CompilationError::MismatchedParameters { expected: expected.borrow().clone(), got: got.borrow().clone() }
    }

    pub fn ref_required_for_assignment(got: impl Borrow<Type>) -> Self {
        CompilationError::RefRequiredForAssignment { got: got.borrow().clone() }
    }

    pub fn cannot_assign_to_ref(expected: impl Borrow<Type>, got: impl Borrow<Type>) -> Self {
        CompilationError::CannotAssignToRef { expected: expected.borrow().clone(), got: got.borrow().clone() }
    }

    pub fn value_is_not_a_function(got: impl Borrow<Type>) -> Self {
        CompilationError::ValueIsNotAFunction { got: got.borrow().clone() }
    }
}

pub struct DisplayableError<'a, 'c> {
    error: &'a CompilationError,
    cache: &'a ModuleCache<'c>,
    styling: &'a Styling,
}

impl fmt::Display for DisplayableError<'_, '_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self { error, cache, styling } = self;

        match error {
            CompilationError::MismatchedParameters { expected, got } => {
                writeln!(
                    f,
                    "Mismatched parameters: expected {}, got {}",
                    expected.display(cache).style(styling.type_),
                    got.display(cache).style(styling.wrong_type),
                )
            },
            CompilationError::RefRequiredForAssignment { got } => writeln!(
                f,
                "Expression of type {} must be a `ref a` type to be assigned to",
                got.display(cache).style(styling.wrong_type),
            ),
            CompilationError::CannotAssignToRef { expected, got } => {
                writeln!(
                    f,
                    "Cannot assign expression of type {} to a ref of type {}",
                    got.display(cache).style(styling.wrong_type),
                    expected.display(cache).style(styling.type_),
                )
            },
            CompilationError::ValueIsNotAFunction { got } => {
                writeln!(
                    f,
                    "Value being called is not a function, it is a {}",
                    got.display(cache).style(styling.wrong_type),
                )
            },
            CompilationError::InvalidNumberOfParameters { function, got, expected } => {
                writeln!(
                    f,
                    "Function {} declared to take {} parameter{}, but {} were supplied",
                    Type::Function(function.clone()).display(cache).style(styling.wrong_type),
                    expected.style(styling.type_),
                    if *expected < 2 { "" } else { "s" },
                    got.style(styling.wrong_type),
                )
            },
        }
    }
}
