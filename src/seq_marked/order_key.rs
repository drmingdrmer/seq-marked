//! Implement the `SeqMarked<()>` which is used as an order key.

use crate::Marked;
use crate::seq_marked::SeqMarked;

impl SeqMarked<()> {
    /// Creates the smallest order key (seq=0, normal).
    pub const fn zero() -> Self {
        Self {
            seq: 0,
            marked: Marked::Normal(()),
        }
    }

    pub const fn max_value() -> Self {
        Self {
            seq: u64::MAX,
            marked: Marked::TombStone,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Marked;

    #[test]
    fn test_zero() {
        let zero = SeqMarked::<()>::zero();
        assert_eq!(zero.seq, 0);
        assert_eq!(zero.marked, Marked::Normal(()));
    }

    #[test]
    fn test_max() {
        let max = SeqMarked::<()>::max_value();
        assert_eq!(max.seq, u64::MAX);
        assert!(max.is_tombstone());
    }
}
