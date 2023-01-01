// ED25519 uses 256 bit key sizes, and so we shall use the same. We will implement 
// a 256-bit integer type.

use std::fmt::Debug;
use std::convert;
use std::io::BufWriter;
use std::ops::RangeFrom;
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
pub const ONE: 	U256 = U256 { words: [1, 0] };

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

	pub fn to_bytes(self) -> [u8 ; 32] {
		let mut bytes: [u8 ; 32] = [0 ; 32];
		let first_word_bytes = self.words[0].to_le_bytes();
		let second_word_bytes = self.words[1].to_le_bytes();
		
		for i in 0..16 {
			bytes[i] = first_word_bytes[i];
		}

		for i in 0..16 {
			bytes[i + 16] = second_word_bytes[i]; 
		}

		bytes
	}

}

impl Debug for U256 {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let str = (*self).to_hex();
		write!(f, "{}", str)
	}
}

// Type Conversions

/*
 * You may be wondering: Sylvan, why on earth did you do it this way? This looks terrible! Surely there is a much more 
 * elegant way to write this. 
 * 
 * And to that I'd say, I know! I hate it! If you have a better way to implement it, please let me know, I'm
 * figuring out Rust. I've tried using the num_traits::PrimInt trait to group these all together, 
 * (shown in the comment) but then I can't implement other conversions from other types, since Rust is concerned 
 * that in the future that time may conform to PrimInt. So, we'll see if I can find a better way to do it.
 * 
 */
// impl<T: PrimInt> convert::From<T> for U256 {
// 	fn from(value: T) -> Self {
// 		match value.to_u128() {
// 			Some(v) => U256 { words: [v, 0] },
// 			None 	=> U256 { words: [0, 0] } // This should never be called
// 		}
// 	}
// }

// Integer conversions

impl convert::From<u8> for U256 {
	fn from(value: u8) -> Self {
		U256 { words: [value as u128, 0] }
	}
}

impl convert::From<u16> for U256 {
	fn from(value: u16) -> Self {
		U256 { words: [value as u128, 0] }
	}
}

impl convert::From<u32> for U256 {
	fn from(value: u32) -> Self {
		U256 { words: [value as u128, 0] }
	}
}

impl convert::From<u64> for U256 {
	fn from(value: u64) -> Self {
		U256 { words: [value as u128, 0] }
	}
}

impl convert::From<u128> for U256 {
	fn from(value: u128) -> Self {
		U256 { words: [value as u128, 0] }
	}
}

// Other conversion types

impl convert::From<&str> for U256 {
	fn from(value: &str) -> Self {
		U256::from_hex_str(value)
	}
}

impl convert::From<[u8 ; 32]> for U256 {

	/**
	 * Converts a byte sequence to a U256, interpreting the sequence as little-endian
	 */
	fn from(value: [u8 ; 32]) -> Self {
		let lo = &value[0..16];
		let hi = &value[16..32];
		let first_word = u128::from_le_bytes(lo.try_into().expect("couldn't convert lo slice type"));
		let second_word = u128::from_le_bytes(hi.try_into().expect("couldn't convert hi slice type"));
		U256 { words: [first_word, second_word] }
	}

}