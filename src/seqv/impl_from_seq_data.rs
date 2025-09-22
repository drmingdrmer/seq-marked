use crate::SeqData;
use crate::SeqV;

impl<M, T> From<SeqData<(Option<M>, T)>> for SeqV<M, T> {
    fn from(value: SeqData<(Option<M>, T)>) -> Self {
        let (seq, (meta, v)) = value.into_parts();

        SeqV::new_with_meta(seq, meta, v)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_seq_marked_with_meta() {
        let seq_marked = SeqData::new(42, (Some("metadata".to_string()), 100u64));
        let seqv: SeqV<String, u64> = seq_marked.into();

        assert_eq!(seqv.seq, 42);
        assert_eq!(seqv.meta, Some("metadata".to_string()));
        assert_eq!(seqv.data, 100);
    }

    #[test]
    fn test_from_seq_marked_without_meta() {
        let seq_marked = SeqData::new(10, (None::<String>, 200u64));
        let seqv: SeqV<String, u64> = seq_marked.into();

        assert_eq!(seqv.seq, 10);
        assert_eq!(seqv.meta, None);
        assert_eq!(seqv.data, 200);
    }
}
