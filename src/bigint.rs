//
// A simple fixed-width big integer implementation for the purposes of ECC
//

use std::mem::transmute;
use std::slice::from_raw_parts;
use std::ops::{
	Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, 
	DivAssign, Rem, RemAssign, Shl, ShlAssign, Shr, ShrAssign, 
	Index, IndexMut
};
use std::cmp::{
	Eq, PartialEq, PartialOrd, Ordering
};

/// A "digit" in the little-endian representation of a `BigInt`
type Word = u64;

/// The number of words in a `BigInt`, each word of type `Word`
const WORD_COUNT: usize = 8;

/// The number of bits in the representation of a `BigInt`
const BITS: u32 = (WORD_COUNT as u32) * Word::BITS;

/// The number of bytes in the representation of a `BigInt`
const BYTE_COUNT: usize = (BITS as usize) / 8;

/// The number of butes in a word of a `BigInt`
const WORD_BYTE_COUNT: usize = (Word::BITS / 8) as usize;

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct BigInt {
	words: [Word ; WORD_COUNT as usize]
}

impl BigInt {

	// until I figure out macro magic, these arrays will have to change with WORD_COUNT
	pub const ZERO: BigInt = BigInt { words: [0, 0, 0, 0, 0, 0, 0, 0] };
	pub const ONE: 	BigInt = BigInt { words: [1, 0, 0, 0, 0, 0, 0, 0] };

	/**
	 * Returns the little-endian byte representation of this `BigInt`
	 */
	pub fn to_le_bytes(self) -> [u8 ; BYTE_COUNT] {
		let mut bytes: [u8 ; BYTE_COUNT] = [0 ; BYTE_COUNT];
		let byte_lists: [[u8 ; WORD_BYTE_COUNT] ; WORD_COUNT] = self.words.map(|w| w.to_le_bytes());

		for w in 0..WORD_COUNT {
			for i in 0..WORD_BYTE_COUNT {
				bytes[(w * WORD_BYTE_COUNT) + i] = byte_lists[w][i];
			}
		}

		bytes
	}

	/**
	 * Returns the big-endian byte representation of this `BigInt`
	 */
	pub fn to_be_bytes(self) -> [u8 ; BYTE_COUNT] {
		let mut be_bytes = self.to_le_bytes();
		be_bytes.reverse();
		be_bytes
	}

	/**
	 * Creates a `BigInt` from a little-endian sequence of bytes
	 */
	pub fn from_le_bytes(le_bytes: [u8 ; BYTE_COUNT]) -> BigInt {
		let mut be_bytes = le_bytes;
		be_bytes.reverse();
		Self::from_be_bytes(be_bytes)
	}

	/**
	 * Creates a `BigInt` from a single word
	 */
	pub const fn from(word: Word) -> BigInt {
		let mut words = [0 ; WORD_COUNT];
		words[0] = word;
		BigInt { words }
	}

	/**
	 * Creates a `BigInt` from a big-endian sequence of bytes
	 */
	pub fn from_be_bytes(be_bytes: [u8 ; BYTE_COUNT]) -> BigInt {
		let mut words = unsafe { 
			transmute::<[u8 ; BYTE_COUNT], [Word ; WORD_COUNT]>(be_bytes) 
		};

		words.reverse();

		BigInt { words }
	}

	/**
	 * Creates a `BigInt` from a hex string
	 *
	 * Precondition: the string is ASCII, and does NOT begin with "0x"
	 */
	pub fn from_hex_str(hex: &str) -> BigInt {
		let bytes = hex.as_bytes();
		let mut padded = [b'0' ; BYTE_COUNT * 2];
		let mut words = [0 ; WORD_COUNT];

		for i in (0..bytes.len()).rev() {
			padded[i] = bytes[i];
		}

		for w in 0..WORD_COUNT {
			let offset = (BYTE_COUNT * 2) - (2 * WORD_BYTE_COUNT * (w + 1));
			unsafe {
				let slice = from_raw_parts(
					padded.as_ptr().offset(offset as isize), 
					WORD_BYTE_COUNT * 2
				);

				let word_str = std::str::from_utf8_unchecked(slice);
				words[w] = Word::from_str_radix(word_str, 16).unwrap();
			}
		}
		
		BigInt { words }
	}

