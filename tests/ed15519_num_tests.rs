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

#[test]
fn test_ed_gcd() {
	let mut a: U256 = 180.into();
	let mut b: U256 = 150.into();

	let mut x: U256 = u256::ZERO;
	let mut y: U256 = u256::ZERO;

	let mut g = U256::ext_gcd(a, b, &mut x, &mut y);

	assert_eq!((a * x) + (b * y), g);
	assert_eq!(g, 30.into());

	a = 191.into();
	b = 43.into();

	g = U256::ext_gcd(a, b, &mut x, &mut y);

	assert_eq!(g, 1.into());
	assert_eq!((a * x) + (b * y), g);

	println!("a: {:?}, \nb: {:?}, \nx: {:?}, \ny: {:?}, \ngcd: {:?}\n", a, b, x, y, g);

	println!("STUFF HEREEE");
	println!("{:?}", a % b);
	println!("{:?}", x % b);
	println!("{:?}", ((a % b) * (x % b)) % b);

	// assert_eq!(((a % b) * (x % b)) % b, u256::ONE);
	// assert_eq!(((b % a) * (y % a)) % a, u256::ONE);

	a = "0x00000000000000000000000000000000C7DAE9C9DBB1E340EEA6CC4DA87AD640".into();
	b = "0x00000000000000000000000000000001782476BDD39D43D025B4BA55D2323CE3".into();

	g = U256::ext_gcd(a, b, &mut x, &mut y);

	assert_eq!(g, u256::ONE);

	// verify that the inverse stuff works
	assert_eq!(((a % b) * (x % b)) % b, u256::ONE);
	assert_eq!(((b % a) * (y % a)) % a, u256::ONE);
	
}