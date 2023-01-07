// ED25519 uses 256 bit key sizes, but uses the integers modulo that big prime number, so we will
// implement all operations in that finite field.

use std::ops::{Add, Sub, Mul, Rem, Neg, Div};
use rand::Rng;

use crate::u256::{U256, self};

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
pub struct ED25519Num {
	/**
	 * Invariant: _num < FIELD_SIZE
	 */
	pub _num: U256 
}

const FIELD_SIZE: U256 = U256 { words: [0xffffffffffffffffffffffffffffffed, 0x7fffffffffffffffffffffffffffffff] };

impl ED25519Num {

	pub const ONE:	ED25519Num = ED25519Num { _num: U256 { words: [1, 0] } }; 
	pub const ZERO: ED25519Num = ED25519Num { _num: U256 { words: [0, 0] } };

	/**
	 * Generates a random number part of the field
	 */
	pub fn rnd() -> ED25519Num {
		let mut rng = rand::thread_rng();

		// basically we're gonna do rejection sampling to get a number in the field.
		let num = U256 { words: [rng.gen(), rng.gen()] };

		if num >= FIELD_SIZE {
			Self::rnd()
		} else {
			ED25519Num { _num: num }
		}
	}
	
	/**
	 * Compute the multiplicative inverse of this number
	 */
	pub fn mul_inv(self) -> ED25519Num {
		let mut inv = u256::ZERO;
		let mut y = u256::ZERO;

		U256::ext_gcd(self._num, FIELD_SIZE, &mut inv, &mut y);

		ED25519Num { _num: inv }
	}

	pub fn gcd(a: ED25519Num, b: ED25519Num) -> ED25519Num {
		ED25519Num { _num: U256::gcd(a._num, b._num) }
	}

	/**
	 * Computes the self^x % FIELD_SIZE
	 */
	pub fn pow(self, x: ED25519Num) -> ED25519Num {
		if x == u256::ZERO.into() {
			u256::ONE.into()
		} else {
			let mut partial_power = x.pow(ED25519Num { _num: x._num >> 1.into() }) % FIELD_SIZE.into();
			partial_power = partial_power * partial_power;

			if x._num.words[0] & 0x1 == 0 {
				partial_power
			} else {
				x * partial_power
			}
		}
	}

	pub fn from(num: U256) -> ED25519Num {
		ED25519Num { _num: num }
	}


}

impl From<U256> for ED25519Num {
	fn from(value: U256) -> Self {
		ED25519Num::from(value)
	}
}

impl Add<ED25519Num> for ED25519Num {
	type Output = ED25519Num;

	fn add(self, rhs: ED25519Num) -> Self::Output {
		ED25519Num { _num: ((self._num % FIELD_SIZE) + (rhs._num % FIELD_SIZE)) % FIELD_SIZE }
	}
}

impl Sub<ED25519Num> for ED25519Num {
	type Output = ED25519Num;

	fn sub(self, rhs: ED25519Num) -> Self::Output {
		self + (-rhs)
	}
}

impl Neg for ED25519Num {
	type Output = ED25519Num;

	fn neg(self) -> Self::Output {
		ED25519Num { _num: (FIELD_SIZE - self._num) % FIELD_SIZE }
	}
}

impl Mul<ED25519Num> for ED25519Num {
	type Output = ED25519Num;

	fn mul(self, rhs: ED25519Num) -> Self::Output {
		ED25519Num { _num: ((self._num % FIELD_SIZE) * (rhs._num % FIELD_SIZE)) % FIELD_SIZE }
	}
}

impl Div<ED25519Num> for ED25519Num {
	type Output = ED25519Num;

	fn div(self, rhs: ED25519Num) -> Self::Output {
		self * rhs.mul_inv()
	}
}

impl Rem<ED25519Num> for ED25519Num {
	type Output = ED25519Num;

	fn rem(self, rhs: ED25519Num) -> Self::Output {
		ED25519Num { _num: self._num % rhs._num }
	}
}

