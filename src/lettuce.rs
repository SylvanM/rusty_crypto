use crate::speck;

pub const PK_BYTES: usize = pqc_kyber::KYBER_PUBLICKEYBYTES;
pub const SK_BYTES: usize = pqc_kyber::KYBER_SECRETKEYBYTES;
const SHARED_SECRET_BYTES: usize = pqc_kyber::KYBER_SSBYTES;
const CIPHERTEXT_KEM_BYTES: usize = pqc_kyber::KYBER_CIPHERTEXTBYTES;

pub type PublicKey = [u8 ; PK_BYTES];
pub type SecretKey = [u8 ; SK_BYTES];

/// A type for the plaintext, an abitrary string of bytes.
pub type Plaintext = Vec<u8>;
pub type Ciphertext = Vec<u8>;

type SharedSecret = [u8 ; SHARED_SECRET_BYTES];
type CiphertextKEM = [u8 ; CIPHERTEXT_KEM_BYTES];

/// A struct holding both a secret and public lettuce key.
///
/// This is stored as a struct rather than as a tuple so that we don't have to worry about
/// the ordering of tuple entries messing up and accidentally publishing the secret key or
/// something.
pub struct KeyPair {
    pub secret_key: SecretKey,
    pub public_key: PublicKey
}

impl From<pqc_kyber::Keypair> for KeyPair {
    fn from(value: pqc_kyber::Keypair) -> Self {
        KeyPair {
            secret_key: value.secret,
            public_key: value.public
        }
    }
}

/// Generates a secret and public key pair.
pub fn gen() -> KeyPair {
    let mut rng = rand::thread_rng();
    match pqc_kyber::keypair(&mut rng) {
        Ok(kp) => kp.into(),
        Err(_) => panic!("Error generating key pair")
    }
}

/// Encrypts a message using another party's public key
pub fn enc(public_key: PublicKey, plaintext: Plaintext) -> Ciphertext {
    let mut rng = rand::thread_rng();

    let (ct_kem, secret) = match pqc_kyber::encapsulate(public_key.as_slice(), &mut rng) {
        Ok(t) => t,
        Err(_) => panic!("Error encapsulating key")
    };

    let inner_ciphertext = speck::enc_vec(secret, plaintext);
    let mut ct = vec![0 ; CIPHERTEXT_KEM_BYTES + inner_ciphertext.len()];

    for i in 0..CIPHERTEXT_KEM_BYTES {
        ct[i] = ct_kem[i];
    }

    for i in CIPHERTEXT_KEM_BYTES..(ct.len()) {
        ct[i] = inner_ciphertext[i - CIPHERTEXT_KEM_BYTES];
    }

    ct
}

/// Decrypts a ciphertext using this party's secret key
pub fn dec(secret_key: SecretKey, ciphertext: Ciphertext) -> Plaintext {    
    let ct_kem = &ciphertext[..CIPHERTEXT_KEM_BYTES];
    let inner_ciphertext = &ciphertext[CIPHERTEXT_KEM_BYTES..];
    let shared_secret = match pqc_kyber::decapsulate(ct_kem, &secret_key) {
        Ok(s) => s,
        Err(_) => panic!("Error decapsulating")
    };
    speck::dec_vec(shared_secret, inner_ciphertext.to_vec())
}


mod tests {
    use rand::Rng;

    #[test]
    fn test_symmetry() {

        for _ in 0..100 {
            // Alice wants to send plaintext to Bob!

            let mut plaintext = vec![0 ; rand::thread_rng().gen_range(4..1000)];
            for i in 0..plaintext.len() {
                plaintext[i] = rand::thread_rng().gen();
            }

            let bobs_keys = super::gen();
            
            let alices_ciphertext = super::enc(bobs_keys.public_key, plaintext.clone());
            let bobs_decrypted_pt = super::dec(bobs_keys.secret_key, alices_ciphertext);

            assert_eq!(plaintext, bobs_decrypted_pt)
        }

    }

}