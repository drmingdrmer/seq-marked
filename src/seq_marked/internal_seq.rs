use std::fmt;
use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Deref;
use std::ops::DerefMut;

/// Internal sequence number type.
///
/// Unlike `SeqV.seq`, where a tombstone always has a sequence number of 0,
/// an [`InternalSeq`] tombstone retains a positive sequence number.
#[derive(Debug)]
#[derive(Default)]
#[derive(Clone, Copy)]
#[derive(PartialEq, Eq)]
#[derive(PartialOrd, Ord)]
#[repr(transparent)]
pub struct InternalSeq {
    seq: u64,
}

impl Deref for InternalSeq {
    type Target = u64;

    fn deref(&self) -> &Self::Target {
        &self.seq
    }
}

impl DerefMut for InternalSeq {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.seq
    }
}

impl InternalSeq {
    pub const fn new(seq: u64) -> Self {
        Self { seq }
    }
}

impl fmt::Display for InternalSeq {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ISeq({})", self.seq)
    }
}

impl Add<u64> for InternalSeq {
    type Output = Self;

    fn add(self, rhs: u64) -> Self::Output {
        Self::new(self.seq + rhs)
    }
}

impl AddAssign<u64> for InternalSeq {
    fn add_assign(&mut self, rhs: u64) {
        self.seq += rhs;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_operations() {
        let seq = InternalSeq::new(42);
        assert_eq!(*seq, 42);

        let default_seq = InternalSeq::default();
        assert_eq!(*default_seq, 0);

        let copied = seq;
        assert_eq!(seq, copied);
    }

    #[test]
    fn test_comparison() {
        let seq1 = InternalSeq::new(10);
        let seq2 = InternalSeq::new(20);

        assert_eq!(seq1, InternalSeq::new(10));
        assert_ne!(seq1, seq2);
        assert!(seq1 < seq2);
    }

    #[test]
    fn test_deref_mutation() {
        let mut seq = InternalSeq::new(42);
        assert_eq!(*seq, 42);

        *seq = 100;
        assert_eq!(*seq, 100);
    }

    #[test]
    fn test_transparent_layout() {
        assert_eq!(
            std::mem::size_of::<InternalSeq>(),
            std::mem::size_of::<u64>()
        );
    }

    #[test]
    fn test_display() {
        let seq = InternalSeq::new(42);
        assert_eq!(format!("{}", seq), "ISeq(42)");
    }

    #[test]
    fn test_add_assign() {
        let mut seq = InternalSeq::new(42);
        seq += 10;
        assert_eq!(seq, InternalSeq::new(52));
    }

    #[test]
    fn test_add() {
        let seq = InternalSeq::new(42);
        let result = seq + 10;
        assert_eq!(result, InternalSeq::new(52));
        assert_eq!(seq, InternalSeq::new(42)); // Original unchanged
    }
}
