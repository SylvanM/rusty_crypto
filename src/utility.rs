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