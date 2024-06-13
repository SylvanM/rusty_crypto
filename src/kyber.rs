use pqc_kyber::*;
use rand;

// These are just wrappers for the pqc_kyber implementation that just use Vec<u8>
// for everything so that it's more immediately compatible with my other code.


/// Generates a public and private key pair
pub fn gen() -> (Vec<u8>, Vec<u8>) {
	let mut rng = rand::thread_rng();
	let keys = keypair(&mut rng).unwrap();
	(keys.secret.to_vec(), keys.public.to_vec())
}

/// Encapsulates a shared secret using another party's public key
pub fn encapsulate(other_pk: Vec<u8>) -> (Vec<u8>, Vec<u8>) {
	let mut rng = rand::thread_rng();
	let (ct, shared_secret) = pqc_kyber::encapsulate(&other_pk, &mut rng).unwrap();
	(ct.to_vec(), shared_secret.to_vec())
}

/// Decapsulates a shared secret using the ciphertext sent by another party
pub fn decapsulate(my_sk: Vec<u8>, other_ct: Vec<u8>) -> Vec<u8> {
	pqc_kyber::decapsulate(&other_ct, &my_sk).unwrap().to_vec()
}