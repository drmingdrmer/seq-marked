use crate::SeqData;
use crate::SeqV;

impl<M, T> From<SeqV<M, T>> for SeqData<(Option<M>, T)> {
    fn from(value: SeqV<M, T>) -> Self {
        SeqData::new(value.seq, (value.meta, value.data))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_seqv_with_meta() {
        let seqv = SeqV::new_with_meta(42, Some("metadata".to_string()), 100u64);
        let seq_data: SeqData<(Option<String>, u64)> = seqv.into();

        assert_eq!(*seq_data.internal_seq(), 42);
        assert_eq!(seq_data.data(), &(Some("metadata".to_string()), 100));
    }

    #[test]
    fn test_from_seqv_without_meta() {
        let seqv = SeqV::new_with_meta(10, None::<String>, 200u64);
        let seq_data: SeqData<(Option<String>, u64)> = seqv.into();

        assert_eq!(*seq_data.internal_seq(), 10);
        assert_eq!(seq_data.data(), &(None, 200));
    }

    #[test]
    fn test_from_seqv_basic_constructor() {
        let seqv = SeqV::new(5, vec![1, 2, 3]);
        let seq_data: SeqData<(Option<()>, Vec<i32>)> = seqv.into();

        assert_eq!(*seq_data.internal_seq(), 5);
        assert_eq!(seq_data.data(), &(None, vec![1, 2, 3]));
    }
}
