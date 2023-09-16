use num_traits::{
    identities::{One,Zero},
    sign::Signed,
    cast::FromPrimitive,
    Num
};

use crate::{Number,ParsingNumberError};

impl Zero for Number {
    fn zero() -> Self {
        Number::Decimal(0.0)
    }

    fn is_zero(&self) -> bool {
        self == &0_f64
    }
}

impl One for Number {
    fn one() -> Self {
        Number::Decimal(1.0)
    }

    fn is_one(&self) -> bool {
        self == &1_f64
    }
}

impl FromPrimitive for Number {
    // Required methods
    fn from_i64(n: i64) -> Option<Self> {
        Some(Self::from(n as f64))
    }
    fn from_u64(n: u64) -> Option<Self> {
        Some(Self::from(n as f64))
    }
}

impl Num for Number {
    type FromStrRadixErr = ParsingNumberError;
    #[inline]
    fn from_str_radix(s: &str, radix: u32)-> Result<Self,Self::FromStrRadixErr> {
        Self::try_from(s)
    }
}

impl Signed for Number {
    fn abs(&self) -> Self {
        match self {
            Number::Decimal(d) => (d.abs()).into(),
            Number::StandardForm(sf) => (sf.abs()).into(),
            Number::Fraction(fr) => (fr.abs()).into(),
        }
    }

    fn abs_sub(&self, other: &Self) -> Self {
        match self <= other {
            true => Self::zero(),
            false => self.clone() - other.clone()
        }
    }
    
    fn signum(&self) -> Self {
        if self.is_zero() { Self::zero() }
        else if self.is_positive() { Self::one() }
        else { -Self::one() } // if self.is_negative() { -Self::one()}
    }

    fn is_positive(&self) -> bool {
        match self {
            Number::Decimal(d) => d.is_positive(),
            Number::StandardForm(sf) => sf.is_positive(),
            Number::Fraction(fr) => fr.is_positive(),
        }
    }

    fn is_negative(&self) -> bool {
        match self {
            Number::Decimal(d) => d.is_negative(),
            Number::StandardForm(sf) => sf.is_negative(),
            Number::Fraction(fr) => fr.is_negative(),
        }
    }
}