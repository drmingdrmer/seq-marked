mod impl_display;
mod impl_from_seqv;
mod impl_seq_value;
mod impl_try_from_meta_bytes;
mod internal_seq;
mod order_key;
mod ref_seq_marked;

use std::fmt;

use crate::Marked;
use crate::seq_marked::internal_seq::InternalSeq;

/// Sequence-numbered marked value.
///
/// Ordered by sequence number first, then tombstone > normal.
///
/// ```rust
/// use seq_marked::SeqMarked;
///
/// let v1 = SeqMarked::new_normal(1, "data");
/// let v2 = SeqMarked::<&str>::new_tombstone(2);
/// assert!(v1 < v2);
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
pub struct SeqMarked<D = Vec<u8>> {
    // Keep the `seq` as the first field so that it can be compared first.
    seq: u64,
    marked: Marked<D>,
}

impl<D> SeqMarked<D> {
    /// Creates a new `SeqMarked` with sequence number and marked data.
    pub fn new(seq: u64, marked: Marked<D>) -> Self {
        Self { seq, marked }
    }

    /// Creates normal value with sequence number.
    pub fn new_normal(seq: u64, data: D) -> Self {
        Self {
            seq,
            marked: Marked::Normal(data),
        }
    }

    /// Creates tombstone with sequence number.
    pub fn new_tombstone(seq: u64) -> Self {
        Self {
            seq,
            marked: Marked::TombStone,
        }
    }

    /// Represents an absent record (not even marked as deleted).
    pub fn new_not_found() -> Self {
        Self {
            seq: 0,
            marked: Marked::TombStone,
        }
    }

    /// Returns `true` if this is normal data.
    pub fn is_normal(&self) -> bool {
        !self.is_tombstone()
    }

    /// Returns `true` if this is a tombstone.
    pub fn is_tombstone(&self) -> bool {
        match self.marked {
            Marked::Normal(_) => false,
            Marked::TombStone => true,
        }
    }

    pub fn is_not_found(&self) -> bool {
        self.is_absent()
    }

    pub fn is_absent(&self) -> bool {
        self.seq == 0 && self.is_tombstone()
    }

    /// Transforms data `D` to `U` while preserving sequence and tombstone state.
    ///
    /// # Example
    ///
    /// ```rust
    /// use seq_marked::SeqMarked;
    ///
    /// let a = SeqMarked::new_normal(1, "data");
    /// let b = a.map(|x| x.len());
    /// assert_eq!(b.data_ref(), Some(&4));
    /// ```
    pub fn map<U>(self, f: impl FnOnce(D) -> U) -> SeqMarked<U> {
        SeqMarked {
            seq: self.seq,
            marked: match self.marked {
                Marked::Normal(data) => Marked::<U>::Normal(f(data)),
                Marked::TombStone => Marked::<U>::TombStone,
            },
        }
    }

    pub fn try_map<U, E>(self, f: impl FnOnce(D) -> Result<U, E>) -> Result<SeqMarked<U>, E> {
        Ok(SeqMarked {
            seq: self.seq,
            marked: match self.marked {
                Marked::Normal(data) => Marked::<U>::Normal(f(data)?),
                Marked::TombStone => Marked::<U>::TombStone,
            },
        })
    }

    /// Creates reference to the data.
    pub fn as_ref(&self) -> SeqMarked<&D> {
        SeqMarked {
            seq: self.seq,
            marked: match &self.marked {
                Marked::Normal(data) => Marked::Normal(data),
                Marked::TombStone => Marked::TombStone,
            },
        }
    }

    /// Returns ordering key (sequence + tombstone state only).
    pub fn order_key(&self) -> SeqMarked<()> {
        SeqMarked {
            seq: self.seq,
            marked: match &self.marked {
                Marked::Normal(_) => Marked::Normal(()),
                Marked::TombStone => Marked::TombStone,
            },
        }
    }

    /// Returns the sequence number for internal use, tombstone also has a seq.
    pub fn internal_seq(&self) -> InternalSeq {
        InternalSeq::new(self.seq)
    }

    /// Returns the sequence number for application use, tombstone always has seq 0.
    pub fn user_seq(&self) -> u64 {
        if self.is_tombstone() { 0 } else { self.seq }
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
    pub fn data_ref(&self) -> Option<&D> {
        match self.marked {
            Marked::Normal(ref d) => Some(d),
            Marked::TombStone => None,
        }
    }

    /// Consumes and returns data if normal, `None` if tombstone.
    pub fn into_data(self) -> Option<D> {
        match self.marked {
            Marked::Normal(data) => Some(data),
            Marked::TombStone => None,
        }
    }

    pub fn into_parts(self) -> (u64, Marked<D>) {
        (self.seq, self.marked)
    }

    /// Returns formatter for display using `Debug` trait.
    pub fn display_with_debug(&self) -> impl fmt::Display + '_
    where D: fmt::Debug {
        struct DisplaySeqMarked<'a, D>(&'a SeqMarked<D>);

        impl<D> fmt::Display for DisplaySeqMarked<'_, D>
        where D: fmt::Debug
        {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{{seq: {}, ", self.0.seq)?;
                match &self.0.marked {
                    Marked::Normal(data) => write!(f, "({:?})", data)?,
                    Marked::TombStone => write!(f, "TOMBSTONE")?,
                }
                write!(f, "}}")
            }
        }

        DisplaySeqMarked(self)
    }
}

#[cfg(test)]
mod tests {
    use std::cmp::Ordering;

