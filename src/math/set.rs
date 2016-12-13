use std::collections::HashSet;
use std::collections::hash_set::Iter;
use std::collections::hash_map::RandomState;
use std::hash::Hash;
use std::ops::Mul;

pub struct Set<T: Hash + Eq, S = RandomState> { set: HashSet<T, S> }
impl<T> Set<T> where T: Hash + Eq {
	pub fn new() -> Set<T> {
		Set {
			set: HashSet::<T>::new()
		}
	}

	pub fn insert(&mut self, a: T) {
		self.set.insert(a);
	}

	pub fn insert_all(&mut self, a: HashSet<T>) {
		for item in a {
			self.set.insert(item);
		}
	}

	pub fn iter(&self) -> Iter<T> {
		self.set.iter()
	}

	pub fn len(&self) -> usize {
		self.set.len()
	}
}

impl<T> Mul<Set<T>> for Set<T> where T: Hash + Eq + Copy {
	type Output = Set<(T,T)>;
	fn mul(self, rhs: Set<T>) -> Set<(T,T)> {
		let mut result = Set::<(T, T)>::new();
		for a in self.iter() {
			for b in rhs.iter() {
				result.insert((*a, *b));
			}
		}
		result
	}
}

#[macro_export]
macro_rules! set {
	( $( $x:expr ),* ) => {
		{
			let mut tmp = Set::new();
			$(
				tmp.insert($x);
			)*
			tmp
		}
	};
}
