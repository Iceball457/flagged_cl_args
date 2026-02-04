use std::collections::HashMap;

use crate::{ArgumentError, FlagDefinition, Variant, VariantFlag};

/// Contains the name of the binary, a list of arguments, and a hashmap of arguments.
pub struct Args {
    binary: String,
    positional: Vec<Variant>,
    named: HashMap<String, Variant>,
}

impl Args {
    /// # Arguments
    ///
    /// `positional_types` is a list of types (stored as [`VariantFlag`]s) your program is expecting.
    /// You do not need to include the name of the binary, it is put into a separate field.
    /// Positional arguments are always required. If the wrong number of positional arguments are supplied, an Err value will be returned.
    ///
    /// `flag_definitions` should contain a list of named arguments (stored as [`FlagDefinition`]s) your program is expecting.
    /// Named arguments are always optional. If a named argument is not supplied, it will simply not be included in the internal HashMap.
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
    pub fn new(
        positional_types: &[VariantFlag],
        flag_definitions: &[FlagDefinition],
    ) -> Result<Args, ArgumentError> {
        Args::from_iter(std::env::args(), positional_types, flag_definitions)
    }

    pub(crate) fn from_iter(
        args: impl Iterator<Item = String>,
        positional_types: &[VariantFlag],
        flag_definitions: &[FlagDefinition],
    ) -> Result<Args, ArgumentError> {
        let mut args = args.enumerate();
        let (_, binary) = args
            .next()
            .ok_or(ArgumentError::new("Argument count is 0"))?;
        let mut named = HashMap::new();
        let mut positional = Vec::new();
        while let Some((index, arg)) = args.next() {
            // Determine if the given flag matches a flag definition
            if let Some(matched_definition) = match_flag_definition(flag_definitions, &arg)? {
                // If the argument is named, we will put it into the hashmap.
                if matched_definition.allowed_type.is_unit() {
                    // There is no next arg, this flag is either present or not present
                    named.insert(matched_definition.name.clone(), Variant::Bool(true));
                } else {
                    // The next argument is a value for this flag
                    let (index, value) = args.next().ok_or(ArgumentError::new(&format!(
                        "Unexpected end of arguments, {} needs a value",
                        matched_definition.name
                    )))?;
                    named.insert(
                        matched_definition.name.clone(),
                        matched_definition
                            .allowed_type
                            .parse(&value)
                            .ok_or(ArgumentError::new(&format!(
                                "Argument {value} at position {index} is not a valid type for --{}",
                                matched_definition.name
                            )))?,
                    );
                }
            } else {
                // If the argument is not named, it must be positional!
                let pos_index = positional.len();
                let allowed_types = positional_types.get(pos_index).ok_or(ArgumentError::new(
                    "There are too many positional arguments",
                ))?;
                positional.push(allowed_types.parse(&arg).ok_or(ArgumentError::new(&format!(
                    "Positional argument {pos_index} at position {index} cannot be parsed as type {allowed_types}"
                )))?);
            }
        }

        if positional.len() != positional_types.len() {
            return Err(ArgumentError::new(
                "Not enough positional arguments were supplied",
            ));
        }

        Ok(Args {
            binary,
            positional,
            named,
        })
    }

    /// Get the first argument, which is normally the name of the binary
    pub fn binary(&self) -> &str {
        &self.binary
    }

    /// Gets a positional argument.
    /// Because the first argument is assumed to be the name of the binary and is kept separately, these indices are offset by 1.
    /// Index 0 refers to the first argument you actually care about.
    ///
    /// Named arguments can be in any position, so the indices may be more or less offset as compared to reading arguments yourself.
    pub fn get_positional(&self, index: usize) -> Option<&Variant> {
        self.positional.get(index)
    }

    /// Gets a named argument.
    pub fn get_named(&self, name: &str) -> Option<&Variant> {
        self.named.get(name)
    }
}

fn match_flag_definition<'a>(
    flag_definitions: &'a [FlagDefinition],
    arg: &str,
) -> Result<Option<&'a FlagDefinition>, ArgumentError> {
    Ok(if arg.starts_with("--") {
        let input_name: String = arg.chars().skip(2).collect();
        Some(
            flag_definitions
                .iter()
                .find(|definition| definition.name == input_name)
                .ok_or(ArgumentError::new(&format!(
                    "--{input_name} does not match any known flag name"
                )))?,
        )
    } else if arg.starts_with('-') && arg.chars().count() == 2 {
        let input_char = arg.chars().last().ok_or(ArgumentError::new("Infallible"))?;
        Some(
            flag_definitions
                .iter()
                .find(|definition| {
                    definition
                        .abbreviation
                        .is_some_and(|abbreviation| input_char == abbreviation)
                })
                .ok_or(ArgumentError::new(&format!(
                    "-{input_char} does not match any known flag abbreviation"
                )))?,
        )
    } else {
        None
    })
}
