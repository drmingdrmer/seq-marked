use std::fmt;

use crate::InternalSeq;
use crate::SeqMarked;

mod impl_from_seq_marked;

/// Sequence-numbered non-marked data.
///
/// It is a subset of SeqMarked where data is always normal.
///
/// ```rust
/// use seq_marked::SeqData;
///
/// let v1 = SeqData::new(1, "data");
/// ```
#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(PartialEq, Eq)]
#[derive(PartialOrd, Ord)]
#[cfg_attr(
    feature = "seq-marked-serde",
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(
    feature = "seq-marked-bincode",
    derive(bincode::Encode, bincode::Decode)
)]
pub struct SeqData<D = Vec<u8>> {
    // Keep the `seq` as the first field so that it can be compared first.
    seq: u64,
    data: D,
}

impl<D> SeqData<D> {
    /// Creates normal value with sequence number.
    pub fn new(seq: u64, data: D) -> Self {
        Self { seq, data }
    }

    /// Transforms data `D` to `U` while preserving sequence and tombstone state.
    ///
    /// # Example
    ///
    /// ```rust
    /// use seq_marked::SeqData;
    ///
    /// let a = SeqData::new(1, "data");
    /// let b = a.map(|x| x.len());
    /// assert_eq!(b.data(), &4);
    /// ```
    pub fn map<U>(self, f: impl FnOnce(D) -> U) -> SeqData<U> {
        SeqData {
            seq: self.seq,
            data: f(self.data),
        }
    }

    pub fn try_map<U, E>(self, f: impl FnOnce(D) -> Result<U, E>) -> Result<SeqData<U>, E> {
        Ok(SeqData {
            seq: self.seq,
            data: f(self.data)?,
        })
    }

    /// Creates reference to the data.
    pub fn as_ref(&self) -> SeqData<&D> {
        SeqData {
            seq: self.seq,
            data: &self.data,
        }
    }

    /// Returns ordering key (sequence + tombstone state only).
    pub fn order_key(&self) -> SeqMarked<()> {
        SeqMarked::new_normal(self.seq, ())
    }

    /// Returns the sequence number for internal use, tombstone also has a seq.
    pub fn internal_seq(&self) -> InternalSeq {
        InternalSeq::new(self.seq)
    }

    /// Returns the sequence number for application use, tombstone always has seq 0.
    pub fn user_seq(&self) -> u64 {
        self.seq
    }

    /// Returns the maximum of two values.
    pub fn max(a: Self, b: Self) -> Self {
        if a.order_key() > b.order_key() { a } else { b }
    }

    /// Returns the maximum of two values.
    pub fn max_ref<'l>(a: &'l Self, b: &'l Self) -> &'l Self {
        if a.order_key() > b.order_key() { a } else { b }
    }

    /// Returns reference to data if normal, `None` if tombstone.
    pub fn data(&self) -> &D {
        &self.data
    }

    /// Consumes and returns data if normal, `None` if tombstone.
    pub fn into_data(self) -> D {
        self.data
    }

    pub fn into_parts(self) -> (u64, D) {
        (self.seq, self.data)
    }

    /// Returns formatter for display using `Debug` trait.
    pub fn display_with_debug(&self) -> impl fmt::Display + '_
    where D: fmt::Debug {
        struct DisplaySeqData<'a, D>(&'a SeqData<D>);

        impl<D> fmt::Display for DisplaySeqData<'_, D>
        where D: fmt::Debug
        {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{{seq: {}, ", self.0.seq)?;
                write!(f, "({:?})", self.0.data)?;
                write!(f, "}}")
            }
        }

        DisplaySeqData(self)
    }
}

#[cfg(test)]
mod tests {
    use std::cmp::Ordering;

    use Ordering::Equal;
    use Ordering::Greater;
    use Ordering::Less;

    use super::*;
    use crate::testing::ts;

    /// Create a `SeqMarked::Normal`.
    pub(crate) fn norm<D>(seq: u64, d: D) -> SeqData<D> {
        SeqData::new(seq, d)
    }

    #[test]
    fn test_seq_data_is_copy() {
        let seq_marked = SeqData::new(5, "data");
        let seq_marked_copy = seq_marked;
        assert_eq!(seq_marked, seq_marked_copy);
    }

    #[test]
    fn test_new() {
        let seq_marked = SeqData::new(5, "data");
        assert_eq!(seq_marked.seq, 5);
        assert_eq!(seq_marked.data, "data");
    }

    #[test]
    fn test_map() -> anyhow::Result<()> {
        let a = norm(1, 1u64);
        assert_eq!(norm(1, 2u32), a.map(|x| (x * 2) as u32));

        let a = ts::<u64>(1);
        assert_eq!(ts::<u32>(1), a.map(|x| (x * 2) as u32));

        Ok(())
    }

    #[test]
    fn test_as_ref() -> anyhow::Result<()> {
        let a = norm(1, 1u64);
        assert_eq!(norm(1, &1u64), a.as_ref());

        let a = ts::<u64>(1);
        assert_eq!(ts::<&u64>(1), a.as_ref());

        Ok(())
    }

