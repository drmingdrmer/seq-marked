use crate::SeqData;
use crate::SeqMarked;

impl<D> From<SeqData<D>> for SeqMarked<D> {
    fn from(value: SeqData<D>) -> Self {
        let (seq, data) = value.into_parts();
        SeqMarked::new_normal(seq, data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Marked;

    #[test]
    fn test_from_seq_data() {
        let seq_data = SeqData::new(42, "data".to_string());
        let seq_marked: SeqMarked<String> = seq_data.into();

        assert_eq!(*seq_marked.internal_seq(), 42);
        assert_eq!(seq_marked.marked, Marked::Normal("data".to_string()));
    }
}
