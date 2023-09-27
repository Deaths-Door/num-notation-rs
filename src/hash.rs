use std::hash::{Hash, Hasher};
use ordered_float::OrderedFloat;

use crate::Number;

impl Hash for Number { 
    fn hash<H>(&self, state: &mut H) where H: Hasher {
        use Number::*;
        match self {
            Decimal(float) => OrderedFloat(*float).hash(state),
            StandardForm(sf) => (*sf).hash(state),
            Fraction(fr) => fr.hash(state)
        }
    }
}