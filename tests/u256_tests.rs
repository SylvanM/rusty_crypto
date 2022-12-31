use rusty_ecc::u256::U256;
use rusty_ecc::u256;

#[test]
fn test_bignumber_constuctors() {
	let a: U256 = 0.into();
	assert_eq!(a.words, [0, 0]);

	let b: U256 = 1.into();
	assert_eq!(b.words, [1, 0]);

	let c: U256 = u128::MAX.into();
	assert_eq!(c.words, [u128::MAX, 0]);

	let d: U256 = (-1).into();
	assert_eq!(d, u256::ZERO);
}

#[test]
fn test_bignumber_strs() {
	let a = U256::from_hex_str("0x1");
	assert_eq!(a, 1.into());

	let b = U256::from_hex_str("1");
	assert_eq!(b, 1.into());

	let c = U256::from_hex_str("0x4000000000000000300000000000000020000000000000001");
	assert_eq!(c.words, [0x00000000000000020000000000000001, 0x40000000000000003]);

	let d = U256::from_hex_str("0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF");
	assert_eq!(d.words, [ 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF ])
}

#[test]
fn test_bignumber_add() {
	let a = U256 { words: [ 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF ] };
	let b = U256 { words: [ 0x00000000000000000000000000000001, 0x00000000000000000000000000000000 ] };
	let sum = a + b;
	assert_eq!(sum.words, [0, 0]);
}

#[test]
fn test_bignumber_sub() {
	let a = U256 { words: [ 0x00000000000000000000000000000000, 0x00000000000000000000000000000001 ] };
	let b = U256 { words: [ 0x00000000000000000000000000000001, 0x00000000000000000000000000000000 ] };
	let diff1 = a - b;
	assert_eq!(diff1.words, [0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF, 0]);

	let zero = U256 { words: [0, 0] };
	let one: U256 = 1.into();
	let diff2 = zero - one;
	assert_eq!(diff2.words, [0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF])
}