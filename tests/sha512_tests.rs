/// SHA-512 Tests

use rusty_crypto::ecdh;
use rusty_crypto::sha512;

#[test]
fn test_known_digests() {
	let key = ecdh::gen_key();
	let hash = sha512::ecdh_key_hash(key);
	let num = ecdh::obj_to_bn(hash);
	println!("{:?}", num);
}