    use Ordering::Equal;
    use Ordering::Greater;
    use Ordering::Less;

    use super::*;
    use crate::testing::norm;
    use crate::testing::ts;

    #[test]
    fn test_seq_marked_is_copy() {
        let seq_marked = SeqMarked::new(5, Marked::Normal("data"));
        let seq_marked_copy = seq_marked;
        assert_eq!(seq_marked, seq_marked_copy);
    }

    #[test]
    fn test_new() {
        let seq_marked = SeqMarked::new(5, Marked::Normal("data"));
        assert_eq!(seq_marked.seq, 5);
        assert_eq!(seq_marked.marked, Marked::Normal("data"));
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
        fn pcmp<D: PartialOrd>(a: &SeqMarked<D>, b: &SeqMarked<D>) -> Option<Ordering> {
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

        // normal vs tombstone

        assert_eq!(Some(Greater), pcmp(&norm(2, 2u64), &ts(1)));
        assert_eq!(
            Some(Less),
            pcmp(&norm(2, 2u64), &ts(2)),
            "tombstone is greater than a normal with the same seq"
        );
        assert_eq!(Some(Less), pcmp(&norm(2, 2u64), &ts(3)));

        // tombstone vs normal

        assert_eq!(Some(Less), pcmp(&ts(1), &norm(2, 2u64)));
        assert_eq!(
            Some(Greater),
            pcmp(&ts(2), &norm(2, 2u64)),
            "tombstone is greater than a normal with the same seq"
        );
        assert_eq!(Some(Greater), pcmp(&ts(3), &norm(2, 2u64)));

        // tombstone vs tombstone

        assert_eq!(Some(Greater), pcmp(&ts::<()>(2), &ts(1)));
        assert_eq!(Some(Equal), pcmp(&ts::<()>(2), &ts(2)));
        assert_eq!(Some(Less), pcmp(&ts::<()>(2), &ts(3)));
        Ok(())
    }

    #[test]
    fn test_ord_operator() -> anyhow::Result<()> {
        // normal vs normal, with the same data

        assert!(norm(2, 2u64) > norm(1, 2u64));
        assert!(norm(2, 2u64) >= norm(1, 2u64));
        assert!(norm(2, 2u64) == norm(2, 2u64));
        assert!(norm(2, 2u64) <= norm(3, 2u64));
        assert!(norm(2, 2u64) < norm(3, 2u64));

        // normal vs normal, same seq, different value

        assert!(norm(2, 2u64) > norm(2, 1u64));
        assert!(norm(2, 2u64) >= norm(2, 1u64));
        assert!(norm(2, 2u64) == norm(2, 2u64));
        assert!(norm(2, 2u64) <= norm(2, 3u64));
        assert!(norm(2, 2u64) < norm(2, 3u64));

        // normal vs tombstone

        assert!(norm(2, 2u64) > ts(1));
        assert!(norm(2, 2u64) >= ts(1));
        assert!(
            norm(2, 2u64) < ts(2),
            "tombstone is greater than a normal with the same seq"
        );
        assert!(
            norm(2, 2u64) <= ts(2),
            "tombstone is greater than a normal with the same seq"
        );
        assert!(norm(2, 2u64) < ts(3));
        assert!(norm(2, 2u64) <= ts(3));

        // tombstone vs normal

        assert!(ts(1) < norm(2, 2u64));
        assert!(ts(1) <= norm(2, 2u64));
        assert!(
            ts(2) > norm(2, 2u64),
            "tombstone is greater than a normal with the same seq"
        );
        assert!(
            ts(2) >= norm(2, 2u64),
            "tombstone is greater than a normal with the same seq"
        );
        assert!(ts(3) > norm(2, 2u64));
        assert!(ts(3) >= norm(2, 2u64));

        // tombstone vs tombstone

        assert!(ts::<()>(2) > ts(1));
        assert!(ts::<()>(2) >= ts(1));
        assert!(ts::<()>(2) >= ts(2));
        assert!(ts::<()>(2) == ts(2));
        assert!(ts::<()>(2) <= ts(2));
        assert!(ts::<()>(2) <= ts(3));
        assert!(ts::<()>(2) < ts(3));

        Ok(())
    }

    #[test]
    fn test_new_absent() {
        let absent = SeqMarked::<u64>::new_not_found();
        assert_eq!(absent.seq, 0);
        assert!(absent.is_tombstone());
    }

    #[test]
    fn test_max() {
        assert_eq!(
            SeqMarked::<u64>::new_normal(2, 1),
            SeqMarked::<u64>::max(
                SeqMarked::<u64>::new_normal(1, 1),
                SeqMarked::<u64>::new_normal(2, 1)
            )
        );
        assert_eq!(
            SeqMarked::<u64>::new_normal(2, 1),
            SeqMarked::<u64>::max(
                SeqMarked::<u64>::new_normal(1, 2),
                SeqMarked::<u64>::new_normal(2, 1)
            )
        );
        assert_eq!(
            SeqMarked::<u64>::new_tombstone(2),
            SeqMarked::<u64>::max(
                SeqMarked::<u64>::new_normal(2, 1),
                SeqMarked::<u64>::new_tombstone(2)
            )
        );
        assert_eq!(
            SeqMarked::<u64>::new_tombstone(2),
            SeqMarked::<u64>::max(
                SeqMarked::<u64>::new_tombstone(1),
                SeqMarked::<u64>::new_tombstone(2)
            )
        );
    }

    #[test]
    fn test_max_ref() {
        let m1 = SeqMarked::new_normal(1, 2);
        let m2 = SeqMarked::new_normal(3, 2);
        let m3 = SeqMarked::new_tombstone(2);

        assert_eq!(SeqMarked::max_ref(&m1, &m2), &m2);
        assert_eq!(SeqMarked::max_ref(&m1, &m3), &m3);
        assert_eq!(SeqMarked::max_ref(&m2, &m3), &m2);

        assert_eq!(SeqMarked::max_ref(&m1, &m1), &m1);
        assert_eq!(SeqMarked::max_ref(&m2, &m2), &m2);
        assert_eq!(SeqMarked::max_ref(&m3, &m3), &m3);
    }

    #[test]
    fn test_is_not_found() {
        assert!(SeqMarked::<u64>::new_not_found().is_not_found());
        assert!(SeqMarked::<u64>::new_tombstone(0).is_not_found());
        assert!(!SeqMarked::<u64>::new_tombstone(1).is_not_found());
        assert!(!SeqMarked::<u64>::new_normal(1, 1).is_not_found());
    }

    #[test]
    fn test_into_parts() {
        let seq_marked = SeqMarked::new_normal(5, "data");
        let (seq, marked) = seq_marked.into_parts();
        assert_eq!(seq, 5);
        assert_eq!(marked, Marked::Normal("data"));
    }

    #[test]
    fn test_internal_seq() {
        let seq_marked = SeqMarked::new_normal(5, "data");
        assert_eq!(*seq_marked.internal_seq(), 5);

        let seq_marked_tombstone = SeqMarked::<u64>::new_tombstone(10);
        assert_eq!(*seq_marked_tombstone.internal_seq(), 10);
    }

    #[test]
    fn test_user_seq() {
        let seq_marked = SeqMarked::new_normal(5, "data");
        assert_eq!(seq_marked.user_seq(), 5);

        let seq_marked_tombstone = SeqMarked::<u64>::new_tombstone(10);
        assert_eq!(seq_marked_tombstone.user_seq(), 0);
    }

    #[test]
    fn test_display_with_debug() {
        let seq_marked = SeqMarked::new_normal(5, "data");
        assert_eq!(
            format!("{}", seq_marked.display_with_debug()),
            "{seq: 5, (\"data\")}"
        );
    }

    #[test]
    fn test_display_with_debug_tombstone() {
        let seq_marked = SeqMarked::<u64>::new_tombstone(5);
        assert_eq!(
            format!("{}", seq_marked.display_with_debug()),
            "{seq: 5, TOMBSTONE}"
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
        let a = SeqMarked::new_normal(5, 1u64);
        let encoded = bincode::encode_to_vec(&a, bincode_config()).unwrap();
        let (decoded, n): (SeqMarked<u64>, usize) =
            bincode::decode_from_slice(&encoded, bincode_config()).unwrap();
        assert_eq!(n, 3);
        assert_eq!(a, decoded);
    }

    #[test]
    fn test_marked_bincode_decode_v010() -> anyhow::Result<()> {
        let value = SeqMarked::new_normal(5, 1u64);
        let encoded = vec![5, 0, 1];

        test_bincode_decode(&encoded, &value)?;

        let value = SeqMarked::<u64>::new_tombstone(6);
        let encoded = vec![6, 1];

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
        let a = SeqMarked::new_normal(5, 1u64);
        let encoded = serde_json::to_string(&a).unwrap();
        let decoded: SeqMarked<u64> = serde_json::from_str(&encoded).unwrap();
        assert_eq!(a, decoded);
    }

    #[test]
    fn test_marked_serde_decode_v010() -> anyhow::Result<()> {
        let value = SeqMarked::new_normal(5, 1u64);
        let encoded = r#"{"seq":5,"marked":{"Normal":1}}"#;

        test_serde_decode(encoded, &value)?;

        let value = SeqMarked::<u64>::new_tombstone(6);
        let encoded = r#"{"seq":6,"marked":"TombStone"}"#;

        test_serde_decode(encoded, &value)?;
        Ok(())
    }
}
