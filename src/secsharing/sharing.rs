//
// Basic functionality for secure secret sharing
// Created by Sylvan Martin on Feb 26, 2024
//

// not sure why Rust thinks this isn't used... it very much is used,
// and removing the import causes an error
// use rand::seq::IteratorRandom;

use std::{mem::transmute, usize};

use matrix_kit::{index, matrix::Matrix};
use super::types::*;

use algebra_kit::{algebra::*, std_impls::*};
use sylvan_number::{bignumber::BigNumber, ubignumber::{UBigNumber, Word}};

// MARK: The Math Stuff

type Intercept = ZMQ;
type Point = (u64, ZMQ);

// -- Shamir Secret Sharing --

/// Adi Shamir's t-out-of-k secret sharing scheme, where only t out of k total 
/// shares are required to recover the secret, and the secret can be contained in 256 bits.
pub fn create_curve<const T: usize, const K: usize>(secret: Intercept) -> [Point ; K] where [(); K * T]: Sized, [(); T * 1]: Sized, [() ; K * 1]: Sized {
	let mut hs = Matrix::<T, 1, ZMQ>::new();
	hs.flatmap[0] = secret;

	for i in 1..T {
		hs.flatmap[i as usize] = ZMQ::rnd();
	}

	// Now we have the random guys, we need to create the Vandermonde matrix
	// let mut vandermonde = [ZM::<Q>::zero() ; K * T as usize];
	let mut vandermonde = Matrix::<K, T, ZMQ>::new();

	for i in 0..K {
		for j in 0..T {
			vandermonde.flatmap[index!(K, T, i, j)] = (ZMQ::from_ubn((i as u64).into())).power(j as i64);
		}
	}

	let shares = vandermonde * hs;

	let mut labeled_shares = [(0u64, ZMQ::zero()) ; K];

	for i in 0..K {
		labeled_shares[i] = (i as u64, shares.flatmap[i]);
	}

	labeled_shares
}

fn h<const T: usize>(i: usize, a: [ZMQ ; T], x: ZMQ) -> ZMQ {
	let mut value = ZMQ::one();

	for j in 0..T {
		if i == j {
			continue;
		}

		value *= x - a[j]
	}
	
	value
}

pub fn lagrange_interpolate<const T: usize, const K: usize>(points: &[Point]) -> Intercept {
	// Lagrange interpolation! It's a clever idea

	// let's unzip the shares
	let mut xs = [ZMQ::zero() ; T];
	let mut ys = [ZMQ::zero() ; T];

	for i in 0..T {
		let (x, y) = points[i];
		xs[i] = ZMQ::from_ubn(UBigNumber::from_int(x as Word));
		ys[i] = y;
	}

	// we need polynomials p_0 ... p_T-1 so that p_i(shares[i]_0) = 1, but p_i(shares[j]_0) = 0 for all i != j
	// then, our polynomial will be 
	// g(x) = shares[0]p_0(x) + shares[1]p_1(x) + ... + shares[T - 1]p_T-1(x)
	// and we return g(0) = shares[0]p_0(0) + shares[1]p_1(0) + ... + shares[T - 1]p_T-1(0)
	// so, we need to compute all p_i(0)
	// we will have p_i(0) = alpha_i h_i(0), where alpha_i = h_i(shares[i]_0)

	let mut inverses = [ZMQ::zero() ; T];
	
	for i in 0..T {
		inverses[i] = h::<T>(i, xs, xs[i]).inverse();
	}

	let mut ps = [ZMQ::zero() ; T];

	for i in 0..T {
		ps[i] = inverses[i] * h::<T>(i, xs, ZMQ::zero());
	}

	let mut secret = ZMQ::zero();

	for i in 0..T {
		secret += ys[i] * ps[i];
	}

	secret

}

// MARK: Secret Sharing Interface

pub const SECRET_SIZE_BYTES: usize = 32;
pub const SHARE_SIZE_BYTES: usize = SECRET_SIZE_BYTES + (2 * std::mem::size_of::<u64>());

/// A 256-bit secret
pub type Secret256 = [u8 ; SECRET_SIZE_BYTES];

/// A share of a 256-bit secret
/// 
/// This has two additional words to hold the "x coordinate" of the share, and to hold a possibly large share.
pub type Share256 = [u8 ; SHARE_SIZE_BYTES];

fn secret_to_intercept(secret: Secret256) -> Intercept {
	// TODO: This could behave differently on different machines depending on endianness.
	// Consider forcing endianness to avoid this.
	let words: [u64 ; 4] = unsafe { std::mem::transmute(secret) };
	let data = [words[0], words[1], words[2], words[3], 0];

	ZMQ { data }
}

