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
fn test_byte_conversions() {
	let mut bytes: [u8 ; 32] = [0 ; 32];
	for i in 0..32 {
		bytes[i] = i as u8;
	}

	let bignum = U256::from_bytes(bytes);
	assert_eq!(bytes, bignum.to_bytes());
	
	let words = [
		0x8070605040302010,
		0xAA12312312312310,
		0xFFFFFFFFFFFFFFFF,
		0xAAAAAAAAAAAAAAAA
	];

	let a: U256 = "0xAAAAAAAAAAAAAAAAFFFFFFFFFFFFFFFFAA123123123123108070605040302010".into();

	assert_eq!(a, U256::from_le_u64(words));
	assert_eq!(a.to_u64(), words);

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

#[test]
fn test_bignumber_mul() {
	let a: U256 = "0x2EC64DDFC60B42204862EFA12C35973D2E871283E2EA4EB9DF30CB1B501E434C".into();
	let b: U256 = "0xAC42518BF63B731220D4CD212118D440636CCA11D22B9891937F2B2428657E35".into();
	let p: U256 = "0xA44C84C82A68311D3D24516603F78748F7D2E4916CDBFA51FFA00F3AE85F56BC".into();

	assert_eq!(a * b, p);
}

#[test]
fn test_bignumber_div() {
	let a: U256 = "0xAC42518BF63B731220D4CD212118D440636CCA11D22B9891937F2B2428657E35".into();
	let b: U256 = "0x2EC64DDFC60B42204862EFA12C35973D2E871283E2EA4EB9DF30CB1B501E434C".into();
	let q: U256 = "0x3".into();
	let r: U256 = "0x1fef67eca419acb147abfe3d9c780e88d7d79286296cac63f5ecc9d2380ab451".into();

	assert_eq!(a / b, q);
	assert_eq!(a % b, r);
}

#[test]
fn test_gcd() {
	let mut a: U256 = "0x00000000000000000000000000000000C7DAE9C9DBB1E340EEA6CC4DA87AD640".into();
	let mut b: U256 = "0x00000000000000000000000000000001782476BDD39D43D025B4BA55D2323CE3".into();

	let mut x: U256 = u256::ZERO;
	let mut y: U256 = u256::ZERO;

	let mut g = U256::ext_gcd(a, b, &mut x, &mut y);

	println!("{:?}", g);

	a = 1398.into();
	b = 324.into();

	g = U256::ext_gcd(a, b, &mut x, &mut y);

	assert_eq!(g, 6.into());
	println!("x: {:?}", x);
	println!("y: {:?}", y);
}