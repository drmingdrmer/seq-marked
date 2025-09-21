use crate::Marked;
use crate::SeqData;
use crate::SeqMarked;

impl<D> From<SeqMarked<D>> for Option<SeqData<D>> {
    fn from(value: SeqMarked<D>) -> Self {
        let (seq, marked) = value.into_parts();
        match marked {
            Marked::TombStone => None,
            Marked::Normal(d) => Some(SeqData::new(seq, d)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_seq_marked() {
        let seq_marked = SeqMarked::new_normal(42, "data".to_string());
        let seq_data: Option<SeqData<String>> = seq_marked.into();

        assert!(seq_data.is_some());
        let seq_data = seq_data.unwrap();
        assert_eq!(seq_data.seq, 42);
        assert_eq!(seq_data.data, "data".to_string());

        let seq_marked = SeqMarked::<String>::new_tombstone(5);
        let seq_data: Option<SeqData<String>> = seq_marked.into();

        assert!(seq_data.is_none());
    }
}
