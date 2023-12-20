//
// The basic Learning with Errors over integer lattices
//

use crate::algebra::*;

use rand::Rng;

/**
 * Generates a system of noisy equations to use as a public key
 */
pub fn gen<const N: usize, const M: usize, const Q: i64, const S: i64>() -> ([ZM<Q> ; N], [ZM<Q> ; M * (N + 1)]) {

	let mut s = [0.into(); N];

	for i in 0..N {
		s[i] = ZM::<Q>::rnd();
	}

	let mut a = [[0.into(); N] ; M];
	for i in 0..M {
		for j in 0..N {
			a[i][j] = ZM::<Q>::rnd()
		}
	}

	// compute b = As + e

	let mut be: [ZM<Q> ; M] = [0.into() ; M];

	for r in 0..M {
		let mut cp = 0.into();
		for c in 0..N {
			cp += a[r][c] * s[c] + (rand::thread_rng().gen_range(-S..=S)).into()
		}
		be[r] = cp
	}

	let mut pubkey = [0.into() ; M * (N + 1)];
	
	for r in 0..M {
		for c in 0..N {
			pubkey[r * N + c] = a[r][c]
		}
	}

	for r in 0..M {
		pubkey[r * N + N] = be[r]
	}

	(s, pubkey)

}

/**
 * Encrypts a bit 
 */
pub fn enc<const N: usize, const M: usize, const Q: i64, const S: i64>(pubkey: [ZM<Q> ; M * (N + 1)], bit: bool) -> [ZM<Q> ; N + 1] {
	// add all the rows of the public key to create a new equation

	let offset: ZM<Q> = if bit { (S / 2).into() } else { 0.into() };

	let mut new_eq = [0.into() ; N + 1];

	let mut selections = [false ; M];

	for r in 0..M {
		selections[r] = rand::thread_rng().gen_bool(0.5);
	}
	
	for c in 0..=N {
		for r in 0..M {
			if selections[r] {
				new_eq[c] += pubkey[r * N + c];
			}
		}
	}

	new_eq[N] += offset;

	new_eq
}

/**
 * Decrypts a bit
 */
pub fn dec<const N: usize, const M: usize, const Q: i64, const S: i64>(seckey: [ZM<Q> ; N], cipher: [ZM<Q> ; N + 1]) -> bool {
	// plug in our secret s into the equation from Bob
	let mut cp: ZM<Q> = 0.into();

	for i in 0..N {
		cp += seckey[i] * cipher[i]
	}

	let diff = (cipher[N] - cp).val;

	// if the difference is small, then we must have encrypted a zero
	(diff > (S / 4)) && (diff < ((3 * S) / 4))
}