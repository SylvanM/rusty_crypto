/// 
/// Elliptic Curve Diffie-Helmen secret sharing algorithm
/// 
use crate::{curve25519::{self, BASE_POINT, KEY_BYTE_COUNT}, point::Point, bigint::{BigInt, self}};

/// A little endian byte array for a key or point value
pub type ED25519Obj = [u8 ; curve25519::KEY_BYTE_COUNT];

// -- Exposed Byte Operations ---

/// Generates a random private key
/// 
/// Currenty **NOT** cryptographically secure
pub fn gen_key() -> ED25519Obj {
	todo!()
}

/// Computes the public key for the given private key
pub fn compute_public_key(private: ED25519Obj) -> ED25519Obj {
	let scalar = obj_to_bn(private);
	let public_point = compute_public_point(scalar);
	pnt_to_obj(public_point)
}

/// Computes the shared secret
pub fn compute_shared_secret(private: ED25519Obj, other_public: ED25519Obj) -> ED25519Obj {
	let private_scalar = obj_to_bn(private);
	let other_point = obj_to_pnt(other_public);
	let shared_secret = compute_shared_secret_point(
		private_scalar, other_point
	);
	pnt_to_obj(shared_secret)
}

// -- Internal Operations --

/// Scales the base point by some value, computing the public key point
fn compute_public_point(scalar: BigInt) -> Point {
	BASE_POINT * scalar
}

/// Computes the shared secret by scaling the other public key 
/// by this party's private scalar
fn compute_shared_secret_point(private: BigInt, other_public: Point) -> Point {
	other_public * private
}

/// Converts a ED25519Obj to a BigInt
fn obj_to_bn(obj: ED25519Obj) -> BigInt {
	let mut bytes = [0 ; bigint::BYTE_COUNT];
	for i in 0..KEY_BYTE_COUNT {
		bytes[i] = obj[i];
	}
	BigInt::from_le_bytes(bytes)
}

/// Converts a ED25519Obj to a Point
fn obj_to_pnt(obj: ED25519Obj) -> Point {
	Point::from(obj_to_bn(obj))
}

/// Converts a BigInt to an ED25519Obj, truncating extraneous data
fn bn_to_obj(bn: BigInt) -> ED25519Obj {
	let mut obj_bytes = [0 ; KEY_BYTE_COUNT];
	let bn_bytes = bn.to_le_bytes();
	for i in 0..KEY_BYTE_COUNT {
		obj_bytes[i] = bn_bytes[i];
	}
	obj_bytes
}

/// Converts a Point to an ED25519
fn pnt_to_obj(point: Point) -> ED25519Obj {
	bn_to_obj(point.affine_x().num)
}