//
// The SHA-3 Hashing function, as described in the paper 
// https://nvlpubs.nist.gov/nistpubs/FIPS/NIST.FIPS.202.pdf
//

use std::mem::transmute;

type Word = u64;

const W: usize = std::mem::size_of::<Word>() * 8;
const L: usize = (W as f32).log2().floor() as usize;

type State = [[Word ; 5] ; 5];
type Block = [u8 ; 25 * W / 8];

const ROUNDS: usize = 12 + 2 * L;

/**
 * Performs the theta step in a round of SHA-3
 */
fn theta(a: State) -> State {
	let mut c = [0 ; 5];

	for x in 0..5 {
		c[x] = a[x][0] ^ a[x][1] ^ a[x][2] ^ a[x][3] ^ a[x][4];
	}

	let mut d = [0 ; 5];

	for x in 0..5 {
		d[x] = c[(x - 1).rem_euclid(5)] ^ c[(x - 1).rem_euclid(5)].rotate_right(1);
	}

	let mut ap = a;

	for x in 0..5 {
		for y in 0..5 {
			ap[x][y] ^= d[x];
		}
	}

	ap
	
}

/**
 * This 
 */

/**
 * Performs the rho step in a round of SHA-3
 */
fn rho(a: State) -> State {
	let mut new_state = a;

	let mut x = 1;
	let mut y = 0;

	for t in 0..24 {

		new_state[x][y] = a[x][y].rotate_right(((t + 1) * (t - 1)) / 2);

		// update indices
		x = y;
		y = (2 * x + 3 * y) % 5;
	}

	new_state
}

/**
 * Performs the pi step in a round of SHA-3
 */
fn pi(a: State) -> State {
	let mut new_state = a;

	for x in 0..5 {
		for y in 0..5 {
			new_state[x][y] ^= a[(x + 3 * y) % 5][x];
		}
	}

	new_state
}

/**
 * Performs the chi step in a round of SHA-3
 */
fn chi(a: State) -> State {
	let mut new_state = a;

	for x in 0..5 {
		for y in 0..5 {
			new_state[x][y] = !a[(x + 1) % 5][y] & a[(x + 2) % 5][y];
		}
	}

	new_state
}

// returns the b-th bit of x
const fn bit(x: Word, b: Word) -> Word {
	(x & (1 << b)) >> b
}

// sets a bit b in x to val
 fn set(x: &mut Word, b: Word, val: Word) {
	if val == 0 {
		*x &= !(1 << b);
	} else {
		*x |= 1 << b;
	}
}

fn rc(t: Word) -> Word {

	if t % 255 == 0 {
		return 1;
	}

	let mut r = 0b00000001;

	for i in 0..=(t % 255) {
		r <<= 1;
		set(&mut r, 0, bit(r, 0) ^ bit(r, 8));
		set(&mut r, 4, bit(r, 4) ^ bit(r, 8));
		set(&mut r, 5, bit(r, 5) ^ bit(r, 8));
		set(&mut r, 6, bit(r, 6) ^ bit(r, 8));
		set(&mut r, 8, 0); // we truncate down to 8 bits by setting the 8th bit to 0
	}

	r & 1
}

/**
 * Performs the iota step in a round of SHA-3 on a state with a round index
 */
fn iota(a: State, i: Word) -> State {
	let mut new_state = a;
	let mut _rc: Word = 0;

	for j in 0..=L {
		set(&mut _rc, (2 as Word).pow(j as u32) - 1, rc((j as Word) + 7 * (i as u64)));
	}

	new_state[0][0] ^= _rc;

	new_state
}

/**
 * Performs the round function on a state
 */
fn round(a: State, i: Word) -> State {
	iota(chi(pi(rho(theta(a)))), i)
}

/**
 * Converts a string of length 5 * 5 * W to a state
 */
fn block_to_state(block: Block) -> State {
	let word_block: [Word ; 25] = unsafe { transmute(block) };
	let mut state: State;
	
	for x in 0..5 {
		for y in 0..5 {
			state[x][y] = word_block[W * (5 * y + x)];
		}
	}

	state
}

/**
 * Converts a 
 */
fn state_to_block(state: State) -> Block {
	let mut word_block: [Word ; 25];
	
	for x in 0..5 {
		for y in 0..5 {
			word_block[5 * x + y] = state[x][y];
		}
	}

	unsafe {
		transmute(word_block)
	}
}

/**
 * Performs the Keccak-p permutation
 */
fn perumte(block: Block, n: Word) -> Block {
	let mut a = block_to_state(block);

	for i in (12 + 2 * (L as Word) - n)..(12 + 2 * (L as Word)) {
		a = round(a, i as Word);
	}
	
	state_to_block(a)
}

fn pad101(x: usize, m: usize) -> &[u8] {
	
}

/**
 * A sponge! This uses permute as its "f" function, and "pad101" as it's padding function
 * 
 * It takes in 
 */
fn sponge<const rate: usize>() ->  {

}