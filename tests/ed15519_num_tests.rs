use rusty_ecc::ed25519_num::ED25519Num;
use rusty_ecc::u256::{self, U256};


#[test]
fn test_ed_add() {
	// we're gonna make sure that multiplicative inverses work here	
	for _ in 0..99 {
		let a = ED25519Num::rnd();
		assert_eq!(a - a, ED25519Num::ZERO);
	}
}

#[test]
fn test_ed_mul() {
	// we're gonna make sure that multiplicative inverses work here	
	for _ in 0..99 {
		let a = ED25519Num::rnd();

		if a != ED25519Num::ZERO {
			assert_eq!(a / a, ED25519Num::ONE);
		}
	}
}

