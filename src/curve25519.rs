use std::ops::{Add, Sub, Mul, Neg, Div};
use std::cmp::{Eq, PartialEq, PartialOrd};
use std::fmt::Debug;

use crate::bigint::{BigInt, self, WORD_BYTE_COUNT};

const LE_25519_WORDS: [u64 ; bigint::WORD_COUNT] = [
	0xffffffffffffffed, 0xffffffffffffffff, 
	0xffffffffffffffff, 0x7fffffffffffffff,
	0x0000000000000000, 0x0000000000000000,
	0x0000000000000000, 0x0000000000000000
];

pub const CURVE_MODULUS: BigInt = BigInt { words: LE_25519_WORDS };

/// The number of bits in an ED25519 key
pub const KEY_BIT_COUNT: usize = 256;

/// The number of bytes in an ED25519 key
pub const KEY_BYTE_COUNT: usize = KEY_BIT_COUNT / 8;

/// The number of BN words required to represent an ED25519 key
pub const KEY_BN_WORD_COUNT: usize = KEY_BYTE_COUNT / WORD_BYTE_COUNT;

/// A number that represents an ED25519 key, or a coordinate as part of a 
/// point in the group.
#[derive(PartialEq, Eq, PartialOrd, Clone, Copy)]
pub struct Curve25519Num {
	pub num: BigInt
}

// For my convenience
type CN = Curve25519Num;

impl CN {
	
	/// Returns the little-endian byte representation of this field element
	pub fn to_le_bytes(self) -> [u8 ; bigint::BYTE_COUNT] {
		self.num.to_le_bytes()
	}

	/// Returns the little-endian byte representation of this field element
	pub fn to_be_bytes(self) -> [u8 ; bigint::BYTE_COUNT] {
		self.num.to_be_bytes()
	}

	/// Creates a field element from little endian bytes
	pub fn from_le_bytes(le_bytes: [u8 ; bigint::BYTE_COUNT]) -> CN {
		Self { num: BigInt::from_le_bytes(le_bytes) }
	}

	/// Creates a field element from big endian bytes
	pub fn from_be_bytes(be_bytes: [u8 ; bigint::BYTE_COUNT]) -> CN {
		Self { num: BigInt::from_be_bytes(be_bytes) }
	}

	/// Creates a field element from a hex string
	pub fn from_hex_str(hex: &str) -> CN {
		CN { num: BigInt::from_hex_str(hex) }
	}

	/// Shows this field element as a hex string
	pub fn to_hex_str(self) -> String {
		self.num.to_hex_str()
	}

	/// Creates a random Field Element, *NOT* cryptographically secure
	pub fn rnd() -> CN {
		loop {
			let num = BigInt::rnd(KEY_BN_WORD_COUNT);
			if num < CURVE_MODULUS { 
				return CN { num };
			}
		}
	}

	/// Modular exponentiation
	/// 
	/// Computes `self^power` in this field
	pub fn pow(self, power: BigInt) -> CN {
		CN { num: self.num.pow_mod(power, CURVE_MODULUS) }
	}

}

// -- Converstions -- 

impl From<&str> for CN {
	fn from(value: &str) -> Self {
		CN { num: value.into() }
	}
}

// -- Convenience -- 

impl Debug for CN {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		self.num.fmt(f)
	}
}

// -- Operations -- 

impl Add<CN> for CN {
	type Output = CN;

	fn add(self, rhs: CN) -> Self::Output {
		CN { num: BigInt::mod_add(self.num, rhs.num, CURVE_MODULUS) }
	}
}

impl Neg for CN {
	type Output = CN;

	fn neg(self) -> Self::Output {
		CN { num: self.num.mod_add_inv(CURVE_MODULUS) }
	}
}

impl Sub<CN> for CN {
	type Output = CN;

	fn sub(self, rhs: CN) -> Self::Output {
		CN { num: BigInt::mod_sub(self.num, rhs.num, CURVE_MODULUS)}
	}
}

impl Mul<CN> for CN {
	type Output = CN;

	fn mul(self, rhs: CN) -> Self::Output {
		CN { num: BigInt::mod_mul(self.num, rhs.num, CURVE_MODULUS)}
	}
}

impl Div<CN> for CN {
	type Output = CN;

	fn div(self, rhs: CN) -> Self::Output {
		CN { num: BigInt::mod_div(self.num, rhs.num, CURVE_MODULUS)}
	}
}