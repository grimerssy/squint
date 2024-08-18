pub const fn tag(tag: &str) -> u64 {
    const_fnv1a_hash::fnv1a_hash_str_64(tag)
}
