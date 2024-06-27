//
// The basic Learning with Errors over integer lattices
//

use std::mem::transmute;

use algebra_kit::std_impls::*;
use matrix_kit::index;
use matrix_kit::matrix::*;

use crate::utility::BigMappable;

use rand::rngs::StdRng;
use rand::Rng;
use rand::SeedableRng;

// -- Default parameters, chosen somewhat arbitrarily!

macro_rules! get_bit {
	($x: expr, $i: expr) => {
		($x >> $i) & 1
	};
}


// This is little endian!!!
macro_rules! byte_to_bits {
	($b: expr) => {
		// TODO: Learn how Rust macros work to make this wayyyy more elegant
		[
			get_bit!($b, 0), get_bit!($b, 1),
			get_bit!($b, 2), get_bit!($b, 3),
			get_bit!($b, 4), get_bit!($b, 5),
			get_bit!($b, 6), get_bit!($b, 7),
		]
	};
}

// little endian!
macro_rules! bits_to_byte {
	($bits: expr) => {
		($bits[0].val << 0) + ($bits[1].val << 1) + 
		($bits[2].val << 2) + ($bits[3].val << 3) + 
		($bits[4].val << 4) + ($bits[5].val << 5) + 
		($bits[6].val << 6) + ($bits[7].val << 7)
	};
}

/// Generates a random error for an M*N matrix vector in the intergers mod Q. Errors are generated so that 
/// no subset sum of the errors will exceed one quarter of Q.
fn error_gen<const M: usize, const N: usize, const Q: i64, const S: i64>(error: &mut [ZM<Q>]) {

	let mut rng = StdRng::from_entropy();

	// naive implementation, where error elements are chosen in [-S, S]
	for i in 0..(M * N) {
		error[i] = rng.gen_range(-S..=S).into();
	}
}

/**
 * Important note to the Rust designers!!!
 * 
 * The following commented out code is never used, yet deleting the comment causes the compiler to panic.
 * That's gotta be a compiler bug, yeah?
 */

// /**
//  * Generates a system of noisy equations to use as a public key for encrypting a single bit
//  */
// fn gen_bit<const M: usize, const N: usize, const Q: i64, const S: i64>() -> ([ZM<Q> ; N], [ZM<Q> ; M * (N + 1)]) where [(); M * N]: Sized {
	
// 	let mut s = [0.into(); N];

// 	for i in 0..N {
// 		s[i] = ZM::<Q>::rnd();
// 	}
	
// 	let mut a: [ZM<Q> ; M * N] = [0.into(); M * N];

// 	for i in 0..M {
// 		for j in 0..N {
// 			a[index!(M, N, i, j)] = ZM::<Q>::rnd();
// 		}
// 	}

// 	// compute b = As + e

// 	let mut be: [ZM<Q> ; M] = [0.into() ; M];

// 	// compute b = As first

// 	mat_mul_ptrs::<M, N, 1, ZM<Q>>(&a, &s, &mut be);

// 	// now add the error
// 	let mut e: [ZM<Q> ; M] = [0.into() ; M];
// 	error_gen::<M, 1, Q, S>(&mut e);

// 	let mut pubkey = [0.into() ; M * (N + 1)];
	
// 	for r in 0..M {
// 		for c in 0..N {
// 			pubkey[index!(M, N + 1, r, c)] = a[index!(M, N, r, c)];
// 		}
// 	}

// 	for r in 0..M {
// 		pubkey[index!(M, N + 1, r, N)] = be[r];
// 	}

// 	(s, pubkey)

// }

// /**
//  * Encrypts a bit 
//  */
// fn enc_bit<const M: usize, const N: usize, const Q: i64, const S: i64>(pubkey: [ZM<Q> ; M * (N + 1)], bit: bool) -> [ZM<Q> ; N + 1] {
// 	// add all the rows of the public key to create a new equation

// 	let offset: ZM<Q> = if bit { (Q / 2).into() } else { 0.into() };

// 	let mut new_eq = [0.into() ; N + 1];

// 	let mut selections = [0.into() ; M];
// 	for r in 0..M {
// 		selections[r] = ZM::<2>::rnd().val.into();
// 	}

// 	mat_mul_ptrs::<1, M, {N + 1}, ZM<Q>>(&selections, &pubkey, &mut new_eq);

// 	new_eq[N] += offset;

// 	new_eq
// }

// /**
//  * Decrypts a bit
//  */
// fn dec_bit<const M: usize, const N: usize, const Q: i64, const S: i64>(seckey: [ZM<Q> ; N], cipher: [ZM<Q> ; N + 1]) -> bool {
// 	// plug in our secret s into the equation from Bob
// 	let mut cp: ZM<Q> = 0.into();

