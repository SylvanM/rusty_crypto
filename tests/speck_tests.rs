#![feature(generic_const_exprs)]

use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
/// Integration tests for Speck128/256
use rusty_crypto::speck::{self, Block, BLOCK_SIZE};
use std::io::{ErrorKind, Read, Write};
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

#[test]
fn test_speck() {
	for i in 0..1 {
		let mut pt_stream = BlockStream::<10>::new();
		for i in 0..10 {
			let t: [u8 ; BLOCK_SIZE] = StdRng::from_entropy().gen();
			pt_stream.write(&t);
		}
		pt_stream.reset();

		let mut ct_stream = BlockStream::<10>::new();

		let key = speck::gen();
		speck::enc(key, &mut pt_stream, &mut ct_stream);

		ct_stream.reset();

		let mut rec_stream = BlockStream::<10>::new();
		speck::dec(key, &mut ct_stream, &mut rec_stream);

		rec_stream.reset();
		pt_stream.reset();

		for _ in 0..10 {
			let mut b1 = [0 ; BLOCK_SIZE];
			let mut b2 = [0 ; BLOCK_SIZE];

			pt_stream.read(&mut b1);
			rec_stream.read(&mut b2);

			assert_eq!(b1, b2)
		}

	}
}