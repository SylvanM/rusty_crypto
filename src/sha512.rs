use std::mem::transmute;

use crate::{ecdh::ED25519Obj, curve25519::{KEY_BYTE_COUNT, KEY_BN_WORD_COUNT}};

/// Implementation of SHA-512 specifically for ED25519

/// The size, in bits, of a block for SHA-512
const SHA_BLOCK_SIZE: usize = 1024;

/// The size, in bytes, of a block for SHA-512
const SHA_BLOCK_SIZE_BYTES: usize = SHA_BLOCK_SIZE / 8;

/// The size, in 64-bit words, of a block for SHA-512
const SHA_BLOCK_SIZE_WORDS: usize = SHA_BLOCK_SIZE / 64;

/// The output digest size for SHA-512, in bits
const SHA_OUTPUT_SIZE: usize = 512;

/// The output digest size for SHA-512, in bytes
const SHA_OUTPUT_SIZE_BYTES: usize = SHA_OUTPUT_SIZE / 8;

/// The output digest size for SHA-512, in 64-bit words
const SHA_OUTPUT_SIZE_WORDS: usize = SHA_OUTPUT_SIZE_BYTES / 8;

/// A chunk of input that SHA works with, in this case 1024 bits as a 
/// byte array
pub type SHABlock = [u64 ; SHA_BLOCK_SIZE_WORDS];

/// A SHA-512 digest object
pub type SHADigest = [u64 ; SHA_OUTPUT_SIZE_WORDS];

// -- Magic Constants --

/// SHA-512 Initial Hash Values
const H: [u64 ; 8] = [
	0x6a09e667f3bcc908, 0xbb67ae8584caa73b, 0x3c6ef372fe94f82b, 0xa54ff53a5f1d36f1, 
	0x510e527fade682d1, 0x9b05688c2b3e6c1f, 0x1f83d9abfb41bd6b, 0x5be0cd19137e2179
];

/// SHA-512 Round Constants
const K: [u64 ; 80] = [
	0x428a2f98d728ae22, 0x7137449123ef65cd, 0xb5c0fbcfec4d3b2f, 0xe9b5dba58189dbbc, 0x3956c25bf348b538, 
	0x59f111f1b605d019, 0x923f82a4af194f9b, 0xab1c5ed5da6d8118, 0xd807aa98a3030242, 0x12835b0145706fbe, 
	0x243185be4ee4b28c, 0x550c7dc3d5ffb4e2, 0x72be5d74f27b896f, 0x80deb1fe3b1696b1, 0x9bdc06a725c71235, 
	0xc19bf174cf692694, 0xe49b69c19ef14ad2, 0xefbe4786384f25e3, 0x0fc19dc68b8cd5b5, 0x240ca1cc77ac9c65, 
	0x2de92c6f592b0275, 0x4a7484aa6ea6e483, 0x5cb0a9dcbd41fbd4, 0x76f988da831153b5, 0x983e5152ee66dfab, 
	0xa831c66d2db43210, 0xb00327c898fb213f, 0xbf597fc7beef0ee4, 0xc6e00bf33da88fc2, 0xd5a79147930aa725, 
	0x06ca6351e003826f, 0x142929670a0e6e70, 0x27b70a8546d22ffc, 0x2e1b21385c26c926, 0x4d2c6dfc5ac42aed, 
	0x53380d139d95b3df, 0x650a73548baf63de, 0x766a0abb3c77b2a8, 0x81c2c92e47edaee6, 0x92722c851482353b, 
	0xa2bfe8a14cf10364, 0xa81a664bbc423001, 0xc24b8b70d0f89791, 0xc76c51a30654be30, 0xd192e819d6ef5218, 
	0xd69906245565a910, 0xf40e35855771202a, 0x106aa07032bbd1b8, 0x19a4c116b8d2d0c8, 0x1e376c085141ab53, 
	0x2748774cdf8eeb99, 0x34b0bcb5e19b48a8, 0x391c0cb3c5c95a63, 0x4ed8aa4ae3418acb, 0x5b9cca4f7763e373, 
	0x682e6ff3d6b2b8a3, 0x748f82ee5defb2fc, 0x78a5636f43172f60, 0x84c87814a1f0ab72, 0x8cc702081a6439ec, 
	0x90befffa23631e28, 0xa4506cebde82bde9, 0xbef9a3f7b2c67915, 0xc67178f2e372532b, 0xca273eceea26619c, 
	0xd186b8c721c0c207, 0xeada7dd6cde0eb1e, 0xf57d4f7fee6ed178, 0x06f067aa72176fba, 0x0a637dc5a2c898a6, 
	0x113f9804bef90dae, 0x1b710b35131c471b, 0x28db77f523047d84, 0x32caab7b40c72493, 0x3c9ebe0a15c9bebc, 
	0x431d67c49c100d4c, 0x4cc5d4becb3e42b6, 0x597f299cfc657e2a, 0x5fcb6fab3ad6faec, 0x6c44198c4a475817
];

