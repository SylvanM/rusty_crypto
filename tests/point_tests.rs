use rusty_crypto::bigint::BigInt;
use rusty_crypto::curve25519::{KEY_BN_WORD_COUNT, BASE_POINT};

/// Tests for point arithmetic

fn test_commuting_scaling() {
	let a = BigInt::rnd(KEY_BN_WORD_COUNT / 2);
	let b = BigInt::rnd(KEY_BN_WORD_COUNT / 2);
	let ab = a * b;
	let g = BASE_POINT;
	assert_eq!(g * ab, (g * a) * b);
}