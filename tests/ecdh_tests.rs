/// Tests for the ECDH protocol
#[cfg(test)]
use rusty_crypto::ecdh;

#[test]
fn test_ecdh_secret_sharing() {

	// make sure that alice and bob do indeed generate the same shared secret

	for _ in 0..99 {
		let alice_priv = ecdh::gen_key();
		let bob_priv = ecdh::gen_key();

		let alice_pub = ecdh::compute_public_key(alice_priv);
		let bob_pub = ecdh::compute_public_key(bob_priv);

		let alice_secret = ecdh::compute_shared_secret(alice_priv, bob_pub);
		let bob_secret   = ecdh::compute_shared_secret(bob_priv, alice_pub);

		assert_eq!(alice_secret, bob_secret);
		
	}

}