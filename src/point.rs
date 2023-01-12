use std::ops::{Add, Mul};

/// Type and operations for a point on Curve25519
use crate::{curve25519::{self, Curve25519Num}, bigint::BigInt};

/// A group element of Curve25519, represented in projective coordinates,
/// that could be the point at infinity
#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Point {
	PointAtInfinity,
	ProjectiveCoords { x: Curve25519Num, z: Curve25519Num }
}

impl Point {

	/// Creates a point from just the x coordinate
	pub const fn from(x: BigInt) -> Point {
		Point::ProjectiveCoords { 
			x: Curve25519Num { num: x }, 
			z: Curve25519Num::ONE 
		}
	}

	/// Returns the affine x coordinate of this point, assuming that
	/// it isn't PointAtInfinity
	pub fn affine_x(self) -> Curve25519Num {
		match self {
			Self::PointAtInfinity => panic!("Point at infinity!"),
			Self::ProjectiveCoords { x, z } => x / z
		}
	}

	// -- Operations --

	/// Computes self + self, doubling this point
	pub fn double(self) -> Point {
		match self {
			Point::PointAtInfinity => Point::PointAtInfinity,
			Point::ProjectiveCoords { x, z } => Point::ProjectiveCoords { 
				x: (x.squared() - z.squared()).squared(), 
				z: Curve25519Num::from(4) * x * z * (
					x.squared() + (curve25519::ED25519_A * x * z) + z.squared()
				) 
			}
		}
	}

	/// The montgomery ladder
	pub fn ladder(scalar: BigInt, point: Point) -> (Point, Point) {
		if scalar == BigInt::ZERO { (Point::PointAtInfinity, point) }
		else {
			let i = scalar >> 1.into();
			let (pi, pi1) = Self::ladder(i, point);
			if scalar.words[0] & 1 == 0 {
				(pi.double(), pi + pi1)
			} else {
				(pi + pi1, pi1.double())
			}
		}
	}

}

impl Add<Point> for Point {
	type Output = Point;

	fn add(self, rhs: Point) -> Self::Output {
		if self == rhs { self.double() }
		else {
			match self {
				Point::PointAtInfinity => rhs,
				Point::ProjectiveCoords { x: xi, z: zi } => match rhs {
					Point::PointAtInfinity => self,
        			Point::ProjectiveCoords { x: xi1, z: zi1 } => {
						let x = ((xi * xi1) - (zi * zi1)).squared();
						let z = curve25519::BASE_POINT_X 
							* (xi * zi1 - xi1 * zi).squared();
						Point::ProjectiveCoords { x, z }
					}
				}
			}
		}
	}
}

impl Mul<BigInt> for Point {
	type Output = Point;

	fn mul(self, rhs: BigInt) -> Self::Output {
		let (point, _) = Point::ladder(rhs, self);
		point
	}
}