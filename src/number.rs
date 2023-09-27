use std::ops::{Add,Sub,Mul,Div,Rem,AddAssign,SubAssign,MulAssign,DivAssign,RemAssign,Neg};
use std::cmp::Ordering;

use standardform::StandardForm;
use fraction::GenericFraction;

use crate::ParsingNumberError;

/// Represents a numeric value that can be either a decimal, a number in standard form,
/// or a fraction with a generic numerator and denominator.
///
/// This enum is designed for flexible numeric handling in Rust applications.
#[derive(Debug,Clone,PartialEq,PartialOrd)]
pub enum Number {
    /// Represents a floating-point decimal number.
    Decimal(f64),

    /// Represents a number in the StandardForm notation.
    StandardForm(StandardForm),

    /// Represents a fraction with a generic numerator and denominator
    Fraction(GenericFraction<u32>)
}

impl std::fmt::Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Number::Decimal(d) => write!(f,"{d}"),
            Number::StandardForm(sf) => write!(f,"{sf}"),
            Number::Fraction(fr) => write!(f,"{fr}"),
        }
    }
}

impl From<StandardForm> for Number {
    fn from(value: StandardForm) -> Self {
        Number::StandardForm(value)
    }
}

impl From<GenericFraction<u32>> for Number {
    fn from(value: GenericFraction<u32>) -> Self {
        Number::Fraction(value)
    }
}

impl From<f64> for Number {
    fn from(value: f64) -> Self {
        Number::Decimal(value)
    }
}

impl TryFrom<&str> for Number {
    type Error = ParsingNumberError;
    fn try_from(value : &str) -> Result<Self, Self::Error> {

        let try_into_f64 = value.parse::<f64>();

        if let Ok(double) = try_into_f64 {
            return Ok(Number::Decimal(double));
        } 

        let try_into_fraction = value.parse::<GenericFraction<u32>>();

        if let Ok(fraction) = try_into_fraction {
            return Ok(Number::Fraction(fraction));
        } 
    
        let try_into_sf = StandardForm::try_from(value);

        if let Ok(sf) = try_into_sf {
            return Ok(Number::StandardForm(sf));
        } 

        Err(ParsingNumberError::new(
            try_into_fraction.unwrap_err(),
            try_into_f64.unwrap_err(),
            try_into_sf.unwrap_err()
        ))
    }
}

impl Neg for Number {
    type Output = Self;
    fn neg(self) -> Self::Output {
        match self {
            Number::Decimal(d) => Number::Decimal(-d),
            Number::StandardForm(sf) => Number::StandardForm(-sf),
            Number::Fraction(fr) => Number::Fraction(-fr),
        }
    }
}

impl Eq for Number {}

impl Ord for Number {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}


fn from_fraction_rational_to_sf(sign : fraction::Sign,ratio : fraction::Ratio<u32>) -> StandardForm {
    let (mantissa,exponent) = (match sign{
        fraction::Sign::Plus => *ratio.numer() as f64,
        fraction::Sign::Minus => -(*ratio.numer() as f64)
    }, -(ratio.denom().trailing_zeros() as i8));
    StandardForm::new(mantissa,exponent)
}


impl Add for Number {
    type Output = Number;

    fn add(self,other : Number) -> Self::Output {
        use crate::Number::*;
        match (self,other) {
            (Decimal(d1),Decimal(d2)) => (d1 + d2).into(),
            (Decimal(d),StandardForm(sf)) => (sf + d).into(),
            (Decimal(d),Fraction(fr)) => (fr + d).into(),

            (StandardForm(sf1),StandardForm(sf2)) => (sf1 + sf2).into(),
            (StandardForm(sf),Decimal(d)) => (sf + d).into(),
            (StandardForm(sf),Fraction(fr)) => match fr {
                GenericFraction::Rational(sign,ratio) => (sf + from_fraction_rational_to_sf(sign,ratio)).into(),
                _ => fr.into()
            },

            (Fraction(fr1),Fraction(fr2)) => (fr1 + fr2).into(),
            (Fraction(fr),Decimal(d)) => (fr + d).into(),
            (Fraction(fr),StandardForm(sf)) => match fr {
                GenericFraction::Rational(sign,ratio) => (from_fraction_rational_to_sf(sign,ratio) + sf).into(),
                _ => fr.into()
            },
        }
    }
}

impl Sub for Number {
    type Output = Number;