	/**
	 * Writes a `BigInt` as a hex string, with no "0x" prefix
	 */
	pub fn to_hex_str(self) -> String {
		if WORD_COUNT == 0 { "0".to_string() } else {
			let mut string: String = "".to_string();

			for i in 0..WORD_COUNT {
				// this format string has to change based on WORD_BYTE_COUNT * 2
				string += &format!("{:016X}", self.words[WORD_COUNT - i])[..];
			}

			string
		}
	}
	
	/**
	 * Accesses the most significant word of this `BigInt`
	 */
	fn msw(&mut self) -> &mut Word {
		&mut self.words[self.size()]
	}

	/**
	 * The number of words needed to represent this `BigInt`
	 */
	fn size(self) -> usize {
		for i in (0..WORD_COUNT).rev() {
			if self[i] != 0 { 
				return i + 1;
			}
		}
		1
	}

	/**
	 * Accesses the least significant word of this `BigInt`
	 */
	fn lsw(&mut self) -> &mut Word {
		&mut self.words[0]
	}

	/**
	 * Computes the quotient and remainder after division
	 */
	pub fn full_divide(self, mut divisor: BigInt) -> (BigInt, BigInt) {
		assert!(divisor != Self::ZERO, "Divide by zero error!");

		let mut quotient = Self::ZERO;
		let mut remainder = self;

		if divisor > self { 
			return (quotient, remainder);
		}

		if divisor == self {
			return (Self::ONE, Self::ZERO);
		}

        while remainder >= divisor {

			let mut partial_product	= self;
        	let mut partial_quotient = Self::ONE;
            
            if *remainder.msw() >= *divisor.msw() {
                *partial_quotient.lsw() = *remainder.msw() / *divisor.msw();
				let shift_amount = ((remainder.size() as u32) - (self.size() as u32)) * Word::BITS;
				partial_quotient <<= Self::from(shift_amount.into());
            }
            else {
				let shift_amount =
					(((remainder.size() as u32) - (self.size() as u32)) * Word::BITS ) 
					+ divisor.msw().leading_zeros() - remainder.msw().leading_zeros();
				
				partial_product <<= BigInt::from(shift_amount.into());
            }

			partial_product = divisor * partial_quotient;
            
            while partial_product > remainder {
                if *partial_quotient.lsw() & 1 == 0 {
					partial_product >>= Self::ONE;
					partial_quotient >>= Self::ONE;
                }
                else {
					*partial_quotient.lsw() = partial_quotient.lsw().wrapping_sub(1);
					partial_product -= divisor;
                }
                
            }

			remainder -= partial_product;
			quotient += partial_quotient;
            
        }

		(remainder, quotient)
	}
	
}

// -- Indexing --

impl Index<usize> for BigInt {
	type Output = Word;

	fn index(&self, index: usize) -> &Self::Output {
		&self.words[index]
	}
}

impl IndexMut<usize> for BigInt {
	fn index_mut(&mut self, index: usize) -> &mut Self::Output {
		&mut self.words[index]
	}
}

// -- Comparison --

impl PartialOrd for BigInt {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		for i in (0..WORD_COUNT).rev() {
			if self[i] > other[i] { return Some(Ordering::Greater); }
			else if self[i] < other[i] { return Some(Ordering::Less); }
		}

		return Some(Ordering::Equal);
	}
}

// -- Operations --

impl Add<BigInt> for BigInt {
	type Output = BigInt;

	fn add(self, rhs: BigInt) -> Self::Output {
		let mut carry = false;
		let mut words = [0 ; WORD_COUNT];

		for i in 0..WORD_COUNT {
			(words[i], carry) = self.words[i].carrying_add(rhs.words[i], carry);
		}

		BigInt { words }
	}
}

impl AddAssign<BigInt> for BigInt {
	fn add_assign(&mut self, rhs: BigInt) {
		*self = self.add(rhs);
	}
}

impl Sub<BigInt> for BigInt {
	type Output = BigInt;

	fn sub(self, rhs: BigInt) -> Self::Output {
		let mut borrow = false;
		let mut words = [0 ; WORD_COUNT]; 

		for i in 0..WORD_COUNT {
			(words[i], borrow) = self.words[i].borrowing_sub(rhs.words[i], borrow);
		}

		BigInt { words }
	}
}

