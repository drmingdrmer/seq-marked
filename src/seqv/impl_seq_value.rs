//! Implement [`SeqValue`] for [`SeqV`]

use super::SeqV;
use crate::SeqValue;

impl<M, V> SeqValue<M, V> for SeqV<M, V> {
    fn seq(&self) -> u64 {
        self.seq
    }

    fn value(&self) -> Option<&V> {
        Some(&self.data)
    }

    fn into_value(self) -> Option<V> {
        Some(self.data)
    }

    fn meta(&self) -> Option<&M> {
        self.meta.as_ref()
    }
}

impl<M, V> SeqValue<M, V> for Option<SeqV<M, V>> {
    fn seq(&self) -> u64 {
        self.as_ref().map(|v| v.seq()).unwrap_or(0)
    }

    fn value(&self) -> Option<&V> {
        self.as_ref().and_then(|v| v.value())
    }

    fn into_value(self) -> Option<V> {
        self.map(|v| v.data)
    }

    fn meta(&self) -> Option<&M> {
        self.as_ref().and_then(|v| v.meta())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_seqv_with_meta() {
        let sv = SeqV {
            seq: 42,
            meta: Some("metadata"),
            data: 100u64,
        };

        assert_eq!(sv.seq(), 42);
        assert_eq!(sv.value(), Some(&100));
        assert_eq!(sv.meta(), Some(&"metadata"));

        let (seq, value) = sv.unpack();
        assert_eq!(seq, 42);
        assert_eq!(value, Some(100));
    }

    #[test]
    fn test_seqv_without_meta() {
        let sv = SeqV {
            seq: 10,
            meta: None::<String>,
            data: 200u64,
        };

        assert_eq!(sv.seq(), 10);
        assert_eq!(sv.value(), Some(&200));
        assert_eq!(sv.meta(), None);

        let (seq, value) = sv.unpack();
        assert_eq!(seq, 10);
        assert_eq!(value, Some(200));
    }

    #[test]
    fn test_option_seqv_some() {
        let sv = Some(SeqV {
            seq: 5,
            meta: Some("test"),
            data: 300u64,
        });

        assert_eq!(sv.seq(), 5);
        assert_eq!(sv.value(), Some(&300));
        assert_eq!(sv.meta(), Some(&"test"));

        let (seq, value) = sv.unpack();
        assert_eq!(seq, 5);
        assert_eq!(value, Some(300));
    }

    #[test]
    fn test_option_seqv_none() {
        let sv = None::<SeqV<String, u64>>;

        assert_eq!(sv.seq(), 0);
        assert_eq!(sv.value(), None);
        assert_eq!(sv.meta(), None);

        let (seq, value) = sv.unpack();
        assert_eq!(seq, 0);
        assert_eq!(value, None);
    }
}
