use std::num::ParseFloatError;

use thiserror::Error;

use fraction::error::ParseError as ParsingFractionError;
use standardform::ParsingStandardFormError ;

/// Error type for parsing number-related errors.
/// This error type encapsulates specific parsing errors for fractions, floating-point numbers,
/// and numbers in standard form.
#[derive(Error,Debug,Clone)]
#[error("Failed to parse the number:\nfraction error: {fraction},\ndouble error: {double},\nstandard form error: {sf}",)]
pub struct ParsingNumberError {
    fraction : ParsingFractionError,
    double : ParseFloatError,
    sf : ParsingStandardFormError 
}

impl ParsingNumberError {
    pub(crate) fn new(fraction : ParsingFractionError, double : ParseFloatError,sf : ParsingStandardFormError) -> Self {
        Self { fraction , double , sf }
    }

    /// Retrieves a reference to the parsing error related to fractions.
    pub fn fraction(&self) -> &ParsingFractionError {
        &self.fraction
    }

    /// Retrieves a reference to the parsing error related to floating-point numbers.
    pub fn double(&self) -> &ParseFloatError {
        &self.double
    }

    /// Retrieves a reference to the parsing error related to numbers in standard form.
    pub fn standardform(&self) -> &ParsingStandardFormError {
        &self.sf
    }
}