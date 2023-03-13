//
// The SHA-3 Hashing function
//

type Word = u64;

const W: usize = std::mem::size_of::<Word>();
const L: usize = (W as f32).log2().floor() as usize;

type State = [[Word ; 5] ; 5];

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
			new_state[x][y] ^= !a[(x + 1) % 5][y] & a[(x + 2) % 5][y]
		}
	}

	new_state
}

fn rc(t: usize) -> usize {

	// returns the b-th bit of x
	const fn bit(x: usize, b: usize) -> usize {
		(x & (1 << b)) >> b
	}

	// sets a bit b in x to val
 	fn set(x: &mut usize, b: usize, val: usize) {
		if val == 0 {
			*x &= !(1 << b);
		} else {
			*x |= 1 << b;
		}
	}

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
 * Performs the iota step in a round of SHA-3
 */
fn iota(a: State) -> State {

}

/**
 * Performs the chi step in a round of SHA-3
 */
fn chi(a: State) -> State {
	let mut new_state = a;

	for x in 0..5 {
		for y in 0..5 {
			new_state[x][y] = a[x][y] ^ ();
		}
	}

	new_state
}

/**
 * Performs one round of SHA-3 on a State
 */
fn round(state: State) -> State {

}