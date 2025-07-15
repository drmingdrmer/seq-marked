use crate::SeqMarked;
use crate::SeqV;

impl<M, T> From<SeqV<M, T>> for SeqMarked<(Option<M>, T)> {
    fn from(value: SeqV<M, T>) -> Self {
        SeqMarked::new_normal(value.seq, (value.meta, value.data))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_seqv_with_meta() {
        let seqv = SeqV::new_with_meta(42, Some("metadata".to_string()), 100u64);
        let seq_marked: SeqMarked<(Option<String>, u64)> = seqv.into();

        assert_eq!(seq_marked.internal_seq(), 42);
        assert_eq!(
            seq_marked.data_ref(),
            Some(&(Some("metadata".to_string()), 100))
        );
        assert!(!seq_marked.is_tombstone());
    }

    #[test]
    fn test_from_seqv_without_meta() {
        let seqv = SeqV::new_with_meta(10, None::<String>, 200u64);
        let seq_marked: SeqMarked<(Option<String>, u64)> = seqv.into();

        assert_eq!(seq_marked.internal_seq(), 10);
        assert_eq!(seq_marked.data_ref(), Some(&(None, 200)));
        assert!(!seq_marked.is_tombstone());
    }

    #[test]
    fn test_from_seqv_basic_constructor() {
        let seqv = SeqV::new(5, vec![1, 2, 3]);
        let seq_marked: SeqMarked<(Option<()>, Vec<i32>)> = seqv.into();

        assert_eq!(seq_marked.internal_seq(), 5);
        assert_eq!(seq_marked.data_ref(), Some(&(None, vec![1, 2, 3])));
        assert!(seq_marked.is_normal());
    }
}