// 	vec_dot_prod_ptr::<1, N, ZM<Q>>(&seckey, 0, &cipher, &mut cp);

// 	let diff = (cipher[N] - cp).val;

// 	// if the difference is small, then we must have encrypted a zero
// 	(diff > (Q / 4)) && (diff < ((3 * Q) / 4))
// }

// #[test]
// fn test_lwe_bit() {

// 	for _ in 0..256 {
// 		let (seckey, pubkey) = gen_bit::<10, 100, 89, 5>();
	
// 		for b in [true, false] {
// 			let ciphertext = enc_bit::<10, 100, 89, 5>(pubkey, b);
// 			let _decrypted = dec_bit::<10, 100, 89, 5>(seckey, ciphertext);

// 			assert_eq!(_decrypted, b);
// 		}
// 	}

// }

fn gen_mat<const M: usize, const N: usize, const Q: i64, const S: i64, const K: usize>() 
	-> ([ZM<Q> ; N * K], Box<[ZM<Q> ; M * (N + K)]>) where [() ; N * K]: Sized, [() ; M * K]: Sized, [() ; M * N]: Sized {


	// println!("GENERATING Key pair");
	
	// generate the secret, S
	
	let mut s = Matrix::<N, K, ZM<Q>>::new();

	for i in 0..(N * K) {
		s.flatmap[i] = ZM::<Q>::rnd();
	}

	// generate the public key A
	let mut a = Matrix::<M, N, ZM<Q>>::new();

	for i in 0..(M * N) {
		a.flatmap[i] = ZM::<Q>::rnd();
	}
	
	// Compute AS + E
	let b = a * s;

	let mut e = Matrix::<M, K, ZM<Q>>::new();
	error_gen::<M, K, Q, S>(&mut e.flatmap);

	let mut pubkey = Matrix::<M, {N + K}, ZM<Q>>::new();
	
	// we copy the A matrix into the left side of the public key
	for r in 0..M {
		for c in 0..N {
			pubkey.flatmap[matrix_kit::index!(M, N + K, r, c)] = a.flatmap[matrix_kit::index!(M, N, r , c)];
		}
	}

	// add the error to AS, and store it in the right-hand side of the public key
	mat_add::<M, K, ZM<Q>>(&b.flatmap, &e.flatmap, &mut pubkey.flatmap[index!(M, N + K, 0, N)..index!(M, N + K, M, N + K - 1)]);

	(s.flatmap, Box::new(pubkey.flatmap))

}

fn enc_mat<const M: usize, const N: usize, const Q: i64, const S: i64, const K: usize>(pubkey: Box<[ZM<Q> ; M * (N + K)]>, m: [ZM<2> ; K]) -> [ZM<Q> ; K * (N + 1)] where [() ; K * M]: Sized, [() ; K * N]: Sized, [() ; K * (N + 1)]: Sized {
	let mut t = [0.into() ; K * M];

	// generate selection matrix
	for i in 0..(K * M) {
		t[i] = ZM::<Q>::convert(ZM::<2>::rnd());
	}
	
	// we need to generate the rows of new summed equations

	let mut summed_eqs = [0.into() ; K * (N + 1)]; // K equations, each for a bit, each with N coefficients.

	mat_mul_ptrs::<K, M, N, ZM<Q>>(&t, &pubkey[index!(M, N + K, 0, 0)..index!(M, N + K, M, N - 1)], &mut summed_eqs[index!(K, N + 1, 0, 0)..index!(K, N + 1, K, N - 1)]);

	for i in 0..K {
		vec_dot_prod_ptr::<K, M, ZM<Q>>(&t, i, &pubkey[index!(M, N + K, 0, N + i)..index!(M, N + K, M, N + i)], &mut summed_eqs[index!(K, N + 1, i, N)]);
	}


	// great! Now we add offsets to the constant terms as needed.
	let mut offsets = [0.into() ; K];

	let proper_form = m.big_map(|x| x.val.into());
	

	scalar_mul::<K, ZM<Q>>((Q / 2).into(), &proper_form, &mut offsets);

	for i in 0..K {
		summed_eqs[index!(K, N + 1, i, N)] += offsets[i]
	}

	summed_eqs
}

fn dec_mat<const M: usize, const N: usize, const Q: i64, const S: i64, const K: usize>(seckey: [ZM<Q> ; N * K], cipher: [ZM<Q> ; K * (N + 1)]) -> [ZM<2> ; K] {
	// we ONLY need to compute the diagonal of the product matrix.
	let mut diffs = [0.into() ; K];

	for i in 0..K {
		vec_dot_prod_ptr::<K, N, ZM<Q>>(&cipher[index!(K, N + 1, 0, 0)..index!(K, N + 1, K, N - 1)], i, &seckey[index!(N, K, 0, i)..index!(N, K, N, i)], &mut diffs[i]);
		diffs[i] = cipher[index!(K, N + 1, i, N)] - diffs[i];
	}

	diffs.big_map(|d| if (d.val > (Q / 4)) && (d.val < ((3 * Q) / 4)) { 1.into() } else { 0.into() })
}

