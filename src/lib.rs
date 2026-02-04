//! A tool for sorting and parsing command line arguments!
//!
//! [`gather_command_line_flags`] is your entry point. See also [`FlagDefinition`], [`Variant`], and [`Args`].

mod args;
mod variant;

pub use crate::args::Args;
pub use crate::variant::Variant;
pub use crate::variant::VariantFlag;
use std::{error::Error, fmt::Display};

/// Defines a named argument that your program is expecting.
///
/// ```
/// FlagDefinition {
///     name: "example".to_string(),
///     abbreviation: Some('e'),
///     allowed_type: VariantFlag::new_bool(),
/// }
/// ```
/// This value will be set by `binary_name --example true` or `binary_name -e false`
pub struct FlagDefinition {
    /// The name of the flagged argument.
    /// Your end users can set this argument by passing `--name <value>`.
    /// This will also be the key in the HashMap<String, Variant> produced by [`gather_command_line_flags`].
    pub name: String,
    /// An optional abbreviation that can be set with `-a <value>`.
    /// The named argument's key is still the argument's name, even if it was passed using its abbreviation.
    pub abbreviation: Option<char>,
    /// The type(s) that [`gather_command_line_flags`] will attempt to parse the given value into.
    pub allowed_type: VariantFlag,
}

/// A simple error type.
/// If something is wrong with the user's input, showing them this error will guide them to correcting it!
#[derive(Debug)]
pub struct ArgumentError(String);

impl ArgumentError {
    fn new(description: &str) -> ArgumentError {
        ArgumentError(description.to_string())
    }
}

impl Error for ArgumentError {}

impl Display for ArgumentError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// An alias to [`crate::Args::new`]
///
/// # Errors
///
/// Argument error contains a description of why the arguments could not be parsed, pass this along to your end user.
///
/// Reasons include:
/// - There were the wrong number of positional arguments
/// - A named argument wasn't listed in the flag definitions
/// - A non-unit named argument didn't have another argument after it
/// - A value could not be parsed into any of the types it is allowed to become
pub fn gather_command_line_flags(
    positional_types: &[VariantFlag],
    flag_definitions: &[FlagDefinition],
) -> Result<Args, ArgumentError> {
    Args::new(positional_types, flag_definitions)
}
