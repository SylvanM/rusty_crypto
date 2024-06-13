# rusty_crypto
`rusty_crypto` is a small cryptographic suite that I put together in Rust. It contains
the Speck secret key cipher, the SHA512 hash function, Shamir Secret Sharing, and 
the Crystals-Kyber post-quantum public key cipher.

**Important:** I did not write the code for the Kyber stuff! This is just a collection
of crypto routines put together in one crate for my convenience. For Kyber, 
I am using [Argyle-Software's Implementatoin, found here.](https://github.com/Argyle-Software/kyber).
All other crypto routines are written by me.