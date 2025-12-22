use std::fmt;
use std::path::PathBuf;

use serde::Serialize;

use crate::arg_matcher::ArgMatcher;
use crate::arg_resolver::PositionalArg;
use serde_with::DisplayFromStr;
use serde_with::serde_as;

pub type Result<T> = std::result::Result<T, Error>;

#[serde_as]
#[derive(Debug, Eq, PartialEq, Serialize)]
#[serde(tag = "type")]
pub enum Error {
    NoSpecForProgram {
        program: String,
    },
    OptionMissingValue {
        program: String,
        option: String,
    },
    OptionFollowedByOptionInsteadOfValue {
        program: String,
        option: String,
        value: String,
    },
    UnknownOption {
        program: String,
        option: String,
    },
    UnexpectedArguments {
        program: String,
        args: Vec<PositionalArg>,
    },
    DoubleDashNotSupportedYet {
        program: String,
    },
    MultipleVarargPatterns {
        program: String,
        first: ArgMatcher,
        second: ArgMatcher,
    },
    RangeStartExceedsEnd {
        start: usize,
        end: usize,
    },
    RangeEndOutOfBounds {
        end: usize,
        len: usize,
    },
    PrefixOverlapsSuffix {},
    NotEnoughArgs {
        program: String,
        args: Vec<PositionalArg>,
        arg_patterns: Vec<ArgMatcher>,
    },
    InternalInvariantViolation {
        message: String,
    },
    VarargMatcherDidNotMatchAnything {
        program: String,
        matcher: ArgMatcher,
    },
    EmptyFileName {},
    LiteralValueDidNotMatch {
        expected: String,
        actual: String,
    },
    InvalidPositiveInteger {
        value: String,
    },
    MissingRequiredOptions {
        program: String,
        options: Vec<String>,
    },
    SedCommandNotProvablySafe {
        command: String,
    },
    ReadablePathNotInReadableFolders {
        file: PathBuf,
        folders: Vec<PathBuf>,
    },
    WriteablePathNotInWriteableFolders {
        file: PathBuf,
        folders: Vec<PathBuf>,
    },
    CannotCheckRelativePath {
        file: PathBuf,
    },
    CannotCanonicalizePath {
        file: String,
        #[serde_as(as = "DisplayFromStr")]
        error: std::io::ErrorKind,
    },
    InvalidDecision {
        decision: String,
    },
    ExampleDidNotMatch {
        rules: Vec<String>,
        examples: Vec<String>,
    },
    ExampleDidMatch {
        rule: String,
        example: String,
    },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::NoSpecForProgram { program } => {
                write!(f, "no spec for program: {program}")
            }
            Error::OptionMissingValue { program, option } => {
                write!(f, "option missing value: {program} {option}")
            }
            Error::OptionFollowedByOptionInsteadOfValue {
                program,
                option,
                value,
            } => {
                write!(
                    f,
                    "option followed by option instead of value: {program} {option} {value}"
                )
            }
            Error::UnknownOption { program, option } => {
                write!(f, "unknown option: {program} {option}")
            }
            Error::UnexpectedArguments { program, args } => {
                write!(f, "unexpected arguments: {program} {:?}", args)
            }
            Error::DoubleDashNotSupportedYet { program } => {
                write!(f, "double dash not supported yet: {program}")
            }
            Error::MultipleVarargPatterns {
                program,
                first,
                second,
            } => {
                write!(
                    f,
                    "multiple vararg patterns: {program} {:?} {:?}",
                    first, second
                )
            }
            Error::RangeStartExceedsEnd { start, end } => {
                write!(f, "range start exceeds end: {start} > {end}")
            }
            Error::RangeEndOutOfBounds { end, len } => {
                write!(f, "range end out of bounds: {end} >= {len}")
            }
            Error::PrefixOverlapsSuffix {} => {
                write!(f, "prefix overlaps suffix")
            }
            Error::NotEnoughArgs {
                program,
                args,
                arg_patterns,
            } => {
                write!(
                    f,
                    "not enough args: {program} {:?} {:?}",
                    args, arg_patterns
                )
            }
            Error::InternalInvariantViolation { message } => {
                write!(f, "internal invariant violation: {message}")
            }
            Error::VarargMatcherDidNotMatchAnything { program, matcher } => {
                write!(
                    f,
                    "vararg matcher did not match anything: {program} {:?}",
                    matcher
                )
            }
            Error::EmptyFileName {} => {
                write!(f, "empty file name")
            }
            Error::LiteralValueDidNotMatch { expected, actual } => {
                write!(
                    f,
                    "literal value did not match: expected {expected}, got {actual}"
                )
            }
            Error::InvalidPositiveInteger { value } => {
                write!(f, "invalid positive integer: {value}")
            }
            Error::MissingRequiredOptions { program, options } => {
                write!(f, "missing required options: {program} {:?}", options)
            }
            Error::SedCommandNotProvablySafe { command } => {
                write!(f, "sed command not provably safe: {command}")
            }
            Error::ReadablePathNotInReadableFolders { file, folders } => {
                write!(
                    f,
                    "readable path not in readable folders: {:?} {:?}",
                    file, folders
                )
            }
            Error::WriteablePathNotInWriteableFolders { file, folders } => {
                write!(
                    f,
                    "writeable path not in writeable folders: {:?} {:?}",
                    file, folders
                )
            }
            Error::CannotCheckRelativePath { file } => {
                write!(f, "cannot check relative path: {:?}", file)
            }
            Error::CannotCanonicalizePath { file, error } => {
                write!(f, "cannot canonicalize path: {file} ({error:?})")
            }
            Error::InvalidDecision { decision } => {
                write!(f, "invalid decision: {decision}")
            }
            Error::ExampleDidNotMatch { rules, examples } => {
                write!(
                    f,
                    "example did not match: rules {:?}, examples {:?}",
                    rules, examples
                )
            }
            Error::ExampleDidMatch { rule, example } => {
                write!(
                    f,
                    "example did match but should not: rule {rule}, example {example}"
                )
            }
        }
    }
}

impl std::error::Error for Error {}
