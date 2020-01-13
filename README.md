# hash-index

Predictable hash indexing with a possible maximum value. Rust adaptation of
[watson/hash-index](https://github.com/watson/hash-index) using [Blake3](https://github.com/BLAKE3-team/BLAKE3)

## Usage

```rust
use hash_index;

let hash = hash_index::hash(b"foobar"); // 2857448067
let hash_with_max = hash_index::hash_with_max(b"foobar", 100); // 67
```

## See Also

- [watson/hash-index](https://github.com/watson/hash-index).
