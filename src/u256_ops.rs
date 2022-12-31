use std::ops;
use std::cmp;
use cmp::Ordering;
use crate::u256::U256;

impl ops::Add<U256> for U256 {
	type Output = U256;

	fn add(self, rhs: U256) -> U256 {
		let (result, carry) = self.words[0].carrying_add(rhs.words[0], false);
		U256 { 
			words: [result, self.words[1].carrying_add(rhs.words[1], carry).0]
		}
    }
}

impl ops::Sub<U256> for U256 {
	type Output = U256;

	fn sub(self, rhs: U256) -> Self::Output {
		let (result, borrow) = self.words[0].borrowing_sub(rhs.words[0], false);
		U256 { 
			words: [result, self.words[1].borrowing_sub(rhs.words[1], borrow).0] 
		}
	}
}

impl ops::Mul<U256> for U256 {
	type Output = U256;

	fn mul(self, rhs: U256) -> Self::Output {
		
	}
}

impl ops::Div<U256> for U256 {
	type Output = U256;

	fn div(self, rhs: U256) -> Self::Output {
		
	}
}

impl ops::Rem<U256> for U256 {
	type Output = U256;

	fn rem(self, rhs: U256) -> Self::Output {
		
	}
}

impl cmp::PartialEq for U256 {
	fn eq(&self, other: &Self) -> bool {
		self.words == other.words
	}

	fn ne(&self, other: &Self) -> bool {
		self.words != other.words
	}
}

impl cmp::PartialOrd for U256 {
	fn ge(&self, other: &Self) -> bool {
		if self.words[1] == other.words[1] { self.words[0] >= other.words[0] }
		else { self.words[1] >= other.words[1] }
	}

	fn gt(&self, other: &Self) -> bool {
		if self.words[1] == other.words[1] { self.words[0] > other.words[0] }
		else { self.words[1] > other.words[1] }
	}

	fn le(&self, other: &Self) -> bool {
		if self.words[1] == other.words[1] { self.words[0] <= other.words[0] }
		else { self.words[1] <= other.words[1] }
	}

	fn lt(&self, other: &Self) -> bool {
		if self.words[1] == other.words[1] { self.words[0] < other.words[0] }
		else { self.words[1] < other.words[1] }
	}

	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		if self == other { Some(Ordering::Equal) }
		else if self < other { Some(Ordering::Less) }
		else { Some(Ordering::Greater) }
	}
}