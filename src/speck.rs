use std::mem::transmute;

///
/// The Speck algorithm designed by the NSA, with a word size of 64 bits, and a 
/// key size of 4 words, so 256 bits.
/// 
/// Speck128/256
///

type Word = u64;
type Block = [Word ; 2];
type Key = [Word ; 4];

const ROUNDS: usize = 34;

/// The size, in bytes, of a block to encrypt
pub const BLOCK_SIZE: usize = 16;

/// The size, in bytes, of a key
pub const KEY_SIZE: usize = 32;

/// 
/// Speck encryption scheme
/// 

pub fn enc(pt: [u8 ; BLOCK_SIZE], k: [u8 ; KEY_SIZE]) -> [u8 ; BLOCK_SIZE] {
	let block = bytes_to_block(pt);
	let key = key_bytes_to_key(k);
	let ct = speck128256_enc(key, block);
	block_to_bytes(ct)
}

pub fn dec(ct: [u8 ; BLOCK_SIZE], k: [u8 ; KEY_SIZE]) -> [u8 ; BLOCK_SIZE] {
	let block = bytes_to_block(ct);
	let key = key_bytes_to_key(k);
	let pt = speck128256_dec(key, block);
	block_to_bytes(pt)
}

/// 
/// Utility Functions
/// 

fn bytes_to_block(bytes: [u8 ; 16]) -> Block {
	let mut rev_block: Block = unsafe { transmute(bytes) };
	rev_block.reverse();
	rev_block
}

fn key_bytes_to_key(bytes: [u8 ; 32]) -> Key {
	unsafe {
		transmute(bytes)
	}
}

fn block_to_bytes(block: Block) -> [u8 ; 16] {
	let mut rev_block = block;
	rev_block.reverse();

	unsafe {
		transmute(rev_block)
	}
}

///
/// Speck actual encryption methods
/// 

const fn speck128256_round(x: Block, round_key: u64) -> Block {
	let rotated_x = x[0].rotate_right(8);
	let added = rotated_x.wrapping_add(x[1]);
	let xored = added ^ round_key;
	[xored, x[1].rotate_left(3) ^ xored]
}

const fn speck128256_round_inv(x: Block, round_key: u64) -> Block {
	let y = (x[0] ^ x[1]).rotate_right(3);
	let x = (x[0] ^ round_key).wrapping_sub(y).rotate_left(8);
	[x, y]
}
 
fn speck128256_enc(key: Key, plaintext: Block) -> Block {

	let keys = speck128256_key_schedule(key);
	let mut ciphertext = plaintext;

	for i in 0..ROUNDS {
		ciphertext = speck128256_round(ciphertext, keys[i]);
	}

	ciphertext
}

fn speck128256_dec(key: Key, ciphertext: Block) -> Block {

	let keys = speck128256_key_schedule(key);
	let mut plaintext = ciphertext;

	for i in (0..ROUNDS).rev() {
		plaintext = speck128256_round_inv(plaintext, keys[i]);
	}

	plaintext
}

fn speck128256_key_schedule(key: Key) -> [Word ; ROUNDS] {
	let mut keys = [0; ROUNDS];
	let mut constants = key;
	let mut i = 0;

	while i < 33 {
		keys[i] = constants[0];

		let mut round_map = speck128256_round([constants[1], constants[0]], i as u64);
		constants[1] = round_map[0];
		constants[0] = round_map[1];

		keys[i + 1] = constants[0];

		round_map = speck128256_round([constants[2], constants[0]], (i + 1) as u64);
		constants[2] = round_map[0];
		constants[0] = round_map[1];

		keys[i + 2] = constants[0];

		round_map = speck128256_round([constants[3], constants[0]], (i + 2) as u64);
		constants[3] = round_map[0];
		constants[0] = round_map[1];

		i += 3;
	}

	keys[33] = constants[0];

	keys
}


///
/// Tests for the Speck block cipher
/// 
/// These are gathered from https://nsacyber.github.io/simon-speck/implementations/ImplementationGuide1.1.pdf
/// ^^that paper
/// 
#[cfg(test)]
mod tests {
	use super::*;

	const PT_BYTES: [u8 ; 16] = [
		0x70, 0x6f, 0x6f, 0x6e, 0x65, 0x72, 0x2e, 0x20, 0x49, 0x6e, 0x20, 0x74, 0x68, 0x6f, 0x73, 0x65
	];

	const PT_WORDS: Block = [ 0x65736f6874206e49, 0x202e72656e6f6f70 ];

	const K_BYTES: [u8 ; 32] = [
		0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 
		0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 
		0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17,
		0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f
	];

	const K_WORDS: Key = [
		0x0706050403020100, 0x0f0e0d0c0b0a0908, 0x1716151413121110, 0x1f1e1d1c1b1a1918
	];

