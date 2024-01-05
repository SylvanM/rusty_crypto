///
/// Algebra implementations
/// 

use std::ops::*;
use std::fmt::Debug;
use rand::Rng;

/// The field of the integers modulo a prime Q
#[derive(Clone, Copy)]
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
	pub fn rnd() -> ZM<Q> {
		ZM::<Q> { val: rand::thread_rng().gen_range(0..Q) as i64 }
	}

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

impl<const Q: i64> Mul<ZM<Q>> for ZM<Q> {
	type Output = ZM<Q>;

	fn mul(self, rhs: ZM<Q>) -> Self::Output {
		ZM::<Q> { val: (self.val * rhs.val) % Q }
	}
}

