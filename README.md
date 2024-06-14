# rusty_crypto
`rusty_crypto` is a small cryptographic suite that I put together in Rust. It contains
the Speck secret key cipher, the SHA512 hash function, Shamir Secret Sharing, and 
a public-key encryption scheme based on the CRYSTALS-Kyber Key Encapsulation Mechanism.

**Important:** I did not write the code for the Kyber KEM stuff. That was written by 
[Argyle-Software, found here.](https://github.com/Argyle-Software/kyber) I wrote a simple
module (which I so hilariously called "lettuce" because I guess it kind of sounds like "lattice")
that uses that KEM to create PKE. 

All other code is written by me! Which means you probably shouldn't use it. Just being real.