	const KNOWN_KEY_SCHEDULE: [u64 ; ROUNDS] = [
		0x0706050403020100, 0x37253b31171d0309, 0xfe1588ce93d80d52, 0xe698e09f31334dfe,
		0xdb60f14bcbd834fd, 0x2dafa7c34cc2c2f8, 0xfbb8e2705e64a1db, 0xdb6f99e4e383eaef,
		0x291a8d359c8ab92d, 0x0b653abee296e282, 0x604236be5c109d7f, 0xb62528f28e15d89c,
		0x10419dd1d0b25f29, 0xfd71e73b9c69fff6, 0x8ea922047f976e93, 0x2e039afd398cffbc,
		0x9c9fcfef22c1072c, 0x25fa8973ed55e6c9, 0x69819861a6b4280c, 0x7b62d87498038f77,
		0xf2351ece62e296fe, 0xa6d382d176ba05ff, 0x8d96e66745b78726, 0xbe77397e9de6bf31,
		0x35177f07af7d9479, 0xb86971c5e7815ff0, 0x7d77bfff103b45ea, 0x9983914c82a1a11e,
		0x1e88e9b26e3307f5, 0x7a0068774fc7061b, 0x1771e55c7df2b16f, 0xa2cb5323bbf86418,
		0x400303547ff5e38b, 0xf4d26f589a56b276
	];

	const KNOWN_ROUND_RESULTS: [Block ; ROUNDS] = [
		[0x6e95e0d0d5e18ede, 0x6fe673fba69af55f], [0x797032ed606dd5e4, 0x0643ad3054ba7f1f],
		[0x14a895add1c2e1a6, 0x26b5fc2f7411195e], [0x2a52445a10d191c1, 0x1ffda521b0595b30],
		[0x3a47062dc1b2183c, 0xc5aa2f204378c1bc], [0x2c4bd1e53df8b12c, 0x011aa8e7263ebcca],
		[0xd6fe16c9551814a0, 0xde2b51f064edf2f0], [0xa46dc9e3cdc0e1eb, 0x55374660eaaf766d],
		[0x69c1391f52f78e63, 0xc07b0a18078c3d09], [0x2881f1efc449d615, 0x2b59a12ff8283e5b],
		[0x20c0159fbbfc154e, 0x7a0d1ce07abde797], [0x7e08f404946c3b30, 0xae6013074183078b],
		[0xce9f862a96a52cef, 0xbd9f1e109abd10b2], [0x501c5aad593a4a28, 0xbce4aa298cd2cfbd],
		[0x6b9de48045bb6494, 0x8cb8b5cc232d1979], [0x0f27c94d9afe2b61, 0x6ae2672c8396e0ad],
		[0x576e411af3f0d9f4, 0x007d787eef47dc9f], [0xd12e6fb3e76e2bb1, 0xd2c5ac449d50cf49],
		[0xed1742d5f78c1578, 0x7b3a20f11d0a6f36], [0x8f45e0476b02743c, 0x5694e7cf83510d8f],
		[0x61113361a85e86fd, 0xd5b60d1db2d6ea87], [0x75c49c8062c54cf2, 0xd874f46df47218cc],
		[0x477c5f6d3163593e, 0x84dbfc0292f39f58], [0x7d54411c9dc3bd80, 0x5b8ba1080a5f4744],
		[0xe91f8a4e89809f78, 0x3542820edb7aa55a], [0x1642d05ccd857a09, 0xbc56c02a165050d8],
		[0xb81abd05632693b8, 0x5aacbc55d1a4157d], [0x8ae7465e55a69d0e, 0x5f82a4f0d88636e4],
		[0x7085658558e8da74, 0x8c9042039cd96d56], [0x7b00af1e6df5502b, 0x1f82bf028b3e3a9f],
		[0x5d8c5aedd45e9e80, 0xa199a2f98daf4a78], [0x833c7c77c07bcd0e, 0x8ff16bbbad019ecb],
		[0xde77ab6c5b37f913, 0xa1fcf6b1333b0f4f], [0x4109010405c0f53e, 0x4eeeb48d9c188f43]
	];

	#[test]
	fn test_bytes_to_words() {
		assert_eq!(bytes_to_block(PT_BYTES), PT_WORDS);
		assert_eq!(key_bytes_to_key(K_BYTES), K_WORDS);
	}

	#[test]
	fn test_words_to_bytes() {
		assert_eq!(block_to_bytes(PT_WORDS), PT_BYTES);
	}

	#[test]
	fn test_rounds() {
		// Make sure that each round of encryption yields the correct result
		let mut pt = PT_WORDS;

		for i in 0..ROUNDS {
			pt = speck128256_round(pt, KNOWN_KEY_SCHEDULE[i]);
			assert_eq!(pt, KNOWN_ROUND_RESULTS[i]);
		}
	}

	#[test]
	fn test_inv_rounds() {

		for i in (1..ROUNDS).rev() {
			let round_pt = speck128256_round_inv(KNOWN_ROUND_RESULTS[i], KNOWN_KEY_SCHEDULE[i]);
			assert_eq!(round_pt, KNOWN_ROUND_RESULTS[i - 1]);
		}

		let pt = speck128256_round_inv(KNOWN_ROUND_RESULTS[0], KNOWN_KEY_SCHEDULE[0]);
		assert_eq!(pt, PT_WORDS);
	}

	#[test]
	fn test_key_schedule() {
		assert_eq!(speck128256_key_schedule(K_WORDS), KNOWN_KEY_SCHEDULE)
	}

	#[test]
	fn test_speck_enc() {
		assert_eq!(speck128256_enc(K_WORDS, PT_WORDS), KNOWN_ROUND_RESULTS[ROUNDS - 1]);
	}

	#[test]
	fn test_speck_dec() {
		assert_eq!(speck128256_dec(K_WORDS, KNOWN_ROUND_RESULTS[ROUNDS - 1]), PT_WORDS);
	}
}