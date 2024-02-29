//
// Basic functionality for secure secret sharing
// Created by Sylvan Martin on Feb 26, 2024
//

use rand::seq::IteratorRandom;

use crate::{algebra::*, index};

// -- Shamir Secret Sharing --

/// Adi Shamir's t-out-of-k secret sharing scheme, where only t out of k total 
/// shares are required to recover the secret.
///
/// Precondition:
///		- Q is prime 
///		- Q > K
pub fn distribute<const T: usize, const K: usize, const Q: i64>(secret: ZM<Q>) -> [(usize, ZM<Q>) ; K] where [(); K * T as usize]: Sized {
	let mut hs = [ZM::<Q>::ZERO ; T as usize];
	hs[0] = secret;

	for i in 1..T {
		hs[i as usize] = ZM::<Q>::rnd();
	}

	// Now we have the random guys, we need to create the Vandermonde matrix
	let mut vandermonde = [ZM::<Q>::ZERO ; K * T as usize];

	for i in 0..K {
		for j in 0..T {
			vandermonde[index!(K, T, i, j)] = (ZM::<Q>::from_int(i as i64)).power(j as i64);
		}
	}

	let mut shares = [ZM::<Q>::ZERO ; K];

	mat_mul_ptrs::<K, T, 1, ZM<Q>>(&vandermonde, &hs, &mut shares);

	let mut labeled_shares = [(0, ZM::<Q>::ZERO) ; K];

	for i in 0..K {
		labeled_shares[i] = (i, shares[i]);
	}

	labeled_shares
}

fn h<const T: usize, const Q: i64>(i: usize, a: [ZM<Q> ; T], x: ZM<Q>) -> ZM<Q> {
	let mut value = ZM::<Q>::ONE;

	for j in 0..T {
		if i == j {
			continue;
		}

		value *= x - a[j]
	}
	
	value
}

pub fn reconstruct<const T: usize, const K: usize, const Q: i64>(shares: &[(usize, ZM<Q>)]) -> ZM<Q> {
	// Lagrange interpolation! It's a clever idea

	// let's unzip the shares
	let mut xs = [ZM::<Q>::ZERO ; T];
	let mut ys = [ZM::<Q>::ZERO ; T];

	for i in 0..T {
		let (x, y) = shares[i];
		xs[i] = (x as i64).into();
		ys[i] = y;
	}

	// we need polynomials p_0 ... p_T-1 so that p_i(shares[i]_0) = 1, but p_i(shares[j]_0) = 0 for all i != j
	// then, our polynomial will be 
	// g(x) = shares[0]p_0(x) + shares[1]p_1(x) + ... + shares[T - 1]p_T-1(x)
	// and we return g(0) = shares[0]p_0(0) + shares[1]p_1(0) + ... + shares[T - 1]p_T-1(0)
	// so, we need to compute all p_i(0)
	// we will have p_i(0) = alpha_i h_i(0), where alpha_i = h_i(shares[i]_0)

	let mut inverses = [ZM::<Q>::ZERO ; T];
	
	for i in 0..T {
		inverses[i] = h::<T, Q>(i, xs, xs[i]).inverse();
	}

	let mut ps = [ZM::<Q>::ZERO ; T];

	for i in 0..T {
		ps[i] = inverses[i] * h::<T, Q>(i, xs, ZM::<Q>::ZERO);
	}

	let mut secret = ZM::<Q>::ZERO;

	for i in 0..T {
		secret += ys[i] * ps[i];
	}

	secret

}

#[test]
fn test_secret_sharing() {
	
	// super simple test, with Q = 11, K = 10, and T = 5. So, we only need 5 out of 10 shares.

	fn sss_test<const T: usize, const K: usize, const Q: i64>() where [() ; K * T as usize]: Sized {
		for _ in 0..100 {
			let secret = ZM::<Q>::rnd();
			let shares = distribute::<T, K, Q>(secret);

			// come up with random ways of combining the shares!
			for _ in 0..10 {
				let share_combo_refs = shares.iter().choose_multiple(&mut rand::thread_rng(), T);
				let mut share_combo = [(0, ZM::<Q>::ZERO) ; T];

				for i in 0..T {
					share_combo[i] = *share_combo_refs[i]
				}

				assert_eq!(reconstruct::<T, K, Q>(&share_combo[0..T]), secret);
			}
		}
	}

	sss_test::<5, 10, 11>();
	sss_test::<5, 10, 6113>();
	sss_test::<1, 10, 6113>();
	sss_test::<10, 10, 6113>();
	sss_test::<5, 6113, 6113>();
	sss_test::<50, 50, 6113>();

}