    fn sub(self,other : Number) -> Self::Output {
        use crate::Number::*;
        match (self,other) {
            (Decimal(d1),Decimal(d2)) => (d1 - d2).into(),
            (Decimal(d),StandardForm(sf)) => (sf - d).into(),
            (Decimal(d),Fraction(fr)) => (fr - d).into(),

            (StandardForm(sf1),StandardForm(sf2)) => (sf1 - sf2).into(),
            (StandardForm(sf),Decimal(d)) => (sf - d).into(),
            (StandardForm(sf),Fraction(fr)) => match fr {
                GenericFraction::Rational(sign,ratio) => (sf - from_fraction_rational_to_sf(sign,ratio)).into(),
                _ => fr.into()
            },

            (Fraction(fr1),Fraction(fr2)) => (fr1 - fr2).into(),
            (Fraction(fr),Decimal(d)) => (fr - d).into(),
            (Fraction(fr),StandardForm(sf)) => match fr {
                GenericFraction::Rational(sign,ratio) => (from_fraction_rational_to_sf(sign,ratio) - sf).into(),
                _ => fr.into()
            },
        }
    }
}

impl Mul for Number {
    type Output = Number;

    fn mul(self,other : Number) -> Self::Output {
        use crate::Number::*;
        match (self,other) {
            (Decimal(d1),Decimal(d2)) => (d1 * d2).into(),
            (Decimal(d),StandardForm(sf)) => (sf * d).into(),
            (Decimal(d),Fraction(fr)) => (fr * d).into(),

            (StandardForm(sf1),StandardForm(sf2)) => (sf1 * sf2).into(),
            (StandardForm(sf),Decimal(d)) => (sf * d).into(),
            (StandardForm(sf),Fraction(fr)) => match fr {
                GenericFraction::Rational(sign,ratio) => (sf * from_fraction_rational_to_sf(sign,ratio)).into(),
                _ => fr.into()
            },

            (Fraction(fr1),Fraction(fr2)) => (fr1 * fr2).into(),
            (Fraction(fr),Decimal(d)) => (fr * d).into(),
            (Fraction(fr),StandardForm(sf)) => match fr {
                GenericFraction::Rational(sign,ratio) => (from_fraction_rational_to_sf(sign,ratio) * sf).into(),
                _ => fr.into()
            },
        }
    }
}

impl Div for Number {
    type Output = Number;

    fn div(self,other : Number) -> Self::Output {
        use crate::Number::*;
        match (self,other) {
            (Decimal(d1),Decimal(d2)) => (d1 / d2).into(),
            (Decimal(d),StandardForm(sf)) => (sf / d).into(),
            (Decimal(d),Fraction(fr)) => (fr / d).into(),

            (StandardForm(sf1),StandardForm(sf2)) => (sf1 / sf2).into(),
            (StandardForm(sf),Decimal(d)) => (sf / d).into(),
            (StandardForm(sf),Fraction(fr)) => match fr {
                GenericFraction::Rational(sign,ratio) => (sf / from_fraction_rational_to_sf(sign,ratio)).into(),
                _ => fr.into()
            },

            (Fraction(fr1),Fraction(fr2)) => (fr1 / fr2).into(),
            (Fraction(fr),Decimal(d)) => (fr / d).into(),
            (Fraction(fr),StandardForm(sf)) => match fr {
                GenericFraction::Rational(sign,ratio) => (from_fraction_rational_to_sf(sign,ratio) / sf).into(),
                _ => fr.into()
            },
        }
    }
}

impl Rem for Number {
    type Output = Number;

    fn rem(self,other : Number) -> Self::Output {
        use crate::Number::*;
        match (self,other) {
            (Decimal(d1),Decimal(d2)) => (d1 % d2).into(),
            (Decimal(d),StandardForm(sf)) => (sf % d).into(),
            (Decimal(d),Fraction(fr)) => (fr % d).into(),

            (StandardForm(sf1),StandardForm(sf2)) => (sf1 % sf2).into(),
            (StandardForm(sf),Decimal(d)) => (sf % d).into(),
            (StandardForm(sf),Fraction(fr)) => match fr {
                GenericFraction::Rational(sign,ratio) => (sf % from_fraction_rational_to_sf(sign,ratio)).into(),
                _ => fr.into()
            },

            (Fraction(fr1),Fraction(fr2)) => (fr1 % fr2).into(),
            (Fraction(fr),Decimal(d)) => (fr % d).into(),
            (Fraction(fr),StandardForm(sf)) => match fr {
                GenericFraction::Rational(sign,ratio) => (from_fraction_rational_to_sf(sign,ratio) % sf).into(),
                _ => fr.into()
            },
        }
    }
}

