//! # seqmarked
//!
//! Sequence-numbered values with tombstone support for LSM trees and versioned data.
//!
//! ## Core Types
//!
//! - [`Marked<D>`]: Data that can be marked as tombstone
//! - [`SeqMarked<D>`]: Sequence-numbered marked value
//!
//! ## Example
//!
//! ```rust
//! use seqmarked::SeqMarked;
//!
//! let v1 = SeqMarked::new_normal(1, "data");
//! let v2 = SeqMarked::new_normal(2, "data");
//! let v2_ts = SeqMarked::<&str>::new_tombstone(2);
//!
//! assert!(v1 < v2); // ordered by sequence
//! assert!(v2 < v2_ts); // ordered by tombstone > normal
//! ```

mod marked;
mod seq_marked;

#[cfg(test)]
pub(crate) mod testing;

pub use marked::Marked;
pub use seq_marked::SeqMarked;
