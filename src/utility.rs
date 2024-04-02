// use std::{array, mem::transmute};

use rand::{rngs::StdRng, Rng, SeedableRng};

///
/// A utility module, for trying to optimize some stuff that Rust hasn't quite
/// optimized yet
/// 
/// Created 1/16/23 by Sylvan Martin
///

pub trait BigMappable<T, const N: usize> {
	fn big_map_ref<F, U>(self, f: F, into: &mut [U ; N]) where F: FnMut(T) -> U;
	fn big_map<F, U: Default + Copy>(self, f: F) -> [U ; N] where F: FnMut(T) -> U;
}

impl<T: Copy, const N: usize> BigMappable<T, N> for [T; N] {
	fn big_map_ref<F, U>(self, mut f: F, into: &mut [U ; N]) where F: FnMut(T) -> U {
		let mut self_iter = self.iter();

		for x in into.iter_mut() {
			*x = f(*self_iter.next().unwrap())
		}
	}

	fn big_map<F, U: Default + Copy>(self, f: F) -> [U ; N] where F: FnMut(T) -> U {
		let mut new = [Default::default() ; N];

		self.big_map_ref(f, &mut new);

		new
	}
}
 //
// File Utility! Woohoo!
//

use std::{fs::File, io::{Error, Read, Write}};

/**
 * A file stream meant for viewing a file as a sequence of words, with padding
 * built in!
 * 
 * Words are each K butes
 */
pub struct PaddedFileStream<'a, const K: usize> {
	file: &'a mut File,
	counter: usize,
	word_count: usize,
	dangling_count: usize
}

impl<const K: usize> PaddedFileStream<'_, K> {
	pub fn new(file: &mut File) -> PaddedFileStream<K> {
		let size = file.metadata().unwrap().len() as usize;
		let word_count = size / K;
		let dangling_bytes =  size % K;
		println!("Created padded file with word count: {:?}", word_count);
		PaddedFileStream { file: file, counter: 0, word_count: word_count, dangling_count: dangling_bytes }
	}

	pub fn reset(&mut self) {
		self.counter = 0
	}
}

impl<const K: usize> Read for PaddedFileStream<'_, K> {
	fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {

		// zero out the buffer?
		for i in 0..K {
			buf[i] = 0;
		}

		if self.counter < self.word_count {
			self.counter += 1;
			self.file.read(buf)
		} else if self.counter == self.word_count {
			let mut bytes_read = 0;
			for i in 0..self.dangling_count {
				match self.file.read(&mut buf[i..(i + 1)]) {
					Ok(t) => bytes_read += t,
					Err(_) => panic!("Error reading last byte {:?}", i)
				};
			}
			crate::padding::pad(buf, self.dangling_count, 0, K);
			self.counter += 1;
			Ok(bytes_read + (K - self.dangling_count))
		} else {
			println!("Got to end of file at word {:?}", self.counter);
			Err( Error::new(std::io::ErrorKind::Other, "End of file") )
		}
	}
}

impl<const K: usize> Write for PaddedFileStream<'_, K> {
	fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
		self.file.write(buf)
	}

	fn flush(&mut self) -> std::io::Result<()> {
		Ok(())
	}
}
