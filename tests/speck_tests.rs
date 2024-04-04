#![feature(generic_const_exprs)]

use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use rusty_crypto::lwe::Ciphertext;
/// Integration tests for Speck128/256
use rusty_crypto::speck::{self, Block, BLOCK_SIZE};
use rusty_crypto::utility::PaddedFileStream;
use core::panic;
use std::fs::File;
use std::io::{stdin, ErrorKind, Read, Seek, Write};
use std::io::Error;

struct BlockStream<const N: usize> where [(); BLOCK_SIZE * N]: Sized {
	blocks: Box<[u8 ; BLOCK_SIZE * N]>,
	counter: usize
}

impl<const N: usize> BlockStream<N> where [(); BLOCK_SIZE * N]: Sized {
	fn new() -> BlockStream<N> {
		BlockStream { blocks: Box::new([0 ; BLOCK_SIZE * N]), counter: 0 }
	}

	fn reset(&mut self) {
		self.counter = 0
	}
}

impl<const N: usize> Read for BlockStream<N> where [(); BLOCK_SIZE * N]: Sized {
	fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
		if self.counter == N {
			Err( Error::new(ErrorKind::Other, "No space") )
		} else {
			for i in 0..BLOCK_SIZE {
				buf[i] = self.blocks[self.counter * BLOCK_SIZE + i];
			}
			self.counter += 1;
			Ok(BLOCK_SIZE)
		}
	}
}

impl<const N: usize> Write for BlockStream<N> where [(); BLOCK_SIZE * N]: Sized {
	fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
		if self.counter < N {
			for i in 0..BLOCK_SIZE {
				self.blocks[self.counter * BLOCK_SIZE + i] = buf[i];
			}
			self.counter += 1;
			Ok(BLOCK_SIZE)
		} else {
			Err( Error::new(ErrorKind::Other, "No space") )
		}
	}
	
	fn flush(&mut self) -> std::io::Result<()> {
		Ok(()) // don't do anything!
	} 
}

fn open_file(path: &str) -> File {
    match File::options().read(true).write(true).open(path) {
		Ok(f) => f,
		Err(e) => match File::create(path) {
			Ok(_) => open_file(path),
			Err(e) => panic!("Error creating file: {:?}", e)
		}
	}
}

#[test]
fn test_file_enc() {
	for i in 0..1 {

		let mut plaintext_file = open_file("tests/test_files/plaintext");

		// write some random bits!

		for _ in 0..412893 { // arbitrary file length
			let byte: [u8 ; 1] = [StdRng::from_entropy().gen()];
			plaintext_file.write(&byte);
		}
		
		plaintext_file.rewind();

		let mut ciphertext_file = open_file("tests/test_files/ciphertext");

		let key = speck::gen();

		speck::enc(key, &mut plaintext_file, &mut ciphertext_file);


		
		match ciphertext_file.rewind() {
			Ok(_) => (),
			Err(_) => panic!("at the disco")
		};

		let mut recovered = open_file("tests/test_files/recovered");
		recovered.rewind();

		speck::dec(key, &mut ciphertext_file, &mut recovered);
		
		// compare both files!

		recovered.rewind();
		plaintext_file.rewind();

		let rec_len = recovered.metadata().unwrap().len();
		let pt_len = recovered.metadata().unwrap().len();

		let min_len = std::cmp::min(rec_len, pt_len);

		for i in 0..min_len {
			let mut pt_byte = [0];
			let mut re_byte = [0];

			match plaintext_file.read(&mut pt_byte[0..1]) {
				Ok(_) => (),
				Err(e) => panic!("Error reading! {:?}", e)
			};

			match recovered.read(&mut re_byte[0..1]) {
				Ok(_) => (),
				Err(e) => panic!("Error reading! {:?}", e)
			};

			assert_eq!(pt_byte[0], re_byte[0], "Bad decryption on byte {:?}. Should be {:02x}, actually is {:02x}", i, pt_byte[0], re_byte[0]);
		}

		assert_eq!(rec_len, pt_len, "Files of unequal size upon decryption");

	}
}