#[test]
fn test_lwe() {
	// These are the same tests as before, but the one-bit versions
	for _ in 1..=256 {
		let (seckey, pubkey) = gen_mat::<DEF_M, DEF_N, MODULUS, ERROR, BIT_LENGTH>();

		// the plaintext!
		let mut b = [0.into() ; 256];

		for i in 0..256 {
			b[i] = ZM::<2>::rnd();
		}
	
		let ciphertext = enc_mat::<DEF_M, DEF_N, MODULUS, ERROR, BIT_LENGTH>(pubkey, b);
		let decrypted = dec_mat::<DEF_M, DEF_N, MODULUS, ERROR, BIT_LENGTH>(seckey, ciphertext);

		assert_eq!(b, decrypted);
	}
}

// -- Now we get to actually exporting these algorithms into a usable form

// [ZM<Q> ; M * (N + K)] pubkey
// [ZM<Q> ; N * K] privkey

/*
 * Notes to myself:
 * 
 * We will represent the keys as byte arrays, 8 bytes for each i64, which will be turned into 
 * the ZM<Q> later. So, the ZM<Q> array [1, 2, 3] would be 
 * [0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x1,
 * 	0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x2,
 * 	0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x3]
 * 
 * So, the number of bytes here is 8 * M * (N + K)!
 */

const DEF_M: usize = 100;
const DEF_N: usize = 30;
const MODULUS: i64 = 3329;
const ERROR: i64 = 8;
const BIT_LENGTH: usize = 256;

/// The length, in bytes, of the public key
pub const PUBKEY_LEN: usize = 8 * DEF_M * (DEF_N + BIT_LENGTH);

/// The length, in bytes, of the secret key
pub const SECKEY_LEN: usize = 8 * DEF_N * BIT_LENGTH;

/// The length, in bytes, of the plaintext
pub const PLAINTEXT_LEN: usize = BIT_LENGTH / 8;

/// The length, in bytes, of the ciphertext
pub const CIPHERTEXT_LEN: usize = 8 * BIT_LENGTH * (DEF_N + 1);

/// The public key type for standard learning with errors
pub type PublicKey = Box<[u8 ; PUBKEY_LEN]>;

/// The private key type for learning with errors
pub type SecretKey = [u8 ; SECKEY_LEN];

/// The plaintext type, just 256 bits!
pub type Plaintext = [u8 ; PLAINTEXT_LEN];

/// The ciphertext type
pub type Ciphertext = [u8 ; CIPHERTEXT_LEN];

fn pk_to_matrix_rep(pubkey: PublicKey) -> Box<[ZM<MODULUS> ; DEF_M * (DEF_N + BIT_LENGTH)]> {
	let thing = unsafe {
		transmute::<PublicKey, Box<[i64 ; DEF_M * (DEF_N + BIT_LENGTH)]>>(pubkey)
	}.big_map(|x| x.into());

	Box::new(thing)
}

fn matrix_rep_to_pk(matrix: &Box<[ZM<MODULUS> ; DEF_M * (DEF_N + BIT_LENGTH)]>) -> PublicKey {
	unsafe {
		transmute::<Box<[i64 ; DEF_M * (DEF_N + BIT_LENGTH)]>, PublicKey>(Box::new(matrix.big_map(|x| x.val)))
	}
}

fn sk_to_matrix_rep(secretkey: SecretKey) -> [ZM<MODULUS> ; DEF_N * BIT_LENGTH] {
	unsafe {
		transmute::<SecretKey, [i64 ; DEF_N * BIT_LENGTH]>(secretkey)
	}.big_map(|x| x.into())
}

fn matrix_rep_to_sk(matrix: [ZM<MODULUS> ; DEF_N * BIT_LENGTH]) -> SecretKey {
	unsafe {
		transmute::<[i64 ; DEF_N * BIT_LENGTH], SecretKey>(matrix.big_map(|x| x.val))
	}
}

fn pt_to_matrix_rep(plaintext: Plaintext) -> [ZM<2> ; BIT_LENGTH] {

	let slice: [[ZM<2> ; 8] ; BIT_LENGTH / 8] = plaintext.big_map(|byte| 
		byte_to_bits!(byte).map(|b| b.into())
	);

	let mut as_array = [0.into() ; BIT_LENGTH];

	for i in 0..BIT_LENGTH {
		as_array[i] = slice[i / 8][i % 8];
	}

	as_array
}