    #[test]
    fn test_order_key() -> anyhow::Result<()> {
        assert!(norm(1, 1u64).order_key() == norm(1, 1u64).order_key());
        assert!(norm(1, 2u64).order_key() == norm(1, 1u64).order_key());
        assert!(norm(2, 2u64).order_key() > norm(1, 1u64).order_key());

        assert!(ts::<u64>(1).order_key() > norm(1, 1u64).order_key());
        assert!(ts::<u64>(2).order_key() > norm(1, 1u64).order_key());

        assert!(ts::<u64>(2).order_key() > ts::<u64>(1).order_key());
        assert!(ts::<u64>(1).order_key() == ts::<u64>(1).order_key());

        Ok(())
    }

    #[test]
    fn test_partial_ord() -> anyhow::Result<()> {
        fn pcmp<D: PartialOrd>(a: &SeqData<D>, b: &SeqData<D>) -> Option<Ordering> {
            PartialOrd::partial_cmp(a, b)
        }

        // normal vs normal, with the same data

        assert_eq!(Some(Greater), pcmp(&norm(2, 2u64), &norm(1, 2u64)));
        assert_eq!(Some(Equal), pcmp(&norm(2, 2u64), &norm(2, 2u64)));
        assert_eq!(Some(Less), pcmp(&norm(2, 2u64), &norm(3, 2u64)));

        // normal vs normal, same seq, different value

        assert_eq!(Some(Greater), pcmp(&norm(2, 2u64), &norm(2, 1u64)));
        assert_eq!(Some(Equal), pcmp(&norm(2, 2u64), &norm(2, 2u64)));
        assert_eq!(Some(Less), pcmp(&norm(2, 2u64), &norm(2, 3u64)));

        Ok(())
    }

    #[test]
    fn test_seq_data_order_key() {
        let a = SeqData::new(1, "data1");

        assert_eq!(a.order_key(), SeqMarked::new_normal(1, ()));
    }

    #[test]
    fn test_max() {
        assert_eq!(
            SeqData::<u64>::new(2, 1),
            SeqData::<u64>::max(SeqData::<u64>::new(1, 1), SeqData::<u64>::new(2, 1))
        );
        assert_eq!(
            SeqData::<u64>::new(2, 1),
            SeqData::<u64>::max(SeqData::<u64>::new(1, 2), SeqData::<u64>::new(2, 1))
        );
    }

    #[test]
    fn test_max_ref() {
        let m1 = SeqData::new(1, 2);
        let m2 = SeqData::new(3, 2);

        assert_eq!(SeqData::max_ref(&m1, &m2), &m2);

        assert_eq!(SeqData::max_ref(&m1, &m1), &m1);
        assert_eq!(SeqData::max_ref(&m2, &m2), &m2);
    }

    #[test]
    fn test_into_parts() {
        let seq_marked = SeqData::new(5, "data");
        let (seq, marked) = seq_marked.into_parts();
        assert_eq!(seq, 5);
        assert_eq!(marked, "data");
    }

    #[test]
    fn test_internal_seq() {
        let seq_marked = SeqData::new(5, "data");
        assert_eq!(*seq_marked.internal_seq(), 5);
    }

    #[test]
    fn test_user_seq() {
        let seq_marked = SeqData::new(5, "data");
        assert_eq!(seq_marked.user_seq(), 5);
    }

    #[test]
    fn test_display_with_debug() {
        let seq_marked = SeqData::new(5, "data");
        assert_eq!(
            format!("{}", seq_marked.display_with_debug()),
            "{seq: 5, (\"data\")}"
        );
    }
}

#[cfg(test)]
#[cfg(feature = "seq-marked-bincode")]
mod tests_bincode {

    use super::*;
    use crate::testing::bincode_config;
    use crate::testing::test_bincode_decode;

    #[test]
    fn test_marked_bincode() {
        let a = SeqData::new(5, 1u64);
        let encoded = bincode::encode_to_vec(&a, bincode_config()).unwrap();
        let (decoded, n): (SeqData<u64>, usize) =
            bincode::decode_from_slice(&encoded, bincode_config()).unwrap();
        assert_eq!(n, 2);
        assert_eq!(a, decoded);
    }

    #[test]
    fn test_marked_bincode_decode_v010() -> anyhow::Result<()> {
        let value = SeqData::new(5, 1u64);
        let encoded = vec![5, 1];

        test_bincode_decode(&encoded, &value)?;

        Ok(())
    }
}

#[cfg(test)]
#[cfg(feature = "seq-marked-serde")]
mod tests_serde {
    use super::*;
    use crate::testing::test_serde_decode;

    #[test]
    fn test_marked_serde() {
        let a = SeqData::new(5, 1u64);
        let encoded = serde_json::to_string(&a).unwrap();
        let decoded: SeqData<u64> = serde_json::from_str(&encoded).unwrap();
        assert_eq!(a, decoded);
    }

    #[test]
    fn test_marked_serde_decode_v010() -> anyhow::Result<()> {
        let value = SeqData::new(5, 1u64);
        let encoded = r#"{"seq":5,"data":1}"#;

        test_serde_decode(encoded, &value)?;

        Ok(())
    }
}
