// ED25519 uses 256 bit key sizes, and so we shall use the same. We will implement 
// a 256-bit integer type.

use std::fmt::Debug;
use std::ops;
use std::cmp;
use std::convert;
use std::slice::from_raw_parts;
use num_traits::PrimInt;

/**
 * The integer type for the coordinates of our finite field
 */
pub struct ED25519Num {
	pub words: [u128 ; 2]
}

pub const ZERO: ED25519Num = ED25519Num { words: [0, 0] };

// Debugging and Utility

impl ED25519Num {

	fn from_hex_str(hex: &str) -> ED25519Num {
		let bytes = hex.as_bytes();
		let start = if bytes[0] == b'0' && (bytes[1] == b'x' || bytes[1] == b'X') { 2 } else { 1 };
		let padded_bytes: [u8; 32] = [b'0'; 32];
	
		let padded_offset = padded_bytes.len() - bytes.len();
	
		for i in (start..bytes.len()).rev() {
			padded_bytes[i + padded_offset] = bytes[i];
		}
	
		let first_word_slice = unsafe {
			from_raw_parts(padded_bytes.as_ptr().offset(32), 32)
		};
	
		let second_word_slice = unsafe {
			from_raw_parts(padded_bytes.as_ptr(), 32)
		};
	
		let first_word_str = unsafe { std::str::from_utf8_unchecked(first_word_slice) };
		let second_word_str = unsafe { std::str::from_utf8_unchecked(second_word_slice) };
	
		let first_word = u128::from_str_radix(first_word_str, 16).unwrap();
		let second_word = u128::from_str_radix(second_word_str, 16).unwrap();
	
		ED25519Num { words: [first_word, second_word] }
	
	}


	pub fn to_hex(self) -> String {
		let first_str = format!("{:032X}", self.words[0]);
		let second_str = format!("{:#034X}", self.words[1]);
		format!("{}{}", second_str, first_str)
	}

}

impl Debug for ED25519Num {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", to_hex())
	}
}

// Operation Overloading

impl ops::Add<ED25519Num> for ED25519Num {
	type Output = ED25519Num;

	fn add(self, rhs: ED25519Num) -> ED25519Num {
		let (result, carry) = self.words[0].carrying_add(rhs.words[0], false);
		ED25519Num { 
			words: [result, self.words[1].carrying_add(rhs.words[1], carry).0] 
		}
    }
}

impl ops::Sub<ED25519Num> for ED25519Num {
	type Output = ED25519Num;

	fn sub(self, rhs: ED25519Num) -> Self::Output {
		let (result, borrow) = self.words[0].borrowing_sub(rhs.words[0], false);
		ED25519Num { 
			words: [result, self.words[1].borrowing_sub(rhs.words[1], borrow).0] 
		}
	}
}

impl cmp::PartialEq for ED25519Num {
	fn eq(&self, other: &Self) -> bool {
		self.words == other.words
	}

	fn ne(&self, other: &Self) -> bool {
		self.words != other.words
	}
}

// Type Conversions

impl<T: PrimInt> convert::From<T> for ED25519Num {
	fn from(value: T) -> Self {
		match value.to_u128() {
			Some(v) => ED25519Num { words: [v, 0] },
			None => ED25519Num { words: [0, 0] } // This should never be called
		}
	}
}