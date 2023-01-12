/// Tests for the ECDH protocol
#[cfg(test)]
use rusty_ecc::{bigint::BigInt, ecdh};

fn test_pubkey_helper(privkey_str: &str, known_pubkey_str: &str) {
	let privkey: BigInt = privkey_str.into();
	let known_pubkey: BigInt = known_pubkey_str.into();

	let computed_public_key = ecdh::compute_public_point(privkey);
	let computed_pubkey_crd = computed_public_key.affine_x().num;

	assert_eq!(known_pubkey, computed_pubkey_crd);
}

#[test]
fn test_pubkey_gen() {

	// Test known public key computations
	// these test cases were taken from https://asecuritysite.com/ecc/eddsa4

	test_pubkey_helper(
		"9d61b19deffd5a60ba844af492ec2cc44449c5697b326919703bac031cae7f60",
		"d75a980182b10ab7d54bfed3c964073a0ee172f3daa62325af021a68f707511a"
	);

	test_pubkey_helper(
		"4ccd089b28ff96da9db6c346ec114e0f5b8a319f35aba624da8cf6ed4fb8a6fb",
		"3d4017c3e843895a92b70aa74d1b7ebc9c982ccf2ec4968cc0cd55f12af4660c"
	);

	test_pubkey_helper(
		"c5aa8df43f9f837bedb7442f31dcb7b166d38535076f094b85ce3a2e0b4458f7",
		"fc51cd8e6218a1a38da47ed00230f0580816ed13ba3303ac5deb911548908025"
	);
}

#[test]
fn test_ecdh_secret_sharing() {

	// make sure that alice and bob do indeed generate the same shared secret

	for i in 0..99 {
		let alice_priv = ecdh::gen_key();
		let bob_priv = ecdh::gen_key();

		let alice_pub = ecdh::compute_public_key(alice_priv);
		let bob_pub = ecdh::compute_public_key(bob_priv);

		let alice_secret = ecdh::compute_shared_secret(alice_priv, bob_pub);
		let bob_secret   = ecdh::compute_shared_secret(bob_priv, alice_pub);

		// this is just so we can see if we actually made it through the 
		// loop at all (we didn't)
		if alice_secret != bob_secret {
			println!("Failed at test {}", i);
			// redoing assertion to print out the values
			assert_eq!(alice_secret, bob_secret);
		}
		
	}

}