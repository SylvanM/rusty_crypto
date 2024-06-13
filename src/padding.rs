///
/// Padding! This is SHA-2 style padding
/// 

/**
 * Pads the remaining bits of a buffer given a start byte and bit (within that byte) and an ending byte
 * The end_byte is EXCLUDED
 * 
 * This treats each byte as being little endian in its bits
 */
// pub fn pad(buf: &mut [u8], start_byte: usize, start_bit: usize, end_byte: usize) {
// 	buf[start_byte] |= 1 << start_bit;

// 	if start_byte == end_byte - 1 {
// 		return;
// 	}

// 	for i in (start_byte + 1)..end_byte {
// 		buf[i] = 0;
// 	}
// }

use std::mem::transmute;

use crate::lwe::enc;

/**
 * Pads the remaining bits of a buffer given a start byte and bit (within that byte) and an ending byte
 * The end_byte is EXCLUDED
 * 
 * This treats each byte as being little endian in its bits
 */
pub fn pad_buf(buf: &mut [u8], start_byte: usize, start_bit: usize, end_byte: usize) {
	buf[start_byte] |= 1 << start_bit;

	if start_byte == end_byte - 1 {
		return;
	}

	for i in (start_byte + 1)..end_byte {
		buf[i] = 0;
	}
}

// I am sorry to the reader. Macros are really unreadable! But alas, in this case,
// it seemed the logical way to implement this.

macro_rules! make_sha_pad {
	(
		$name:ident,
		$unname:ident,
		$test_suite_name:ident,
		$block_len:literal,
		$len_type:ty,
		$lt_cnst:ident
	) => {

		const $lt_cnst: usize = std::mem::size_of::<$len_type>();

		/// $name 
		pub fn $name(pt: Vec<u8>) -> Vec<u8> {
			// compute how many bytes we need to add in order to get be a multiple
			// of $block_len

			let bytes_for_metadata = 1 + $lt_cnst; // One byte for that 0x80, and more bytes to remember the name
			let zeroes_needed = ($block_len / 8) - (pt.len() + bytes_for_metadata) % ($block_len / 8);

			let padded_len = pt.len() + bytes_for_metadata + zeroes_needed;
			let mut padded_pt = vec![0 ; padded_len];

			// first, go ahead and copy the plaintext
			for i in 0..pt.len() {
				padded_pt[i] = pt[i];
			}

			// add the 1 bit
			padded_pt[pt.len()] = 0x80;

			// now, we write the original length, as a $len_type.
			let original_bit_len = ((pt.len() * 8) as $len_type).to_le();

			// This will be little-endian, but this is okay, since we will be reading it in reverse order.
			let bit_len_bytes: [u8 ; std::mem::size_of::<$len_type>()] = unsafe { transmute(original_bit_len) };

			for i in 0..std::mem::size_of::<$len_type>() {
				padded_pt[padded_len - 1 - i] = bit_len_bytes[i];
			}

			padded_pt
			
		}

		pub fn $unname(padded: Vec<u8>) -> Vec<u8> {
			// first, go ahead and trim off the original size.
			let padded_len = padded.len();
			let original_len_bytes: [u8 ; $lt_cnst] = padded[(padded_len - $lt_cnst)..].try_into().unwrap();

			let original_len = <$len_type>::from_be_bytes(original_len_bytes);

			// now, we find where the 0x80 byte is!
			let mut truncate_index = 0;

			for i in (0..(padded.len() - $lt_cnst)).rev() {
				if i == 0 && padded[i] != 0x80 {
					panic!("Corrupted pad: Never found 1 bit");
				}

				if padded[i] == 0x80 {
					truncate_index = i;
					break;
				} else if padded[i] != 0x00 {
					panic!("Corrupted pad: Found nonzero byte in padding");
				}
			}

			let original = &padded[..truncate_index];

			if (original.len() * 8) as $len_type  != original_len {
				panic!("Corrupted pad: Lengths do not match");
			}

			original.to_vec()

		}

		#[cfg(test)]
		mod $test_suite_name {

			use rand::Rng;
			use super::*;

			#[test]
			fn test_symmetry() {

				for _ in 0..100 {
					let mut rand_vec = vec![0u8 ; rand::thread_rng().gen_range(4..34857)];
					for i in 0..rand_vec.len() {
						rand_vec[i] = rand::thread_rng().gen();
					}

					let padded = $name(rand_vec.clone());
					let unpadded = $unname(padded);

					assert_eq!(unpadded, rand_vec);
				}

			}

			#[test]
			fn test_abc() {
				let string = "abc".as_bytes().to_vec();
				let padded = $name(string);
				
				for i in 0..padded.len() {
					print!("{:02X}", padded[i]);
					if i % 8 == 7 {
						print!(" ");
					}
				}
				println!("");
			}

		}
	};
}

make_sha_pad!(
	pad_sha512,
	unpad_sha512,
	pad_sha512_tests,
	1025,
	u128,
	U128_BYTE_LEN
);

make_sha_pad!(
	pad_sha256,
	unpad_sha256,
	pad_sha256_tests,
	512,
	u64,
	U64_BYTE_LEN
);