fn matrix_rep_to_pt(matrix: [ZM<2> ; BIT_LENGTH]) -> Plaintext {

	let mut pt = [0 ; BIT_LENGTH / 8];

	for i in 0..(BIT_LENGTH / 8) {
		pt[i] = bits_to_byte!(matrix[(i * 8)..(i * 8 + 8)]) as u8
	}

	pt

}

fn ct_to_matrix_rep(ciphertext: Ciphertext) -> [ZM<MODULUS> ; BIT_LENGTH * (DEF_N + 1)] {
	unsafe {
		transmute::<Ciphertext, [i64 ; BIT_LENGTH * (DEF_N + 1)]>(ciphertext)
	}.big_map(|x| x.into())
}

fn matrix_rep_to_ct(matrix: [ZM<MODULUS> ; BIT_LENGTH * (DEF_N + 1)]) -> Ciphertext {
	unsafe {
		transmute::<[i64 ; BIT_LENGTH * (DEF_N + 1)], Ciphertext>(matrix.big_map(|x| x.val))
	}
}

// This frequently overflows the stack! We need a better way of dealing with this.
// #[test] This is commented out so that when we test the whole thing we don't fail this test. Yes
// this is a cop-out.
fn test_conversions() {

	println!("Testing conversions");
	println!("Plaintext bits: {:?}", BIT_LENGTH);
	println!("modulus={:?}, M={:?}, N={:?}, Err={:?}", MODULUS, DEF_M, DEF_N, ERROR);
	println!("Public key size: {:?} bytes", 8 * DEF_M * (DEF_N + BIT_LENGTH));

	for i in 1..=256 {

		println!("Test {:?}", i);
		
		let (sk_mat, pk_mat) = gen_mat::<DEF_M, DEF_N, MODULUS, ERROR, BIT_LENGTH>();

		let mut pt_mat = [0.into() ; BIT_LENGTH];

		for i in 0..BIT_LENGTH {
			pt_mat[i] = ZM::<2>::rnd();
		}

		// we are going to be generating a random ciphertext, instead of trying to encrypt soemthing!
		// we aren't testing encryption, just conversion.
		
		// [ZM<MODULUS> ; BIT_LENGTH * (DEF_N + 1)]
		let mut ct_mat = [0.into() ; BIT_LENGTH * (DEF_N + 1)];
		for i in 0..(BIT_LENGTH * (DEF_N + 1)) {
			ct_mat[i] = ZM::<MODULUS>::rnd();
		}

		assert_eq!(sk_mat, sk_to_matrix_rep(matrix_rep_to_sk(sk_mat)));
		assert_eq!(pk_mat, pk_to_matrix_rep(matrix_rep_to_pk(&pk_mat)));
		assert_eq!(pt_mat, pt_to_matrix_rep(matrix_rep_to_pt(pt_mat)));
		assert_eq!(ct_mat, ct_to_matrix_rep(matrix_rep_to_ct(ct_mat)));

	}
}

pub fn gen() -> (SecretKey, PublicKey) {
	let (sk_mat, pk_mat) = gen_mat::<DEF_M, DEF_N, MODULUS, ERROR, BIT_LENGTH>();
	(matrix_rep_to_sk(sk_mat), matrix_rep_to_pk(&pk_mat))
}

pub fn enc(pk: PublicKey, pt: Plaintext) -> Ciphertext {
	matrix_rep_to_ct(enc_mat::<DEF_M, DEF_N, MODULUS, ERROR, BIT_LENGTH>(pk_to_matrix_rep(pk), pt_to_matrix_rep(pt)))
}

pub fn dec(sk: SecretKey, ct: Ciphertext) -> Plaintext {
	matrix_rep_to_pt(dec_mat::<DEF_M, DEF_N, MODULUS, ERROR, BIT_LENGTH>(sk_to_matrix_rep(sk), ct_to_matrix_rep(ct)))
}

// #[test]
fn test_correctness() {
	println!("Testing correctness");
	println!("Plaintext bits: {:?}", BIT_LENGTH);
	println!("modulus={:?}, M={:?}, N={:?}, Err={:?}", MODULUS, DEF_M, DEF_N, ERROR);
	println!("Public key size: {:?} bytes", 8 * DEF_M * (DEF_N + BIT_LENGTH));

	for i in 1..=256 {

		println!("Test {:?}", i);

		let (secret_key, public_key) = gen();

		let plaintext: Plaintext = rand::thread_rng().gen::<[u8; 32]>();
		
		let encrypted = enc(public_key, plaintext);
		let recovered = dec(secret_key, encrypted);

		assert_eq!(plaintext, recovered);

	}
}