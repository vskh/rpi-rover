use std::hash::{DefaultHasher, Hash, Hasher};

pub fn calc_hash<T: Hash>(value: T) -> u64 {
    let mut s = DefaultHasher::new();
    value.hash(&mut s);
    s.finish()
}