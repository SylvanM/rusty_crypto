# rusty_crypto
A simple cryptographic suite for Rust

TODO: Move the following into the website cryptography markdown file

Okay so we are going to sum at most m equations
each with an error of at most S
so the final error will be mS, which needs to STILL be relatively small
how do we make sure this is relatively small? We need mS to be <1/4 Q

so

mS < 1/4Q

does this make sense? ...yeah right?

So let's choose a HUGE Q, like Q=3329, so Q/4 is a little more than 800, so we'll say we need mS to be under 800
we want m to be as big as possible, maybe say m=200, so we'll say S=4