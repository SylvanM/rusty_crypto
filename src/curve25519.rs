use std::ops::{Add, Sub, Mul, Rem, Neg, Div, BitAnd};
use std::str::FromStr;
use rand::Rng;

use crate::bigint::BigInt;

/**
 * A number in the field Z/
 */
pub struct Curve25519Num {
	num: BigInt
}

// For my convenience
type CN = Curve25519Num;

impl CN {
	
}
