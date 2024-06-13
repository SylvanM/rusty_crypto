
// MARK: Types

use std::mem::transmute;

use crate::padding;

/// The words in the SHA-512 Algorithm
type Word = u64;

/// The amount of bits in a SHA-512 digest
const DIGEST_BIT_COUNT: usize = 512;

/// The amount of bytes in a SHA-512 digest
const DIGEST_BYTE_COUNT: usize = DIGEST_BIT_COUNT / 8;

/// The amount of words in a SHA-512 digest
const DIGEST_WORD_COUNT: usize = DIGEST_BYTE_COUNT / std::mem::size_of::<Word>();

/// The amount of bits in a chunk that SHA hashes at a time
const CHUNK_BIT_COUNT: usize = 1024;

/// The amount of bytes in a chunk
const CHUNK_BYTE_COUNT: usize = CHUNK_BIT_COUNT / 8;

/// The amount of words in a chunk
const CHUNK_WORD_COUNT: usize = CHUNK_BYTE_COUNT / std::mem::size_of::<Word>();

const ROUNDS: usize = 80;

/// The 512-bit hash from SHA-512, as an array of words
type Digest = [Word ; DIGEST_WORD_COUNT];

type Chunk = [Word ; CHUNK_WORD_COUNT];

// MARK: Algorithm Constants

/// Initial hash constants
const H: [Word ; DIGEST_WORD_COUNT] = [
	0x6a09e667f3bcc908, 0xbb67ae8584caa73b, 0x3c6ef372fe94f82b, 0xa54ff53a5f1d36f1, 
	0x510e527fade682d1, 0x9b05688c2b3e6c1f, 0x1f83d9abfb41bd6b, 0x5be0cd19137e2179
];

/// Round Constants
const K: [Word ; ROUNDS] = [
	0x428a2f98d728ae22, 0x7137449123ef65cd, 0xb5c0fbcfec4d3b2f, 0xe9b5dba58189dbbc, 
	0x3956c25bf348b538, 0x59f111f1b605d019, 0x923f82a4af194f9b, 0xab1c5ed5da6d8118, 
	0xd807aa98a3030242, 0x12835b0145706fbe, 0x243185be4ee4b28c, 0x550c7dc3d5ffb4e2, 
	0x72be5d74f27b896f, 0x80deb1fe3b1696b1, 0x9bdc06a725c71235, 0xc19bf174cf692694, 
	0xe49b69c19ef14ad2, 0xefbe4786384f25e3, 0x0fc19dc68b8cd5b5, 0x240ca1cc77ac9c65, 
	0x2de92c6f592b0275, 0x4a7484aa6ea6e483, 0x5cb0a9dcbd41fbd4, 0x76f988da831153b5, 
	0x983e5152ee66dfab, 0xa831c66d2db43210, 0xb00327c898fb213f, 0xbf597fc7beef0ee4, 
	0xc6e00bf33da88fc2, 0xd5a79147930aa725, 0x06ca6351e003826f, 0x142929670a0e6e70, 
	0x27b70a8546d22ffc, 0x2e1b21385c26c926, 0x4d2c6dfc5ac42aed, 0x53380d139d95b3df, 
	0x650a73548baf63de, 0x766a0abb3c77b2a8, 0x81c2c92e47edaee6, 0x92722c851482353b, 
	0xa2bfe8a14cf10364, 0xa81a664bbc423001, 0xc24b8b70d0f89791, 0xc76c51a30654be30, 
	0xd192e819d6ef5218, 0xd69906245565a910, 0xf40e35855771202a, 0x106aa07032bbd1b8, 
	0x19a4c116b8d2d0c8, 0x1e376c085141ab53, 0x2748774cdf8eeb99, 0x34b0bcb5e19b48a8, 
	0x391c0cb3c5c95a63, 0x4ed8aa4ae3418acb, 0x5b9cca4f7763e373, 0x682e6ff3d6b2b8a3, 
	0x748f82ee5defb2fc, 0x78a5636f43172f60, 0x84c87814a1f0ab72, 0x8cc702081a6439ec, 
	0x90befffa23631e28, 0xa4506cebde82bde9, 0xbef9a3f7b2c67915, 0xc67178f2e372532b, 
	0xca273eceea26619c, 0xd186b8c721c0c207, 0xeada7dd6cde0eb1e, 0xf57d4f7fee6ed178, 
	0x06f067aa72176fba, 0x0a637dc5a2c898a6, 0x113f9804bef90dae, 0x1b710b35131c471b, 
	0x28db77f523047d84, 0x32caab7b40c72493, 0x3c9ebe0a15c9bebc, 0x431d67c49c100d4c, 
	0x4cc5d4becb3e42b6, 0x597f299cfc657e2a, 0x5fcb6fab3ad6faec, 0x6c44198c4a475817
];

// MARK: Helper Functions

