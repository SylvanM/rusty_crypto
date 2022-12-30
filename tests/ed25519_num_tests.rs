use rusty_ecc::ed25519_num::ED25519Num;
use rusty_ecc::ed25519_num;

#[test]
fn test_bignumber_constuctors() {
	let a: ED25519Num = 0.into();
	assert_eq!(a.words, [0, 0]);

	let b: ED25519Num = 1.into();
	assert_eq!(b.words, [1, 0]);

	let c: ED25519Num = u128::MAX.into();
	assert_eq!(c.words, [u128::MAX, 0]);

	let d: ED25519Num = (-1).into();
	assert_eq!(d, ed25519_num::ZERO);
}

#[test]
fn test_bignumber_add() {
	let a = ED25519Num { words: [ 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF ] };
	let b = ED25519Num { words: [ 0x00000000000000000000000000000001, 0x00000000000000000000000000000000 ] };
	let sum = a + b;
	assert_eq!(sum.words, [0, 0]);
}

#[test]
fn test_bignumber_sub() {
	let a = ED25519Num { words: [ 0x00000000000000000000000000000000, 0x00000000000000000000000000000001 ] };
	let b = ED25519Num { words: [ 0x00000000000000000000000000000001, 0x00000000000000000000000000000000 ] };
	let diff1 = a - b;
	assert_eq!(diff1.words, [0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF, 0]);

	let zero = ED25519Num { words: [0, 0] };
	let one: ED25519Num = 1.into();
	let diff2 = zero - one;
	assert_eq!(diff2.words, [0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF])
}