impl AddAssign for Number {
    fn add_assign(&mut self, other: Number) {
        *self = self.clone() + other;
    }
}

impl SubAssign for Number {
    fn sub_assign(&mut self, other: Number) {
        *self = self.clone() - other;
    }
}

impl MulAssign for Number {
    fn mul_assign(&mut self, other: Number) {
        *self = self.clone() * other;
    }
}

impl DivAssign for Number {
    fn div_assign(&mut self, other: Number) {
        *self = self.clone() / other;
    }
}

impl RemAssign for Number {
    fn rem_assign(&mut self, other: Number) {
        *self = self.clone() % other;
    }
}


macro_rules! primitives {
    (eq => $($t : ty),*) => {
        $(
            impl PartialEq<$t> for Number {
                fn eq(&self, other: &$t) -> bool {
                    match self {
                        Number::Decimal(d) => d == &(*other as f64),
                        Number::StandardForm(sf) => {
                            let xy : StandardForm = (*other).into();
                            sf == &xy
                        },
                        Number::Fraction(fr) => {
                            let xy : GenericFraction<u32> = (*other).into();
                            fr == &xy
                        }
                    }
                }
            }
        )*
    };
    (ord => $($t : ty),*) => {
        $(
            impl PartialOrd<$t> for Number {
                fn partial_cmp(&self, other: &$t) -> Option<Ordering> {
                    match self {
                        Number::Decimal(f) => f.partial_cmp(&(*other as f64)),
                        Number::StandardForm(sf) => {
                            let xy : StandardForm = (*other).into();
                            sf.partial_cmp(&xy)
                        },
                        Number::Fraction(fr) => {
                            let xy : GenericFraction<u32> = (*other).into();
                            fr.partial_cmp(&xy)
                        }
                    }
                }
            }
        )*
    };

    (add => $($t : ty),*) => {
        $(
            impl Add<$t> for Number {
                type Output = Self;
                fn add(self, other: $t) -> Self {
                    match self {
                        Number::Decimal(f) => (f + other as f64).into(),
                        Number::StandardForm(sf) =>(sf + other).into(),
                        Number::Fraction(fr) => (fr + other).into()
                    }
                }
            }
            
            impl AddAssign<$t> for Number {
                fn add_assign(&mut self, other: $t) {
                    *self += Number::Decimal(other as f64)
                }
            }
        )*
    };

    (sub => $($t : ty),*) => {
        $(
            impl Sub<$t> for Number {
                type Output = Self;
                fn sub(self, other: $t) -> Self {
                    match self {
                        Number::Decimal(f) => Number::Decimal(f - other as f64),
                        Number::StandardForm(sf) => Number::StandardForm(sf - other),
                        Number::Fraction(fr) => (fr - other).into()

                    }
                }
            }
            
            impl SubAssign<$t> for Number {
                fn sub_assign(&mut self, other: $t) {
                    *self -= Number::Decimal(other as f64)
                }
            }
        )*
    };
    (mul => $($t : ty),*) => {
        $(
            impl Mul<$t> for Number {
                type Output = Self;
                fn mul(self, other: $t) -> Self {
                    match self {
                        Number::Decimal(f) => Number::Decimal(f * other as f64),
                        Number::StandardForm(sf) => Number::StandardForm(sf * other),
                        Number::Fraction(fr) => (fr * other).into()
                    }
                }
            }
            
            impl MulAssign<$t> for Number {
                fn mul_assign(&mut self, other: $t) {
                    *self *= Number::Decimal(other as f64)
                }
            }
        )*
    };
    (div => $($t : ty),*) => {
        $(
            impl Div<$t> for Number {
                type Output = Self;
                fn div(self, other: $t) -> Self {
                    match self {
                        Number::Decimal(f) => Number::Decimal(f / other as f64),
                        Number::StandardForm(sf) => Number::StandardForm(sf / other),
                        Number::Fraction(fr) => (fr / other).into()
                    }
                }
            }
            
            impl DivAssign<$t> for Number {
                fn div_assign(&mut self, other: $t) {
                    *self /= Number::Decimal(other as f64)
                }
            }
        )*
    };
    (rem => $($t : ty),*) => {
        $(
            impl Rem<$t> for Number {
                type Output = Self;
                fn rem(self, other: $t) -> Self {
                    match self {
                        Number::Decimal(f) => Number::Decimal(f % other as f64),
                        Number::StandardForm(sf) => Number::StandardForm(sf % other),
                        Number::Fraction(fr) => (fr % other).into()
                    }
                }
            }
            
            impl RemAssign<$t> for Number {
                fn rem_assign(&mut self, other: $t) {
                    *self %= Number::Decimal(other as f64)
                }
            }
        )*
    };
    (operations => $($t:ty),*) => {
        $(
            primitives!(add => $t);
            primitives!(sub => $t);
            primitives!(mul => $t);
            primitives!(div => $t);
            primitives!(rem => $t);
        )*
    }
}

