use std::{fmt::Debug, ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign}};

use rand::{rngs::StdRng, Rng, SeedableRng};
use sylvan_number::{bignumber::BigNumber, ubignumber::UBigNumber};
use algebra_kit::{algebra::*, std_impls::ZM};


macro_rules! secret_modulus {
	() => {
		UBigNumber::from_words(vec![0x9B, 0, 0, 0, 2])
	};
}

/// The field of integers modulo Q, where Q is that big prime.
#[derive(Clone, Copy)]
pub struct ZMQ {
	pub data: [sylvan_number::ubignumber::Word ; 5]
}

impl Debug for ZMQ {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{:01X}{:016X}{:016X}{:016X}{:016X}", self.data[4], self.data[3], self.data[2], self.data[1], self.data[0])?;
		Ok(())
	}
}

impl ZMQ {

	/// Generates a secret from a UBN
	pub fn from_ubn(ubn: UBigNumber) -> ZMQ {
		let modded = ubn % secret_modulus!();

		let mut words = [0 ; 5];

		for i in 0..5 {
			words[i] = modded.safe_word(i);
		}

		ZMQ { data: words }
	}

	/// A helper function that does modular exponentiation when the exponent is a power of 2, written as 2^n
	fn pow_mod_pow2(self, n: i64) -> ZMQ {
		if n == 0 {
			self
		} else {
			self.clone().pow_mod_pow2(n - 1) * self.pow_mod_pow2(n - 1)
		}
	}

	/// Securely generates a random integer modulo q
	pub fn rnd() -> ZMQ {
		let mut rng = StdRng::from_entropy();
		let msw_set: bool = rng.gen();

		let mut words = [0 ; 5];

		for i in 0..4 {
			words[i] = rng.gen()
		}

		words[4] = if msw_set { 1 } else { 0 };
		
		ZMQ { data: words }
	}

}

impl PartialEq for ZMQ {
	fn eq(&self, other: &Self) -> bool {
		self.data == other.data
	}
}

impl Add for ZMQ {
	type Output = ZMQ;

	fn add(self, rhs: Self) -> Self::Output {
		ZMQ::from_ubn(UBigNumber::from_words(self.data.to_vec()) + UBigNumber::from_words(rhs.data.to_vec()))
	}
}

impl AddAssign for ZMQ {
	fn add_assign(&mut self, rhs: Self) {
		*self = self.clone() + rhs;
	}
}

impl Neg for ZMQ {
	type Output = ZMQ;

	fn neg(self) -> Self::Output {
		ZMQ::from_ubn(secret_modulus!() - UBigNumber::from_words(self.data.to_vec()))
	}
}

impl Sub for ZMQ {
	type Output = ZMQ;

	fn sub(self, rhs: Self) -> Self::Output {
		self + (-rhs)
	}
}

impl SubAssign for ZMQ {
	fn sub_assign(&mut self, rhs: Self) {
		*self = self.clone() - rhs;
	}
}

impl Mul for ZMQ {
	type Output = ZMQ;

	fn mul(self, rhs: Self) -> Self::Output {
		ZMQ::from_ubn(UBigNumber::from_words(self.data.to_vec()) * UBigNumber::from_words(rhs.data.to_vec()))
	}
}

impl MulAssign for ZMQ {
	fn mul_assign(&mut self, rhs: Self) {
		*self = self.clone() * rhs;
	}
}

impl Div for ZMQ {
	type Output = ZMQ;

	fn div(self, rhs: Self) -> Self::Output {
		self * rhs.inverse()
	}
}

impl DivAssign for ZMQ {
	fn div_assign(&mut self, rhs: Self) {
		*self = self.clone() / rhs;
	}
}

impl Ring for ZMQ {
	fn one() -> Self {
		ZMQ::from_ubn(UBigNumber::one())
	}

	fn zero() -> Self {
		ZMQ::from_ubn(UBigNumber::zero())
	}

	fn is_zero(&self) -> bool {
		self.data == [0 ; 5]
	}

	fn power(&self, n: i64) -> Self {		
		let mut product = ZMQ::one();

		for i in 0..64 { // check n, bit by bit!
			if n & (1 << i) != 0 {
				product *= self.clone().pow_mod_pow2(i)
			}
		}

		product
	}
}

impl Field for ZMQ {
	fn inverse(&self) -> Self {
		// compute a modular inverse!
		let bn_self: BigNumber = BigNumber::from_ubn(UBigNumber::from_words(self.data.to_vec()));
		let bn_modulus: BigNumber = secret_modulus!().into();

		let (_, mut x, _) = algebra_kit::algebra::ext_gcd(bn_self, bn_modulus.clone());
		
		while x.is_negative {
			x += bn_modulus.clone();
		}

		ZMQ::from_ubn(x.into())
	}
}

#[cfg(test)]
mod tests {

	use sylvan_number::ubignumber::UBigNumber;
    use algebra_kit::algebra::{Field, Ring};
	use rand::Rng;
	
    use super::ZMQ;

	fn naive_pow(a: ZMQ, b: i64) -> ZMQ {
		let mut product = ZMQ::one();

		for _ in 0..b {
			product *= a;
		}

		product
	}

	#[test]
	fn test_pow() {
		for _ in 0..100 {
			let x = ZMQ::rnd();
			let pow = rand::thread_rng().gen_range(0..10000);

			assert_eq!(naive_pow(x, pow), x.power(pow))
		}
	}

	#[test]
	fn test_mul_inv() {
		for _ in 0..1000 {

			let a = ZMQ::rnd();
			let inv = a.inverse();

			assert_eq!(a * inv, ZMQ::one())
		}
	}

	#[test]
	fn test_add() {
		println!("{:?}", secret_modulus!());
		for _ in 0..10 {
			let a = ZMQ::rnd();
			let b = ZMQ::rnd();
			let c = a + b;

			println!("\n");
			println!("\t{:?}\n+\t{:?}\n------------------------------------------------------------------------- \n\t{:?}", a, b, c);
			println!("\n");
		}
	}
}