impl SubAssign<BigInt> for BigInt {
	fn sub_assign(&mut self, rhs: BigInt) {
		*self = self.sub(rhs);
	}
}

impl Mul<BigInt> for BigInt {
	type Output = BigInt;

	fn mul(self, rhs: BigInt) -> Self::Output {

		fn addmul(a: Word, b: Word, c: Word, d: Word) -> (Word, Word) {
			let (mut lo, mut hi) = a.carrying_mul(b, 0);
		
			let mut add = lo.carrying_add(c, false);
			lo = add.0;
		
			if add.1 { hi = hi.wrapping_add(1); }
		
			add = lo.carrying_add(d, false);
			lo = add.0;
		
			if add.1 { hi = hi.wrapping_add(1); }
			
			(lo, hi)
		}

		if self == Self::ZERO || rhs == Self::ZERO { Self::ZERO }
		else if self == Self::ONE { rhs }
		else if rhs	 == Self::ONE { self }
		else {
			let mut words = [0 ; WORD_COUNT];
			let mut carry: Word;

			for j in 0..WORD_COUNT {
				carry = 0;
				for i in 0..(WORD_COUNT - j) {
					(words[i + j], carry) = addmul(self[i], rhs[j], carry, words[i + j])
				}
			}

			BigInt { words }
		}
	}
}

impl MulAssign<BigInt> for BigInt {
	fn mul_assign(&mut self, rhs: BigInt) {
		*self = self.mul(rhs);
	}
}

impl Div<BigInt> for BigInt {
	type Output = BigInt;

	fn div(self, rhs: BigInt) -> Self::Output {
		let (quotient, _) = self.full_divide(rhs);
		quotient
	}
}

impl DivAssign<BigInt> for BigInt {
	fn div_assign(&mut self, rhs: BigInt) {
		(*self, _) = self.full_divide(rhs);
	}
}

impl Rem<BigInt> for BigInt {
	type Output = BigInt;

	fn rem(self, rhs: BigInt) -> Self::Output {
		let (_, rem) = self.full_divide(rhs);
		rem
	}
}

impl RemAssign<BigInt> for BigInt {
	fn rem_assign(&mut self, rhs: BigInt) {
		(_, *self) = self.full_divide(rhs);
	}
}

impl ShlAssign for BigInt {
	fn shl_assign(&mut self, rhs: Self) {
		let shift = rhs[0]; // not goint to bother generalizing
		let bitshift = shift % Word::BITS as Word;
		let wordshift = shift / Word::BITS as Word;

		for i in (wordshift..(WORD_COUNT) as u64).rev() {
            self[i as usize] = self[(i - wordshift) as usize];
        }

        for i in 0..wordshift {
            self[i as usize] = 0;
        }

        for i in (1..WORD_COUNT).rev() {
            self[i] <<= bitshift;
            self[i] += self[i - 1] >> (Word::BITS - bitshift as u32);
        }

        self[0] <<= bitshift;
	}
}

impl Shl for BigInt {
	type Output = BigInt;

	fn shl(self, rhs: Self) -> Self::Output {
		let mut shifted = self;
		shifted.shl_assign(rhs);
		shifted
	}
}

impl ShrAssign for BigInt {
	fn shr_assign(&mut self, rhs: Self) {
		let shift = rhs[0]; // not goint to bother generalizing
		let bitshift = shift % Word::BITS as Word;
		let wordshift = shift / Word::BITS as Word;

		for i in 0..(WORD_COUNT - wordshift as usize) {
            self[i] = self[i + wordshift as usize];
        }
        
        for i in (WORD_COUNT - wordshift as usize)..WORD_COUNT {
            self[i] = 0;
        }

		for i in 0..(WORD_COUNT - 1) {
            self[i] >>= bitshift;
            self[i] += self[i + 1] << (Word::BITS - bitshift as u32);
        }

        self[WORD_COUNT - 1] >>= bitshift;
	}
}

impl Shr for BigInt {
	type Output = BigInt;

	fn shr(self, rhs: Self) -> Self::Output {
		let mut shifted = self;
		shifted.shr_assign(rhs);
		shifted
	}
}