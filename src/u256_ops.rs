use std::ops;
use std::cmp;
use std::ops::ShlAssign;
use std::ops::ShrAssign;
use cmp::Ordering;
use crate::u256;
use crate::u256::U256;

/**
 * Computes the 128-bit result of the operation `a*b + c + d`
 *
 * This calls compiler intrinsic commands which just call processor instructions or whatever
 */
fn addmul(a: u64, b: u64, c: u64, d: u64) -> (u64, u64) {
	let (mut lo, mut hi) = a.carrying_mul(b, 0);

	let mut add = lo.carrying_add(c, false);
	lo = add.0;

	if add.1 { hi = hi.wrapping_add(1); }

	add = lo.carrying_add(d, false);
	lo = add.0;

	if add.1 { hi = hi.wrapping_add(1); }

	println!("{} {} {} {}", a, b, c, d);
	println!("{}", lo);
	println!("{}", hi);
	println!("NEXT");

	(lo, hi)
}

impl U256 {

	fn full_divide(dividend: U256, divisor: U256, quotient: &mut U256, remainder: &mut U256) {

		assert!(divisor != u256::ZERO, "Divide by zero error!");

		*quotient = 0.into();
		*remainder = dividend;

		if divisor > dividend { return; }

		if divisor == dividend {
			*quotient = 1.into();
			*remainder = 0.into();
			return;
		}

        while *remainder >= divisor {

			let mut partial_product 	= dividend;
        	let mut partial_quotient: U256 = 1.into();

			let div_size = if divisor.words[1] != 0 { 2 } else { 1 };
			let rem_size = if remainder.words[1] != 0 { 2 } else { 1 };
            
            if remainder.words[1] >= divisor.words[1] {
                partial_quotient.words[0] = remainder.words[1] / divisor.words[1];
				partial_quotient <<= ((rem_size - div_size) * u256::WordType::BITS).into();
            }
            else {
				partial_product <<= (((rem_size - div_size) * u256::WordType::BITS) + divisor.words[1].leading_zeros() - remainder.words[1].leading_zeros()).into();
            }

			partial_product = divisor * partial_quotient;
            
            while partial_product > *remainder {
                
                if partial_quotient.words[0] & 1 == 0 {
					partial_product >>= 1.into();
					partial_quotient >>= 1.into();
                }
                else {
					partial_quotient.words[0] = partial_quotient.words[0].wrapping_sub(1);
					partial_product -= divisor;
                }
                
            }

			*remainder -= partial_product;
			*quotient += partial_quotient;
            
        }
		
	}

}

impl ops::Shl for U256 {
	type Output = U256;

	fn shl(self, rhs: Self) -> Self::Output {
		let mut t = self;
		t.shl_assign(rhs);
		t
	}
}

impl ops::ShlAssign for U256 {
	fn shl_assign(&mut self, rhs: Self) {
		if rhs.words[1] != 0 {
			self.words = [0, 0];
			return;
		}

		let bitwidth = u256::WordType::BITS as u128;

		let wordshift = rhs.words[0] / bitwidth;
		let bitshift = rhs.words[0] % bitwidth;

		if wordshift > 1 {
			self.words = [0 , 0];
			return;
		}

		if wordshift == 1 {
			// shift those words!
			self.words[1] = self.words[0];
			self.words[0] = 0;
		}

		// now shift those bits!
		self.words[1] <<= bitshift;
		self.words[1] += self.words[0] >> (bitwidth - bitshift);
		self.words[0] <<= bitshift;

	}
}

impl ops::Shr for U256 {
	type Output = U256;

	fn shr(self, rhs: Self) -> Self::Output {
		let mut t = self;
		t.shr_assign(rhs);
		t
	}
}

impl ops::ShrAssign for U256 {
	fn shr_assign(&mut self, rhs: Self) {
		if rhs.words[1] != 0 {
			self.words = [0, 0];
			return;
		}

		let bitwidth = u256::WordType::BITS as u128;

		let wordshift = rhs.words[0] / bitwidth;
		let bitshift = rhs.words[0] % bitwidth;

		if wordshift > 1 {
			self.words = [0 , 0];
			return;
		}

		if wordshift == 1 {
			// shift those words!
			self.words[0] = self.words[1];
			self.words[1] = 0;
		}

		// now shift those bits!
		self.words[0] >>= bitshift;
        self.words[0] += self.words[1] << (bitwidth - bitshift);
		self.words[1] >>= bitshift;
		
	}
}

