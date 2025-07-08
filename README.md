# seqmarked

Sequence-numbered values with tombstone support for LSM trees and versioned data.

## Core Types

- `Marked<D>`: Data that can be marked as tombstone
- `SeqMarked<D>`: Sequence-numbered marked value

## Usage

```rust
use seqmarked::{Marked, SeqMarked};

// Basic sequence-numbered values
let v1 = SeqMarked::new_normal(1, "data");
let v2 = SeqMarked::new_normal(2, "data");
let v2_ts = SeqMarked::<&str>::new_tombstone(2);

assert!(v1 < v2); // ordered by sequence
assert!(v2 < v2_ts); // tombstone > normal

// Working with marked values
let data = Marked::Normal("hello");
let tombstone = Marked::<&str>::TombStone;
assert!(tombstone > data);

// Accessing data
assert_eq!(v1.data_ref(), Some(&"data"));
assert_eq!(v2_ts.data_ref(), None);
assert!(v2_ts.is_tombstone());

// Transform data while preserving sequence
let lengths = v1.map(|s| s.len());
assert_eq!(lengths.data_ref(), Some(&4));
```

## API Overview

### `Marked<D>`
- `Normal(D)` - contains data
- `TombStone` - deletion marker
- Tombstones are always ordered after normal values

### `SeqMarked<D>`
- `new_normal(seq, data)` - create normal value
- `new_tombstone(seq)` - create tombstone
- `seq()` - get sequence number
- `data_ref()` / `into_data()` - access data
- `is_normal()` / `is_tombstone()` - check type
- `map(fn)` - transform data while preserving sequence
- `order_key()` - get ordering key without data

## Features

- Sequence-based ordering with tombstone support
- Optional serde/bincode serialization
- Comprehensive ordering semantics for LSM trees


## License

Apache License 2.0 - see [LICENSE](LICENSE) file for details.

## Contributing

Contributions welcome! Please ensure all tests pass and code is properly formatted before submitting a PR.
