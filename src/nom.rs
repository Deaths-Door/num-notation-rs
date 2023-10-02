use fraction::GenericFraction;
use standardform::parse_standard_form_with_required_exponent;

use nom::{
    IResult,
    number::complete::double, 
    branch::alt, 
    combinator::{map, opt, map_res},
    character::complete::{char, digit1}, sequence::{pair, preceded},
};

use crate::Number;


/// Parses a numeric input string and returns a `Number` enum.
///
/// This function attempts to parse the input string into different numeric formats
/// and returns a `Number` enum variant based on the successful parsing result.
/// Supported formats include floating-point decimals, standard form numbers, and fractions.
///
/// # Arguments
///
/// * `input` - The input string to parse.
pub fn parse_number(input : &str) -> IResult<&str,Number> {
    alt((
        map(parse_fraction,Number::Fraction),
        map(parse_standard_form_with_required_exponent,Number::StandardForm),
        map(double,Number::Decimal),
    ))(input)
}

fn parse_fraction(input : &str) -> IResult<&str,GenericFraction<u32>> {
    let (input, sign) = opt(char('-'))(input)?;
    let (input, (numerator, denominator)) = pair(
        map_res(digit1, str::parse::<u32>),
        preceded(
            char('/'), 
            map_res(digit1, str::parse::<u32>)
        ),
    )(input)?;
    
    let fraction = match sign {
        Some(_) => GenericFraction::new_neg(numerator, denominator),
        None => GenericFraction::new(numerator ,denominator),
    };

    Ok((input,fraction))
}

#[cfg(test)]
mod parse_fraction_tests {
    use super::*;
    
    #[test]
    fn test_parse_positive_fraction() {
        let input = "2/3";
        let expected_result = Ok(("",GenericFraction::new(2u32, 3u32)));
        assert_eq!(parse_fraction(input), expected_result);
    }
    
    #[test]
    fn test_parse_negative_fraction() {
        let input = "-4/5";
        let expected_result = Ok(("",(GenericFraction::new_neg(4u32, 5u32))));
        assert_eq!(parse_fraction(input), expected_result);
    }
    
    #[test]
    fn test_parse_invalid_input() {
        let input = "abc";
        assert!(parse_fraction(input).is_err());
    }
}


#[cfg(test)]
mod tests {
    use standardform::StandardForm;

    use super::*;
    
    #[test]
    fn test_parse_decimal() {
        let input = "3.14";
        let expected_result = Ok(("", Number::Decimal(3.14)));
        assert_eq!(parse_number(input), expected_result);
    }
    
    #[test]
    fn test_parse_standard_form_2() {
        let input = "1*10^-9";
        let expected_result = Ok(("", Number::StandardForm(StandardForm::new(1.0,-9))));
        assert_eq!(parse_number(input), expected_result);
    }
    #[test]
    fn test_parse_fraction() {
        let input = "2/3";
        let expected_result = Ok(("", Number::Fraction(GenericFraction::new(2u32, 3u32))));
        assert_eq!(parse_number(input), expected_result);
    }
    
    #[test]
    fn test_parse_invalid_input() {
        let input = "abc";
        assert!(parse_number(input).is_err());
    }
}
