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
	 * Computes the quotient and remainder after division
	 */
	pub fn full_divide(self, divisor: BigInt) -> (BigInt, BigInt) {
		// TODO: Actually do this!
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