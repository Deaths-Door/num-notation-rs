use num_traits::{
    identities::{One,Zero},
    sign::Signed,
    cast::FromPrimitive,
    ToPrimitive,
    Num,
    Pow
};

use fraction::GenericFraction;
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

impl ToPrimitive for Number {
    // Required methods
    fn to_i64(&self) -> Option<i64> {
        let x : f64 = self.clone().into();

        Some(x as i64)
    }
    fn to_u64(&self) -> Option<u64> {
        let x : f64 = self.clone().into();

        Some(x as u64)
    }
}

impl Num for Number {
    type FromStrRadixErr = ParsingNumberError;
    #[inline]
    fn from_str_radix(s: &str, _: u32)-> Result<Self,Self::FromStrRadixErr> {
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

fn fraction_f64_values<F: FnOnce(f64,f64) -> Number>(fr : GenericFraction<u32>,calculate : F) -> Number {
    match fr {
        GenericFraction::Rational(sign,ratio) => {
            let mut n = *ratio.numer() as f64;

            if sign.is_negative() {
                n = -n;
            }

            let d = *ratio.denom() as f64;

            calculate(n,d)
        },
        _ => fr.into()
    }
}

impl Pow<Self> for Number {
    type Output = Self;

    fn pow(self,other : Self) -> Self {
        match (self,other) {
            (Number::Decimal(d1), Number::Decimal(d2)) => d1.powf(d2).into(),
            (Number::Decimal(d), Number::StandardForm(sf)) => d.powf(sf.into()).into(),
            (Number::Decimal(d1), Number::Fraction(fr)) => fraction_f64_values(fr,|n,d| d1.powf(n/d).into()),
            (Number::StandardForm(sf), Number::Decimal(d)) => f64::from(sf).powf(d).into(),
            (Number::StandardForm(sf1), Number::StandardForm(sf2)) => sf1.pow(sf2).into(),
            (Number::StandardForm(sf), Number::Fraction(fr)) => fraction_f64_values(fr,|n,d| f64::from(sf).powf(n/d).into()),
            (Number::Fraction(fr), Number::Decimal(d1)) => fraction_f64_values(fr,|n,d| (n/d).powf(d1).into()),
            (Number::Fraction(fr), Number::StandardForm(sf)) =>fraction_f64_values(fr,|n,d| (n/d).powf(sf.into()).into()),
            (Number::Fraction(fr1), Number::Fraction(fr2)) => fraction_f64_values(fr1,|n1,d1| fraction_f64_values(fr2,|n2,d2| (n1/d1).powf(n2/d2).into())),
        }
    }
}