use blake3;

pub fn hash(input: &[u8]) -> u32 {
  hash_with_max(input, std::u32::MAX)
}

pub fn hash_with_max(input: &[u8], max: u32) -> u32 {
  let hash = blake3::hash(input);
  let hash = hash.as_bytes().to_vec();

  let be16_bytes = u16::from_be_bytes([hash[0], hash[1]]);
  let be32_bytes = u32::from_be_bytes([hash[0], hash[1], hash[2], hash[3]]);

  let num = u32::from(be16_bytes)
    .overflowing_mul(0xffffffff)
    .0
    .overflowing_add(be32_bytes)
    .0;

  num % max
}
