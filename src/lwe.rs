//
// The basic Learning with Errors over integer lattices
//

use crate::algebra::*;

use rand::{Rng, seq::index};

macro_rules! index {
	($m: expr, $n: expr, $r: expr, $c: expr) => {
		$c * $m + $r
	};
}

/// Computes the dot product between the r-th row of the matrix x, and the vector y.
/// 
/// * `M` - 
fn vec_dot_prod_ptr<const M: usize, const N: usize, const Q: i64>(x: &[ZM<Q>], r: usize, y: &[ZM<Q>], out: &mut ZM<Q>) {
	*out = 0.into();
	for i in 0..N {
		*out += x[index!(M, N, r, i)] * y[i]
	}
}

fn mat_vec_mul_ptr<const M: usize, const N: usize, const Q: i64>(a: &[ZM<Q>], vec: &[ZM<Q>], to_vec: &mut [ZM<Q>]) {
	for r in 0..M {
		vec_dot_prod_ptr::<M, N, Q>(a, r, vec, &mut to_vec[r]);
	}
}

fn mat_add<const M: usize, const N: usize, const Q: i64>(a: &[ZM<Q>], b: &[ZM<Q>], out: &mut [ZM<Q>]) {
	for i in 0..(M * N) {
		out[i] = a[i] + b[i];
	}
}

fn scalar_mul<const N: usize, const Q: i64>(k: ZM<Q>, v: &[ZM<Q>], out: &mut [ZM<Q>]) {
	for i in 0..N {
		out[i] = k * v[i];
	}
}

/// Generates a random error for an M*N matrix vector in the intergers mod Q. Errors are generated so that 
/// no subset sum of the errors will exceed one quarter of Q.
fn error_gen<const M: usize, const N: usize, const Q: i64, const S: i64>(error: &mut [ZM<Q>]) {
	// naive implementation, where error elements are chosen in [-S, S]
	for i in 0..(M * N) {
		error[i] = rand::thread_rng().gen_range(-S..=S).into();
	}
}

#[test]
fn test_mat_vec_mul_ptr() {
	// test the super simple identity!
	let identity = [1.into(), 0.into(), 0.into(), 1.into()];
	let simple_vector = [3.into(), 7.into()];
	let mut out_vector = [0.into() ; 2];
	mat_vec_mul_ptr::<2, 2, 11>(&identity, &simple_vector, &mut out_vector);

	assert_eq!(out_vector, simple_vector);

	let mat = [3.into(), 2.into(), 5.into(), 1.into(), 7.into(), 0.into()];
	let vec = [1.into(), 4.into(), 9.into()];
	let mut out = [0.into() ; 2];
	mat_vec_mul_ptr::<2, 3, 11>(&mat, &vec, &mut out);

	assert_eq!(out, [9.into(), 6.into()]);

}

fn mat_mul_ptrs<const M: usize, const K: usize, const N: usize, const Q: i64>(a: &[ZM<Q>], b: &[ZM<Q>], out: &mut [ZM<Q>]) {
	for c in 0..N {
		mat_vec_mul_ptr::<M, K, Q>(a, 
			&b[index!(K, N, 0, c)..index!(K, N, K, c)], 
			&mut out[index!(M, N, 0, c)..index!(M, N, M, c)]
		);
	}
}

#[test]
fn test_full_mat_mul() {
	let a = [4.into(), 8.into(), 5.into(), 5.into(), 6.into(), 5.into(), 9.into(), 1.into(), 10.into(), 2.into(), 1.into(), 0.into()];
	let b = [1.into(), 2.into(), 8.into(), 3.into(), 1.into(), 4.into(), 0.into(), 7.into()];

	let mut prod = [0.into() ; 3 * 2];
	mat_mul_ptrs::<3, 4, 2, 11>(&a, &b, &mut prod);
	

	assert_eq!(prod, [4.into(), 9.into(), 7.into(), 5.into(), 6.into(), 3.into()]);
}

