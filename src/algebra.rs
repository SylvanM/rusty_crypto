///
/// Algebra implementations
/// 

use std::{i64, ops::*};
use std::fmt::Debug;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

pub trait Ring: Copy + Sized + Add + AddAssign + Sub + SubAssign + Mul + MulAssign + Mul<Output = Self> + Add<Output = Self> {
 	const ONE: Self;
	const ZERO: Self;
	fn negate(self) -> Self;
	fn power(self, n: i64) -> Self;
	fn rnd() -> Self;
}

pub trait Field: Ring + Div + DivAssign + Copy + std::ops::Mul<Output = Self> {
	fn inverse(self) -> Self;
}

/// The field of the integers modulo a prime Q
#[derive(Clone, Copy, Default)]
pub struct ZM<const Q: i64> {
	pub val: i64
}

impl<const Q: i64> Debug for ZM<Q> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		self.val.fmt(f)
	}
}

impl<const Q: i64> PartialEq for ZM<Q> {
	fn eq(&self, other: &Self) -> bool {
		self.val == other.val
	}

	fn ne(&self, other: &Self) -> bool {
		self.val != other.val
	}
}

impl<const Q: i64> ZM<Q> {
	pub fn convert<const P: i64>(other: ZM<P>) -> ZM<Q> {
		other.val.into()
	}

	pub fn from_int(x: i64) -> ZM<Q> {
		x.into()
	}
}

impl<const Q: i64> From<i64> for ZM<Q> {
	fn from(value: i64) -> Self {
		ZM::<Q> { val: value.rem_euclid(Q) } 
	}
}

impl<const Q: i64> From<u8> for ZM<Q> {
	fn from(value: u8) -> Self {
		ZM::<Q> { val: (value as i64).rem_euclid(Q) } 
	}
}

impl<const Q: i64> From<i32> for ZM<Q> {
	fn from(value: i32) -> Self {
		ZM::<Q> { val: (value as i64).rem_euclid(Q) } 
	}
}

impl<const Q: i64> Add<ZM<Q>> for ZM<Q> {
	type Output = ZM<Q>;

	fn add(self, rhs: ZM<Q>) -> Self::Output {
		ZM::<Q> { val: (self.val + rhs.val) % Q }
	}
}

impl<const Q: i64> AddAssign<ZM<Q>> for ZM<Q> {
	fn add_assign(&mut self, rhs: ZM<Q>) {
		*self = *self + rhs
	}
}

impl<const Q: i64> Sub<ZM<Q>> for ZM<Q> {
	type Output = ZM<Q>;

	fn sub(self, rhs: ZM<Q>) -> Self::Output {
		ZM::<Q> { val: (self.val - rhs.val + Q) % Q }
	}
}

impl<const Q: i64> SubAssign<ZM<Q>> for ZM<Q> {
	fn sub_assign(&mut self, rhs: ZM<Q>) {
		*self = *self - rhs
	}
}

impl<const Q: i64> Mul<ZM<Q>> for ZM<Q> {
	type Output = ZM<Q>;

	fn mul(self, rhs: ZM<Q>) -> ZM<Q> {
		ZM::<Q> { val: ((((self.val % Q) as i128) * ((rhs.val % Q) as i128)) % (Q as i128)) as i64 }
	}
}

impl<const Q: i64> MulAssign<ZM<Q>> for ZM<Q> {
	fn mul_assign(&mut self, rhs: ZM<Q>) {
		*self = *self * rhs
	}
}

impl<const Q: i64> Ring for ZM<Q> {
	const ONE: Self = ZM::<Q> { val: 1 } ;
	const ZERO: Self = ZM::<Q> { val: 0 };

	fn negate(self) -> Self {
		(Q - self.val).into()
	}

	fn rnd() -> ZM<Q> {
		ZM::<Q> { val: StdRng::from_entropy().gen() }
	}

	fn power(self, n: i64) -> Self {
		// TODO: Make this WAYY more efficient... Double and add, yeah?
		let mut power = ZM::<Q>::ONE;

		for _ in 1..=n {
			power *= self
		}

		power
	}
}

/// Returns (g, x, y) so that 
/// - g = gcd(a, b)
/// ax + by = gcd(a, b)
fn ext_gcd(a: i64, b: i64) -> (i64, i64, i64) {

	if a == 0 {
		return (b, 0, 1)
	}

	let (g, x1, y1) = ext_gcd(b % a, a);

	let x = y1 - (b/a) * x1;
	let y = x1;

	(g, x, y)
}

fn mod_inv(x: i64, m: i64) -> i64 {
	match ext_gcd(x, m) { (_, i, _) => i }
}