impl ops::Add<U256> for U256 {
	type Output = U256;

	fn add(self, rhs: U256) -> U256 {
		let (result, carry) = self.words[0].carrying_add(rhs.words[0], false);
		U256 { 
			words: [result, self.words[1].carrying_add(rhs.words[1], carry).0]
		}
    }
}

impl ops::AddAssign<U256> for U256 {
	fn add_assign(&mut self, rhs: U256) {
		*self = *self + rhs
	}
}

impl ops::Sub<U256> for U256 {
	type Output = U256;

	fn sub(self, rhs: U256) -> Self::Output {
		let (result, borrow) = self.words[0].borrowing_sub(rhs.words[0], false);
		U256 { 
			words: [result, self.words[1].borrowing_sub(rhs.words[1], borrow).0] 
		}
	}
}

impl ops::SubAssign<U256> for U256 {
	fn sub_assign(&mut self, rhs: U256) {
		*self = *self - rhs
	}
}

impl ops::Mul<U256> for U256 {
	type Output = U256;

	fn mul(self, rhs: U256) -> Self::Output {
		if self == u256::ZERO || rhs == u256::ZERO { u256::ZERO }
		else if self == u256::ONE { rhs }
		else if rhs == u256::ONE { self }
		else {
			let a = self.to_u64();
			let b = rhs.to_u64();

			let mut product: [u64 ; 4] = [0 ; 4];

			let mut carry: u64;

			for j in 0..4 {
				carry = 0;
				for i in 0..(4 - j) {
					(product[i + j], carry) = addmul(a[i], b[j], carry, product[i + j])
				}
			}

			U256::from_le_u64(product)
		}
	}
}

impl ops::MulAssign<U256> for U256 {
	fn mul_assign(&mut self, rhs: U256) {
		*self = *self * rhs
	}
}

impl ops::Div<U256> for U256 {
	type Output = U256;

	fn div(self, rhs: U256) -> Self::Output {
		let mut rem = u256::ZERO;
		let mut quo = u256::ZERO;
		U256::full_divide(self, rhs, &mut quo, &mut rem);
		quo
	}

}

impl ops::DivAssign<U256> for U256 {
	fn div_assign(&mut self, rhs: U256) {
		*self = *self / rhs
	}
}

impl ops::Rem<U256> for U256 {
	type Output = U256;

	fn rem(self, rhs: U256) -> Self::Output {
		let mut rem = u256::ZERO;
		let mut quo = u256::ZERO;
		U256::full_divide(self, rhs, &mut quo, &mut rem);
		rem
	}
}

impl ops::RemAssign<U256> for U256 {
	fn rem_assign(&mut self, rhs: U256) {
		*self = *self % rhs
	}
}

impl cmp::PartialEq for U256 {
	fn eq(&self, other: &Self) -> bool {
		self.words == other.words
	}

	fn ne(&self, other: &Self) -> bool {
		self.words != other.words
	}
}

impl cmp::PartialOrd for U256 {
	fn ge(&self, other: &Self) -> bool {
		if self.words[1] == other.words[1] { self.words[0] >= other.words[0] }
		else { self.words[1] >= other.words[1] }
	}

	fn gt(&self, other: &Self) -> bool {
		if self.words[1] == other.words[1] { self.words[0] > other.words[0] }
		else { self.words[1] > other.words[1] }
	}

	fn le(&self, other: &Self) -> bool {
		if self.words[1] == other.words[1] { self.words[0] <= other.words[0] }
		else { self.words[1] <= other.words[1] }
	}

	fn lt(&self, other: &Self) -> bool {
		if self.words[1] == other.words[1] { self.words[0] < other.words[0] }
		else { self.words[1] < other.words[1] }
	}

	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		if self == other { Some(Ordering::Equal) }
		else if self < other { Some(Ordering::Less) }
		else { Some(Ordering::Greater) }
	}
}