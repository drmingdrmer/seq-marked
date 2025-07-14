use crate::Marked;
use crate::SeqMarked;
use crate::SeqV;

impl<M, T> From<SeqMarked<(Option<M>, T)>> for Option<SeqV<M, T>> {
    fn from(value: SeqMarked<(Option<M>, T)>) -> Self {
        let (seq, marked) = value.into_parts();

        match marked {
            Marked::TombStone => None,
            Marked::Normal((meta, v)) => Some(SeqV::new_with_meta(seq, meta, v)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_seq_marked_with_meta() {
        let seq_marked = SeqMarked::new_normal(42, (Some("metadata".to_string()), 100u64));
        let seqv: Option<SeqV<String, u64>> = seq_marked.into();

        assert!(seqv.is_some());
        let seqv = seqv.unwrap();
        assert_eq!(seqv.seq, 42);
        assert_eq!(seqv.meta, Some("metadata".to_string()));
        assert_eq!(seqv.data, 100);
    }

    #[test]
    fn test_from_seq_marked_without_meta() {
        let seq_marked = SeqMarked::new_normal(10, (None::<String>, 200u64));
        let seqv: Option<SeqV<String, u64>> = seq_marked.into();

        assert!(seqv.is_some());
        let seqv = seqv.unwrap();
        assert_eq!(seqv.seq, 10);
        assert_eq!(seqv.meta, None);
        assert_eq!(seqv.data, 200);
    }

    #[test]
    fn test_from_seq_marked_tombstone() {
        let seq_marked = SeqMarked::<(Option<String>, u64)>::new_tombstone(5);
        let seqv: Option<SeqV<String, u64>> = seq_marked.into();

        assert!(seqv.is_none());
    }
}
