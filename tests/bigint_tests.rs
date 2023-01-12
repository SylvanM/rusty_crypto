use rusty_crypto::curve25519::CURVE_MODULUS;
use rusty_crypto::{bigint::BigInt, curve25519};


#[test]
fn test_bn_constructors() {
	let a = BigInt::from_hex_str("FFFFFF77777382593534059328758FE0000000000000000000000000036346");
	println!("{:?}", a.words);
	println!("{}", a.to_hex_str());
}

#[test]
fn test_bn_simple() {
	let a = BigInt::from_hex_str("36359DC37B504BDEF851761A36E154C84AC53E03F8F14CB089B378392D515A096AF588C8F56EE9681B6660FAB2A4CC7A955FC523F1333340D786CDA60503A9D4");
	let b = BigInt::from_hex_str("E25FC2431A8A0DEC62E36B9016D534E71FA4B37302776D4E4F4C5BAB56DB3FF6600A9A524EDFCFAB0E4D6E1817F52FD4CA76A0419554C759839898F35FE92C07");

	let sum_computed = a + b;
	let pro_computed = a * b;
	let quo_computed = a / b;
	let rem_computed = a % b;

	let sum_known = BigInt::from_hex_str("1895600695da59cb5b34e1aa4db689af6a69f176fb68b9fed8ffd3e4842c99ffcb00231b444eb91329b3cf12ca99fc4f5fd665658687fa9a5b1f669964ecd5db");
	let pro_known = BigInt::from_hex_str("9e7a14acd4a6a2cfbdbd85186d8e539fc0a6ac0716e43bc872d91d96168c5cb972e91bf719cea788e582aa7749152ddb397425577e42342c192f28fe023e14cc");
	let quo_known = BigInt::from_hex_str("0");
	let rem_known = BigInt::from_hex_str("36359DC37B504BDEF851761A36E154C84AC53E03F8F14CB089B378392D515A096AF588C8F56EE9681B6660FAB2A4CC7A955FC523F1333340D786CDA60503A9D4");

	assert_eq!(sum_computed, sum_known);
	assert_eq!(pro_computed, pro_known);
	assert_eq!(quo_computed, quo_known);
	assert_eq!(rem_computed, rem_known);

	let (div, rem) = BigInt::full_divide("7B428446BEA9CBC2F05FC6CC012888DDFD0B8B718FA83D4D9E6FE0791798729E4E35F3D056E0EAA72B8613826A118C2E3834358B31D0F8EA5504E3DBCB36E1".into(), curve25519::CURVE_MODULUS);
	assert_eq!(div, "f685088d7d539785e0bf8d98025111bbfa1716e31f507a9b3cdfc0f22f30e5".into());
	assert_eq!(rem, "309a159650a4152996d9bd95ca9615dd21c7eae86684ca126fd9a035d54bd7e0".into());
}

#[test]
fn test_mod_stuff() {
	let m = BigInt::from(23);
	let a = BigInt::from(7);
	let b = BigInt::from(17);

	// test modular multiplication

	assert_eq!(BigInt::from(7) * 9.into(), 63.into());
	assert_eq!(BigInt::mod_mul(7.into(), 9.into(), m), 17.into());

	// Test modular exponentiation

	assert_eq!(BigInt::pow_mod(2.into(), 3.into(), m), 8.into());
	assert_eq!(BigInt::pow_mod(11.into(), 7.into(), m), 7.into());
	assert_eq!(BigInt::pow_mod(2.into(), 4.into(), m), 16.into());
	assert_eq!(BigInt::pow_mod(22.into(), 22.into(), m), 1.into());
	assert_eq!(BigInt::pow_mod(a, b, m), 19.into());

	// Test modular inverses

	assert_eq!(BigInt::mod_div(a, a, m), BigInt::ONE);
	assert_eq!(BigInt::mod_div(b, b, m), BigInt::ONE);

	for _ in 0..99 {
		let mut test_inv = BigInt::rnd(curve25519::KEY_BN_WORD_COUNT);
		test_inv %= CURVE_MODULUS;
		
		let inv = test_inv.mod_mul_inv(CURVE_MODULUS);
		assert_eq!(BigInt::mod_mul(test_inv, inv, CURVE_MODULUS), BigInt::ONE);
	}


}