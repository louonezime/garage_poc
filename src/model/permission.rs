use std::cmp::Ordering;

use serde::{Deserialize, Serialize};

use garage_util::crdt::*;

/// Permission given to a key in a bucket
#[derive(PartialOrd, Ord, PartialEq, Eq, Clone, Copy, Debug, Serialize, Deserialize)]
pub struct BucketKeyPerm {
	/// Timestamp at which the permission was given
	pub timestamp: u64,

	/// The key can be used to read the bucket
	pub allow_read: bool,
	/// The key can be used to write objects to the bucket
	pub allow_write: bool,
	/// The key can be used to control other aspects of the bucket:
	/// - enable / disable website access
	/// - delete bucket
	pub allow_owner: bool,
}

impl Crdt for BucketKeyPerm {
	fn merge(&mut self, other: &Self) {
		match other.timestamp.cmp(&self.timestamp) {
			Ordering::Greater => {
				*self = *other;
			}
			Ordering::Equal if other != self => {
				warn!("Different permission sets with same timestamp: {:?} and {:?}, merging to most restricted permission set.", self, other);
				if !other.allow_read {
					self.allow_read = false;
				}
				if !other.allow_write {
					self.allow_write = false;
				}
			}
			_ => (),
		}
	}
}
