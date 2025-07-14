mod impl_from_seq_marked;
mod impl_seq_value;

use std::fmt;
use std::ops::Deref;
use std::ops::DerefMut;

/// Some value bound with a seq number.
///
/// [`SeqV`] is the meta-service API level generic value.
/// Meta-service application uses this type to interact with meta-service.
///
/// Inside the meta-service, the value is stored in the form of `Marked`, which could be a
/// tombstone. A `Marked::TombStone` is converted to `None::<SeqV>` and a `Marked::Normal` is
/// converted to `Some::<SeqV>`.
///
/// A `Marked::TombStone` also has an `internal_seq`, representing the freshness of the tombstone.
/// `internal_seq` will be discarded when `Marked::TombStone` is converted to `None::<SeqV>`.
#[derive(Default, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "seqv-serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SeqV<M, T = Vec<u8>> {
    pub seq: u64,
    pub meta: Option<M>,
    pub data: T,
}

impl<M, T> Deref for SeqV<M, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<M, T> DerefMut for SeqV<M, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

impl<M, T> fmt::Debug for SeqV<M, T>
where
    M: fmt::Debug,
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut de = f.debug_struct("SeqV");
        de.field("seq", &self.seq);
        de.field("meta", &self.meta);
        de.field("data", &"[binary]");

        de.finish()
    }
}

impl<M, T> SeqV<M, T> {
    pub fn new(seq: u64, data: T) -> Self {
        Self {
            seq,
            meta: None,
            data,
        }
    }

    pub fn new_with_meta(seq: u64, meta: Option<M>, data: T) -> Self {
        Self { seq, meta, data }
    }

    #[must_use]
    pub fn with_seq(mut self, seq: u64) -> Self {
        self.seq = seq;
        self
    }

    #[must_use]
    pub fn with_meta(mut self, m: Option<M>) -> Self {
        self.meta = m;
        self
    }

    #[must_use]
    pub fn with_value(mut self, v: T) -> Self {
        self.data = v;
        self
    }

    /// Convert data to type U and leave seq and meta unchanged.
    pub fn map<U>(self, f: impl FnOnce(T) -> U) -> SeqV<M, U> {
        SeqV {
            seq: self.seq,
            meta: self.meta,
            data: f(self.data),
        }
    }

    /// Try to convert data to type U and leave seq and meta unchanged.
    /// `f` returns an error if the conversion fails.
    pub fn try_map<U, E>(self, f: impl FnOnce(T) -> Result<U, E>) -> Result<SeqV<M, U>, E> {
        Ok(SeqV {
            seq: self.seq,
            meta: self.meta,
            data: f(self.data)?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let sv = SeqV::<(), _>::new(42, 100u64);
        assert_eq!(sv.seq, 42);
        assert_eq!(sv.meta, None);
        assert_eq!(sv.data, 100);
    }

    #[test]
    fn test_new_with_meta() {
        let sv = SeqV::new_with_meta(10, Some("metadata"), 200u64);
        assert_eq!(sv.seq, 10);
        assert_eq!(sv.meta, Some("metadata"));
        assert_eq!(sv.data, 200);

        let sv_none = SeqV::new_with_meta(5, None::<String>, 300u64);
        assert_eq!(sv_none.meta, None);
    }

    #[test]
    fn test_builder_methods() {
        let sv = SeqV::new(1, 100u64).with_seq(42).with_meta(Some("test")).with_value(200u64);

        assert_eq!(sv.seq, 42);
        assert_eq!(sv.meta, Some("test"));
        assert_eq!(sv.data, 200);
    }

    #[test]
    fn test_map() {
        let sv = SeqV::new_with_meta(10, Some("meta"), 5u64);
        let mapped = sv.map(|x| x * 2);

        assert_eq!(mapped.seq, 10);
        assert_eq!(mapped.meta, Some("meta"));
        assert_eq!(mapped.data, 10u64);
    }

    #[test]
    fn test_try_map_success() {
        let sv = SeqV::new_with_meta(20, Some("meta"), "123");
        let result = sv.try_map(|s| s.parse::<u64>());

        assert!(result.is_ok());
        let mapped = result.unwrap();
        assert_eq!(mapped.seq, 20);
        assert_eq!(mapped.meta, Some("meta"));
        assert_eq!(mapped.data, 123u64);
    }

    #[test]
    fn test_try_map_error() {
        let sv = SeqV::<(), _>::new(30, "invalid");
        let result = sv.try_map(|s| s.parse::<u64>());

        assert!(result.is_err());
    }

    #[test]
    fn test_deref() {
        let sv = SeqV::<(), _>::new(1, vec![1, 2, 3]);
        assert_eq!(sv.len(), 3);
        assert_eq!(sv[0], 1);
    }

    #[test]
    fn test_deref_mut() {
        let mut sv = SeqV::<(), _>::new(1, vec![1, 2, 3]);
        sv[0] = 10;
        assert_eq!(sv.data[0], 10);
    }

    #[test]
    fn test_default() {
        let sv = SeqV::<String, u64>::default();
        assert_eq!(sv.seq, 0);
        assert_eq!(sv.meta, None);
        assert_eq!(sv.data, 0);
    }

    #[test]
    fn test_debug() {
        let sv = SeqV::new_with_meta(42, Some("test"), vec![1, 2, 3]);
        assert_eq!(
            "SeqV { seq: 42, meta: Some(\"test\"), data: \"[binary]\" }",
            format!("{:?}", sv)
        );
    }

    #[test]
    fn test_clone_eq() {
        let sv1 = SeqV::new_with_meta(10, Some("meta"), 100u64);
        let sv2 = sv1.clone();
        assert_eq!(sv1, sv2);
    }
}

#[cfg(test)]
#[cfg(feature = "seqv-serde")]
mod tests_serde {
    use serde_json;

    use super::*;

    #[test]
    fn test_serde() {
        let sv = SeqV::new_with_meta(42, Some("metadata".to_string()), vec![1, 2, 3]);
        let json = serde_json::to_string(&sv).unwrap();
        let deserialized: SeqV<String, Vec<u8>> = serde_json::from_str(&json).unwrap();
        assert_eq!(sv, deserialized);

        // Test with None metadata
        let sv_none = SeqV::new_with_meta(10, None::<String>, vec![4, 5, 6]);
        let json_none = serde_json::to_string(&sv_none).unwrap();
        let deserialized_none: SeqV<String, Vec<u8>> = serde_json::from_str(&json_none).unwrap();
        assert_eq!(sv_none, deserialized_none);
    }
}