/// Splits an array of bytes into 1024 bit chunks. This assumes the bytes
/// array is already padded to an appropriate length.
fn bytes_to_chunks(bytes: Vec<u8>) -> Vec<Chunk> {
	let mut chunks = vec![[0 ; CHUNK_WORD_COUNT] ; bytes.len() / CHUNK_BYTE_COUNT];

	for i in 0..chunks.len() {
		// each chunk is 16 words. we need those words!
		chunks[i] = unsafe {
			let be: [u8 ; CHUNK_BYTE_COUNT] = bytes[(i * CHUNK_BYTE_COUNT)..(i * CHUNK_BYTE_COUNT + CHUNK_BYTE_COUNT)].try_into().unwrap();
			transmute(be)
		};
		
		for j in 0..CHUNK_WORD_COUNT {
			chunks[i][j] = (chunks[i][j] as u64).to_be();
		}
	}

	chunks
}

pub fn hash(bytes: Vec<u8>) -> Digest {

	let padded = padding::pad_sha512(bytes);

	// now we divide it into chunks!
	let chunks = bytes_to_chunks(padded);

	let mut hash_buffer = H;

	for chunk in chunks {
		// message schedule
		let mut w = [0u64 ; ROUNDS];

		// copy chunk into first 16 words
		for i in 0..CHUNK_WORD_COUNT {
			w[i] = chunk[i];
		}

		for i in 16..80 {
			let sum0 = w[i - 15].rotate_right(1) ^ w[i - 15].rotate_right(8) ^ (w[i-15] >> 7);
			let sum1 = w[i - 2].rotate_right(19) ^ w[i - 2].rotate_right(61) ^ (w[i-2] >> 6);
        	w[i] = w[i - 16].wrapping_add(sum0).wrapping_add(w[i - 7]).wrapping_add(sum1);
		}

		// so, this is the a b c d e f g h that we see in the literature.
		let mut local_buffer = hash_buffer;

		// compression function main loop
		for i in 0..80 {
			let sigma1 = local_buffer[4].rotate_right(14) ^ local_buffer[4].rotate_right(18) ^ local_buffer[4].rotate_right(41);
			let ch = (local_buffer[4] & local_buffer[5]) ^ ((!local_buffer[4]) & local_buffer[6]);
			let temp1 = local_buffer[7].wrapping_add(sigma1).wrapping_add(ch).wrapping_add(K[i]).wrapping_add(w[i]);
			let sigma0 = local_buffer[0].rotate_right(28) ^ local_buffer[0].rotate_right(34) ^ local_buffer[0].rotate_right(39);
			let maj = (local_buffer[0] & local_buffer[1]) ^ (local_buffer[0] & local_buffer[2]) ^ (local_buffer[1] & local_buffer[2]);
			let temp2 = sigma0.wrapping_add(maj);
	
			local_buffer[7] = local_buffer[6];
			local_buffer[6] = local_buffer[5];
			local_buffer[5] = local_buffer[4];
			local_buffer[4] = local_buffer[3].wrapping_add(temp1);
			local_buffer[3] = local_buffer[2];
			local_buffer[2] = local_buffer[1];
			local_buffer[1] = local_buffer[0];
			local_buffer[0] = temp1.wrapping_add(temp2);
		}

		for i in 0..8 {
			hash_buffer[i] = hash_buffer[i].wrapping_add(local_buffer[i]);
		}
	}

	unsafe { transmute(hash_buffer) }
	
}

#[cfg(test)]
mod tests {
    use crate::sha512::DIGEST_WORD_COUNT;

    use super::{hash, Digest};

	#[test]
	fn test_abc() {
		let string = vec![0x61, 0x62, 0x63];
		let digest = hash(string);

		let known_digest: Digest = [
			0xDDAF35A193617ABA, 0xCC417349AE204131, 0x12E6FA4E89A97EA2, 0x0A9EEEE64B55D39A, 
			0x2192992A274FC1A8, 0x36BA3C23A3FEEBBD, 0x454D4423643CE80E, 0x2A9AC94FA54CA49F
		];

		assert_eq!(digest, known_digest);
	}

	#[test]
	fn test_long_abc() {
		let string = "abcdefghbcdefghicdefghijdefghijkefghijklfghijklmghijklmnhijklmnoijklmnopjklmnopqklmnopqrlmnopqrsmnopqrstnopqrstu".as_bytes().to_vec();
		let digest = hash(string);

		let known_digest: Digest = [
			0x8E959B75DAE313DA, 0x8CF4F72814FC143F, 0x8F7779C6EB9F7FA1, 0x7299AEADB6889018,
			0x501D289E4900F7E4, 0x331B99DEC4B5433A, 0xC7D329EEB6DD2654, 0x5E96E55B874BE909
		];

		assert_eq!(digest, known_digest);
	}

}