fn intercept_to_secret(intercept: Intercept) -> Secret256 {
	// this fails if the incertept is too big, so it is only to be used with intercepts generated from 4 words.
	assert_eq!(intercept.data[4], 0);

	unsafe {
		std::mem::transmute([intercept.data[0], intercept.data[1], intercept.data[2], intercept.data[3]])
	}
}

fn share_to_point(share: Share256) -> Point {
	// we first take the first word (8 bytes) and make it into the share label, then the remaining 5 words 
	// are the actual value.

	let label = u64::from_be_bytes(share[0..8].try_into().unwrap());

	let data: [u64 ; 5] = unsafe {
		transmute::<[u8 ; SECRET_SIZE_BYTES + 8], [u64 ; 5]>(share[8..].try_into().unwrap())
	};

	(label, ZMQ { data })
}

fn point_to_share(point: Point) -> Share256 {
	let (label, value) = point;

	let mut share = [0u8 ; SHARE_SIZE_BYTES];

	let label_bytes: [u8 ; 8] = label.to_be_bytes().try_into().unwrap();
	for i in 0..8 {
		share[i] = label_bytes[i];
	}

	let data = unsafe {
		transmute::<[u64 ; 5], [u8 ; SECRET_SIZE_BYTES + 8]>(value.data)
	};

	for i in 0..40 {
		share[i + 8] = data[i];
	}

	share
}

/// Creates K shares of a secret, out of which T are required to reconstruct the secret.
pub fn distribute<const T: usize, const K: usize>(secret: Secret256) -> Vec<Share256> where [(); K * T]: Sized, [(); T * 1]: Sized, [() ; K * 1]: Sized {
	let points = create_curve::<T, K>(secret_to_intercept(secret));
	points.map(|p| point_to_share(p)).to_vec()
}

/// Combines T shares of a secret
pub fn reconstruct<const T: usize, const K: usize>(shares: [Share256 ; T]) -> Secret256 {
	let points = shares.map(|s| share_to_point(s));
	intercept_to_secret(lagrange_interpolate::<T, K>(&points))
}

// MARK: Tests

#[cfg(test)]
mod tests {

    use algebra_kit::algebra::Ring;
    use rand::seq::IteratorRandom;

    use crate::{secsharing::sharing::{create_curve, intercept_to_secret, lagrange_interpolate, point_to_share, secret_to_intercept, share_to_point}, speck};

    use super::{distribute, reconstruct, Intercept, Secret256, ZMQ};

	#[test]
	fn test_converstion_symmetry() {
		for _ in 0..100 {
			let secret: Secret256 = speck::gen();
			let shares = distribute::<3, 4>(secret);

			assert_eq!(secret, intercept_to_secret(secret_to_intercept(secret)));

			for share in shares {
				assert_eq!(share, point_to_share(share_to_point(share)));
			}
		}
	}

	#[test]
	fn test_simple_recovery() {
		let secret: Intercept = ZMQ::from_ubn(4.into());

		let share1 = (1, ZMQ::from_ubn(8.into()));
		let share2 = (2, ZMQ::from_ubn(8.into()));
		let share3 = (3, ZMQ::from_ubn(4.into()));

		assert_eq!(secret, lagrange_interpolate::<3, 3>(&[share1, share2, share3]));
	}

	fn sss_test<const T: usize, const K: usize>() where [() ; K * T]: Sized, [() ; T * 1]: Sized, [() ; K * 1]: Sized {
		for _ in 0..100 {
			let secret = ZMQ::rnd();
			let shares = create_curve::<T, K>(secret);

			// first, go ahead and recombibe the first T shares.
			assert_eq!(lagrange_interpolate::<T, K>(&shares[0..T]), secret);
			// come up with random ways of combining the shares!
			for _ in 0..10 {
				let share_combo_refs = shares.iter().choose_multiple(&mut rand::thread_rng(), T);
				let mut share_combo = [(0, ZMQ::zero()) ; T];

				for i in 0..T {
					share_combo[i] = *share_combo_refs[i]
				}

				assert_eq!(lagrange_interpolate::<T, K>(&share_combo[0..T]), secret);
			}
		}
	}

	#[test]
	fn test_secret_sharing() {
	
		// super simple test, with Q = 11, K = 10, and T = 5. So, we only need 5 out of 10 shares.
		sss_test::<5, 10>();
		sss_test::<5, 10>();
		sss_test::<1, 10>();
		sss_test::<10, 10>();
		// sss_test::<5, 61>();
		// sss_test::<50, 50>();
	
	}
}
