extern crate hash_index;

#[test]
fn hasher() {
  assert_eq!(hash_index::hash(b"foobar"), 2857448067);
  assert_eq!(
    hash_index::hash(b"The quick brown fox jumps over the lazy dog"),
    789898499
  );
  assert_eq!(hash_index::hash_with_max(b"foobar", 100), 67);
  assert_eq!(
    hash_index::hash_with_max(b"The quick brown fox jumps over the lazy dog", 100),
    99
  );
}
