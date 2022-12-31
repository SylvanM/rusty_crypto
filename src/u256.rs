// ED25519 uses 256 bit key sizes, and so we shall use the same. We will implement 
// a 256-bit integer type.

use std::fmt::Debug;
use std::convert;
use std::slice::from_raw_parts;
use num_traits::PrimInt;

/**
 * The integer type for the coordinates of our finite field
 */
#[derive(Clone, Copy)]
pub struct U256 {
	pub words: [u128 ; 2]
}

pub const ZERO: U256 = U256 { words: [0, 0] };

// Debugging and Utility

impl U256 {

	pub fn from_hex_str(hex: &str) -> U256 {
		let bytes = hex.as_bytes();
		let start = if bytes[0] == b'0' && (bytes[1] == b'x' || bytes[1] == b'X') { 2 } else { 0 };
		let mut padded_bytes: [u8; 64] = [b'0'; 64];
	
		let padded_offset: i8 = (padded_bytes.len() as i8) - (bytes.len() as i8);
	
		for i in ((start as i8)..(bytes.len() as i8)).rev() {
			let index = i + padded_offset;
			padded_bytes[index as usize] = bytes[i as usize];
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
	
		U256 { words: [first_word, second_word] }
	
	}


	pub fn to_hex(self) -> String {
		let first_str = format!("{:032X}", self.words[0]);
		let second_str = format!("{:#034X}", self.words[1]);
		format!("{}{}", second_str, first_str)
	}

}

impl Debug for U256 {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let str = (*self).to_hex();
		write!(f, "{}", str)
	}
}

// Type Conversions

impl<T: PrimInt> convert::From<T> for U256 {
	fn from(value: T) -> Self {
		match value.to_u128() {
			Some(v) => U256 { words: [v, 0] },
			None => U256 { words: [0, 0] } // This should never be called
		}
	}
}