impl<const Q: i64> Div<ZM<Q>> for ZM<Q> {
	type Output = ZM<Q>;

	fn div(self, rhs: ZM<Q>) -> Self::Output {
		self * mod_inv(rhs.val, Q).into()
	}
}

impl<const Q: i64> DivAssign<ZM<Q>> for ZM<Q> {
	fn div_assign(&mut self, rhs: ZM<Q>) {
		*self = *self / rhs
	}
}

impl<const Q: i64> Field for ZM<Q> {
	fn inverse(self) -> Self {
		mod_inv(self.val, Q).into()
	}
}

// -- POLYNOMIALS --

/// A polynomial of at most degree `D` with coefficients over `F`
/// Notice, this allows for the D-th coefficient to be zero.
#[derive(Copy, Clone)]
pub struct Polynomial<const D: i64, F: Field> where [(); D as usize]: Sized {
	pub coefficients: [F ; D as usize]
}

impl<const D: i64, F: Field> Polynomial<D, F> where [(); D as usize]: Sized {
	pub fn rnd() -> Self {
		let mut coeffs = [F::ZERO; D as usize];
		for i in 0..=D {
			coeffs[i as usize] = F::rnd();
		}
		Polynomial { coefficients: coeffs }
	}
}

impl<const D: i64, F: Field> Polynomial<D, F> where [(); D as usize]: Sized {
	pub fn evaluate(self, point: F) -> F {
		let mut value = F::ZERO;

		for i in 0..=D {
			value += point * self.coefficients[i as usize].power(i)
		}

		value
	}
}

// -- MATRICES --

#[macro_export]
macro_rules! index {
	($m: expr, $n: expr, $r: expr, $c: expr) => {
		$c * $m + $r
	};
}

/// Computes the dot product between the r-th row of the matrix x, and the vector y.
/// 
/// * `M` - 
pub fn vec_dot_prod_ptr<const M: usize, const N: usize, R: Ring>(x: &[R], r: usize, y: &[R], out: &mut R) {
	*out = R::ZERO;
	for i in 0..N {
		*out += x[index!(M, N, r, i)] * y[i]
	}
}

pub fn mat_vec_mul_ptr<const M: usize, const N: usize, R: Ring>(a: &[R], vec: &[R], to_vec: &mut [R]) {
	for r in 0..M {
		vec_dot_prod_ptr::<M, N, R>(a, r, vec, &mut to_vec[r]);
	}
}

pub fn mat_add<const M: usize, const N: usize, R: Ring>(a: &[R], b: &[R], out: &mut [R]) {
	for i in 0..(M * N) {
		out[i] = a[i] + b[i];
	}
}

pub fn scalar_mul<const N: usize, R: Ring>(k: R, v: &[R], out: &mut [R]) {
	for i in 0..N {
		out[i] = k * v[i];
	}
}

#[test]
fn test_mat_vec_mul_ptr() {
	// test the super simple identity!
	let identity = [1.into(), 0.into(), 0.into(), 1.into()];
	let simple_vector = [3.into(), 7.into()];
	let mut out_vector = [0.into() ; 2];
	mat_vec_mul_ptr::<2, 2, ZM<11>>(&identity, &simple_vector, &mut out_vector);

	assert_eq!(out_vector, simple_vector);

	let mat = [3.into(), 2.into(), 5.into(), 1.into(), 7.into(), 0.into()];
	let vec = [1.into(), 4.into(), 9.into()];
	let mut out = [0.into() ; 2];
	mat_vec_mul_ptr::<2, 3, ZM<11>>(&mat, &vec, &mut out);

	assert_eq!(out, [9.into(), 6.into()]);

}

pub fn mat_mul_ptrs<const M: usize, const K: usize, const N: usize, R: Ring>(a: &[R], b: &[R], out: &mut [R]) {
	for c in 0..N {
		mat_vec_mul_ptr::<M, K, R>(a, 
			&b[index!(K, N, 0, c)..index!(K, N, K, c)], 
			&mut out[index!(M, N, 0, c)..index!(M, N, M, c)]
		);
	}
}

#[test]
fn test_full_mat_mul() {
	let a = [4.into(), 8.into(), 5.into(), 5.into(), 6.into(), 5.into(), 9.into(), 1.into(), 10.into(), 2.into(), 1.into(), 0.into()];
	let b = [1.into(), 2.into(), 8.into(), 3.into(), 1.into(), 4.into(), 0.into(), 7.into()];

	let mut prod = [0.into() ; 3 * 2];
	mat_mul_ptrs::<3, 4, 2, ZM<11>>(&a, &b, &mut prod);
	

	assert_eq!(prod, [4.into(), 9.into(), 7.into(), 5.into(), 6.into(), 3.into()]);
}