// -- SHA Algorithm --

const fn shr(x: u64, n: u32) -> u64 {
	x.checked_shl(n).unwrap_or(0)
}

const fn rotr(x: u64, n: u32) -> u64 {
	shr(x, n).overflowing_add(shr(x, 64 - n)).0
}

const fn sig1(x: u64) -> u64 {
	shr(x, 6).overflowing_add(rotr(x, 61).overflowing_add(rotr(x, 19)).0).0
}

const fn sigt(x: u64) -> u64 {
	shr(x, 7).overflowing_add(rotr(x, 8).overflowing_add(rotr(x, 1)).0).0
}

/// Hashes a single chunk of input into a BigInt
pub fn hash_block(previous_digest: SHADigest, block: SHABlock) -> SHADigest {

	let mut w = [0 ; 80];
	let mut h = previous_digest;

	for i in 0..SHA_BLOCK_SIZE_WORDS {
		w[i] = block[i];
	}

	for i in SHA_BLOCK_SIZE_WORDS..80 {
		w[i] = ((sig1(w[i - 1]).overflowing_add(w[i - 7]).0)
				.overflowing_add(sigt(w[i - 15])).0)
				.overflowing_add(w[i - 16]).0;
	}

	for i in 0..80 {
		let ma = (h[0] & h[1]) ^ (h[0] & h[2]) ^ (h[1] & h[2]);
		let mut ch = (h[4] & h[5]) ^ ((!h[4]) ^ h[6]);
		let mut sa = (h[0] >> 2) ^ (h[0] >> 13) ^ (h[0] >> 22);
		sa = sa.overflowing_add(ma).0;
		ch = ch.overflowing_add(h[7]).0;
		let se = (h[4] >> 6) ^ (h[4] >> 11) ^ (h[4] >> 25);
		ch = ch.overflowing_add(se).0;
		ch = ch.overflowing_add(w[i]).0;
		ch = ch.overflowing_add(K[i]).0;
		let ds = ch.overflowing_add(h[3]).0;
		sa = sa.overflowing_add(ch).0;
		h[7] = h[6];
		h[6] = h[5];
		h[5] = h[4];
		h[4] = ds;
		h[3] = h[2];
		h[2] = h[1];
		h[1] = h[0];
		h[0] = sa;
	}

	h
}

/// Performs pre-processing on an `ED25519Obj` to convert it to a 
/// `SHAChunk`
pub fn format(obj: ED25519Obj) -> SHABlock {
	let mut block = [0 ; SHA_BLOCK_SIZE_BYTES];

	for i in 0..KEY_BYTE_COUNT {
		block[i] = obj[i]
	}

	// place the padding `1` where it needs to be, the zeros are already 
	// in place by default
	block[KEY_BYTE_COUNT] = 1 << 7;

	// the message size is 256 bits! that number can't fit in one byte,
	// so we'll put a little 1 in the second to last byte.
	// this may seem like I'm over-explaining, but seeing this line
	// out of context may look a bit weird, so I'm more putting this in
	// for my own sake.
	block[SHA_BLOCK_SIZE_BYTES - 2] = 1;

	unsafe { 
		transmute::<[u8 ; SHA_BLOCK_SIZE_BYTES], SHABlock>(block)
	}
}

/// Performs SHA-512 on a secret, returning the lower 512 bits of 
/// the hash 
pub fn ecdh_key_hash(obj: ED25519Obj) -> ED25519Obj {
	let digest = &hash_block(H, format(obj));
	let mut lower_digest = [0 ; KEY_BN_WORD_COUNT];

	for i in 0..KEY_BN_WORD_COUNT {
		lower_digest[i] = digest[i];
	}
	
	unsafe {
		transmute::<[u64 ; KEY_BN_WORD_COUNT], ED25519Obj>(lower_digest)
	}
}