primitives!(eq => u8,u16,u32,u64,i8,i16,i32,i64,f32,f64);
primitives!(ord => u8,u16,u32,u64,i8,i16,i32,i64,f32,f64);
primitives!(operations => i8, i16, i32, i64, u8, u16, u32, u64,f32,f64);

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_addition() {
        let num1 = Number::Decimal(2.5);
        let num2 = Number::Decimal(3.5);
        let result = num1 + num2;
        assert_eq!(result, Number::Decimal(6.0));
    }

    // Test subtraction
    #[test]
    fn test_subtraction() {
        let num1 = Number::Decimal(5.5);
        let num2 = Number::Decimal(3.5);
        let result = num1 - num2;
        assert_eq!(result, Number::Decimal(2.0));
    }

    // Test multiplication
    #[test]
    fn test_multiplication() {
        let num1 = Number::Decimal(2.5);
        let num2 = Number::Decimal(3.0);
        let result = num1 * num2;
        assert_eq!(result, Number::Decimal(7.5));
    }

    // Test division
    #[test]
    fn test_division() {
        let num1 = Number::Decimal(10.0);
        let num2 = Number::Decimal(2.0);
        let result = num1 / num2;
        assert_eq!(result, Number::Decimal(5.0));
    }

    // Test addition assignment
    #[test]
    fn test_addition_assignment() {
        let mut num = Number::Decimal(3.0);
        let num2 = Number::Decimal(2.0);
        num += num2;
        assert_eq!(num, Number::Decimal(5.0));
    }

    // Test subtraction assignment
    #[test]
    fn test_subtraction_assignment() {
        let mut num = Number::Decimal(5.0);
        let num2 = Number::Decimal(3.0);
        num -= num2;
        assert_eq!(num, Number::Decimal(2.0));
    }

    // Test multiplication assignment
    #[test]
    fn test_multiplication_assignment() {
        let mut num = Number::Decimal(2.5);
        let num2 = Number::Decimal(3.0);
        num *= num2;
        assert_eq!(num, Number::Decimal(7.5));
    }

    // Test division assignment
    #[test]
    fn test_division_assignment() {
        let mut num = Number::Decimal(10.0);
        let num2 = Number::Decimal(2.0);
        num /= num2;
        assert_eq!(num, Number::Decimal(5.0));
    }

    #[test]
    fn test_display_decimal() {
        let number = Number::Decimal(3.14);
        assert_eq!(format!("{}", number), "3.14");
        assert_eq!(number.to_string(), "3.14");
    }

    #[test]
    fn test_try_from_valid_number() {
        // Test a valid number conversion
        let input = "3.14";
        let result = Number::try_from(input);
        assert!(result.is_ok());

        // Check if the correct variant and value are returned
        if let Ok(Number::Decimal(value)) = result {
            assert_eq!(value, 3.14);
        } else {
            assert!(false, "Expected Ok(Number::Decimal(_)), but got an error.");
        }
    }

    #[test]
    fn test_try_from_invalid_number() {
        // Test an invalid number conversion
        let input = "abc"; // This is not a valid floating-point number
        let result = Number::try_from(input);
        assert!(result.is_err());

        // Check if the correct error variant is returned
        if let Err(_) = result {
        } else {
            assert!(false, "Expected Err(ParseFloatError), but got a success.");
        }
    }

    #[test]
    fn test_try_from_empty_string() {
        // Test conversion from an empty string
        let input = "";
        let result = Number::try_from(input);
        assert!(result.is_err());

        // Check if the correct error variant is returned
        if let Err(_) = result {
        } else {
            assert!(false, "Expected Err(ParseFloatError), but got a success.");
        }
    }
}