/**
 * Generates a system of noisy equations to use as a public key for encrypting a single bit
 */
fn gen_bit<const M: usize, const N: usize, const Q: i64, const S: i64>() -> ([ZM<Q> ; N], [ZM<Q> ; M * (N + 1)]) where [(); M * N]: Sized {
	
	let mut s = [0.into(); N];

	for i in 0..N {
		s[i] = ZM::<Q>::rnd();
	}
	
	let mut a: [ZM<Q> ; M * N] = [0.into(); M * N];

	for i in 0..M {
		for j in 0..N {
			a[index!(M, N, i, j)] = ZM::<Q>::rnd();
		}
	}

	// compute b = As + e

	let mut be: [ZM<Q> ; M] = [0.into() ; M];

	// compute b = As first

	mat_mul_ptrs::<M, N, 1, Q>(&a, &s, &mut be);

	// now add the error
	let mut e: [ZM<Q> ; M] = [0.into() ; M];
	error_gen::<M, 1, Q, S>(&mut e);

	let mut pubkey = [0.into() ; M * (N + 1)];
	
	for r in 0..M {
		for c in 0..N {
			pubkey[index!(M, N + 1, r, c)] = a[index!(M, N, r, c)];
		}
	}

	for r in 0..M {
		pubkey[index!(M, N + 1, r, N)] = be[r];
	}

	(s, pubkey)

}

/**
 * Encrypts a bit 
 */
fn enc_bit<const M: usize, const N: usize, const Q: i64, const S: i64>(pubkey: [ZM<Q> ; M * (N + 1)], bit: bool) -> [ZM<Q> ; N + 1] {
	// add all the rows of the public key to create a new equation

	let offset: ZM<Q> = if bit { (Q / 2).into() } else { 0.into() };

	let mut new_eq = [0.into() ; N + 1];

	let mut selections = [0.into() ; M];
	for r in 0..M {
		selections[r] = ZM::<2>::rnd().val.into();
	}

	mat_mul_ptrs::<1, M, {N + 1}, Q>(&selections, &pubkey, &mut new_eq);

	new_eq[N] += offset;

	new_eq
}

/**
 * Decrypts a bit
 */
fn dec_bit<const M: usize, const N: usize, const Q: i64, const S: i64>(seckey: [ZM<Q> ; N], cipher: [ZM<Q> ; N + 1]) -> bool {
	// plug in our secret s into the equation from Bob
	let mut cp: ZM<Q> = 0.into();

	vec_dot_prod_ptr::<1, N, Q>(&seckey, 0, &cipher, &mut cp);

	let diff = (cipher[N] - cp).val;

	// if the difference is small, then we must have encrypted a zero
	(diff > (Q / 4)) && (diff < ((3 * Q) / 4))
}

#[test]
fn test_lwe_bit() {

	for _ in 0..256 {
		let (seckey, pubkey) = gen_bit::<10, 100, 89, 5>();
	
		for b in [true, false] {
			let ciphertext = enc_bit::<10, 100, 89, 5>(pubkey, b);
			let _decrypted = dec_bit::<10, 100, 89, 5>(seckey, ciphertext);

			assert_eq!(_decrypted, b);
		}
	}

}

