///
/// Algebra implementations
/// 

use std::ops::*;
use rand::Rng;

/// The field of the integers modulo a prime Q
#[derive(Clone, Copy)]
pub struct ZM<const Q: i64> {
	pub val: i64
}

impl<const Q: i64> ZM<Q> {
	pub fn rnd() -> ZM<Q> {
		ZM::<Q> { val: rand::thread_rng().gen_range(1..Q) as i64 }
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

