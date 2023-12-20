use rusty_crypto::lwe;

#[test]
fn test_lwe() {
	for _ in 1..=256 {
		let (seckey, pubkey) = lwe::gen::<100, 10, 89, 5>();
	
		for b in [true, false] {
			let ciphertext = lwe::enc::<100, 10, 89, 5>(pubkey, b);
			let _decrypted = lwe::dec::<100, 10, 89, 5>(seckey, ciphertext);
		}
	}
}