pub fn gen<const M: usize, const N: usize, const Q: i64, const S: i64, const K: usize>() 
	-> ([ZM<Q> ; N * K], [ZM<Q> ; M * (N + K)]) where [() ; N * K]: Sized, [() ; M * K]: Sized, [() ; M * N]: Sized {


	// println!("GENERATING Key pair");
	
	// generate the secret, S
	let mut s = [0.into(); N * K];

	for i in 0..(N * K) {
		s[i] = ZM::<Q>::rnd();
	}

	// generate the public key A
	let mut a = [0.into() ; M * N];

	for i in 0..(M * N) {
		a[i] = ZM::<Q>::rnd();
	}
	
	// Compute AS + E
	let mut b = [0.into() ; M * K];

	mat_mul_ptrs::<M, N, K, Q>(&a, &s, &mut b);

	let mut e = [0.into() ; M * K];
	error_gen::<M, K, Q, S>(&mut e);

	let mut pubkey = [0.into() ; M * (N + K)];
	
	// we copy the A matrix into the left side of the public key
	for r in 0..M {
		for c in 0..N {
			pubkey[index!(M, N + K, r, c)] = a[index!(M, N, r , c)];
		}
	}

	// add the error to AS, and store it in the right-hand side of the public key
	mat_add::<M, K, Q>(&b, &e, &mut pubkey[index!(M, N + K, 0, N)..index!(M, N + K, M, N + K - 1)]);

	(s, pubkey)

}

pub fn enc<const M: usize, const N: usize, const Q: i64, const S: i64, const K: usize>(pubkey: [ZM<Q> ; M * (N + K)], m: [ZM<2> ; K]) -> [ZM<Q> ; K * (N + 1)] where [() ; K * M]: Sized, [() ; K * N]: Sized, [() ; K * (N + 1)]: Sized {
	let mut t = [0.into() ; K * M];

	// generate selection matrix
	for i in 0..(K * M) {
		t[i] = ZM::<Q>::convert(ZM::<2>::rnd());
	}
	
	// we need to generate the rows of new summed equations

	let mut summed_eqs = [0.into() ; K * (N + 1)]; // K equations, each for a bit, each with N coefficients.

	mat_mul_ptrs::<K, M, N, Q>(&t, &pubkey[index!(M, N + K, 0, 0)..index!(M, N + K, M, N - 1)], &mut summed_eqs[index!(K, N + 1, 0, 0)..index!(K, N + 1, K, N - 1)]);

	for i in 0..K {
		vec_dot_prod_ptr::<K, M, Q>(&t, i, &pubkey[index!(M, N + K, 0, N + i)..index!(M, N + K, M, N + i)], &mut summed_eqs[index!(K, N + 1, i, N)]);
	}


	// great! Now we add offsets to the constant terms as needed.
	let mut offsets = [0.into() ; K];
	let proper_form = m.map(|x| x.val.into());

	scalar_mul::<K, Q>((Q / 2).into(), &proper_form, &mut offsets);

	for i in 0..K {
		summed_eqs[index!(K, N + 1, i, N)] += offsets[i]
	}

	summed_eqs
}

fn dec<const M: usize, const N: usize, const Q: i64, const S: i64, const K: usize>(seckey: [ZM<Q> ; N * K], cipher: [ZM<Q> ; K * (N + 1)]) -> [ZM<2> ; K] {
	// we ONLY need to compute the diagonal of the product matrix.
	let mut diffs = [0.into() ; K];

	for i in 0..K {
		vec_dot_prod_ptr::<K, N, Q>(&cipher[index!(K, N + 1, 0, 0)..index!(K, N + 1, K, N - 1)], i, &seckey[index!(N, K, 0, i)..index!(N, K, N, i)], &mut diffs[i]);
		diffs[i] = cipher[index!(K, N + 1, i, N)] - diffs[i];
	}

	diffs.map(|d| if (d.val > (Q / 4)) && (d.val < ((3 * Q) / 4)) { 1.into() } else { 0.into() })
}

#[test]
fn test_lwe() {
	// These are the same tests as before, but the one-bit versions
	for _ in 1..=256 {
		let (seckey, pubkey) = gen::<100, 30, 3329, 8, 256>();

		// the plaintext!
		let mut b = [0.into() ; 256];

		for i in 0..256 {
			b[i] = ZM::<2>::rnd();
		}
	
		let ciphertext = enc::<100, 30, 3329, 8, 256>(pubkey, b);
		let decrypted = dec::<100, 30, 3329, 8, 256>(seckey, ciphertext);

		assert_eq!(b, decrypted);
	}
}