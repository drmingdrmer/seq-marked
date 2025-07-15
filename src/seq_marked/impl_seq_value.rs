use crate::SeqMarked;
use crate::SeqValue;

/// [`SeqMarked`] is a superset of [`SeqValue`].
impl<M, T> SeqValue<M, T> for SeqMarked<(Option<M>, T)> {
    fn seq(&self) -> u64 {
        self.user_seq()
    }

    fn value(&self) -> Option<&T> {
        self.data_ref().map(|(_meta, value)| value)
    }

    fn into_value(self) -> Option<T> {
        self.into_data().map(|(_meta, value)| value)
    }

    fn meta(&self) -> Option<&M> {
        self.data_ref().and_then(|(meta, _value)| meta.as_ref())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_seq_value_with_meta() {
        let sv = SeqMarked::new_normal(42, (Some("metadata"), 100u64));

        assert_eq!(sv.seq(), 42);
        assert_eq!(sv.value(), Some(&100));
        assert_eq!(sv.meta(), Some(&"metadata"));

        let (seq, value) = sv.unpack();
        assert_eq!(seq, 42);
        assert_eq!(value, Some(100));
    }

    #[test]
    fn test_seq_value_without_meta() {
        let sv = SeqMarked::new_normal(10, (None::<String>, 200u64));

        assert_eq!(sv.seq(), 10);
        assert_eq!(sv.value(), Some(&200));
        assert_eq!(sv.meta(), None);

        let (seq, value) = sv.unpack();
        assert_eq!(seq, 10);
        assert_eq!(value, Some(200));
    }

    #[test]
    fn test_seq_value_tombstone() {
        let sv = SeqMarked::<(Option<String>, u64)>::new_tombstone(5);

        assert_eq!(sv.seq(), 0);
        assert_eq!(sv.value(), None);
        assert_eq!(sv.meta(), None);

        let (seq, value) = sv.unpack();
        assert_eq!(seq, 0);
        assert_eq!(value, None);
    }
}
