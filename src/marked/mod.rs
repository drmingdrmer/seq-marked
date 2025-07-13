mod impl_try_from_meta_bytes;

/// Data that can be marked as tombstone.
///
/// Tombstones are always greater than normal values for ordering.
///
/// ```rust
/// use seqmarked::Marked;
///
/// let data = Marked::Normal("hello");
/// let tombstone = Marked::<&str>::TombStone;
/// assert!(tombstone > data);
/// ```
#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq, Eq)]
#[derive(PartialOrd, Ord)]
#[cfg_attr(
    feature = "seqmarked-serde",
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(
    feature = "seqmarked-bincode",
    derive(bincode::Encode, bincode::Decode)
)]
pub enum Marked<D> {
    /// Normal data.
    // Keep `Normal` as the first variant so that `TombStone` is greater than `Normal`.
    Normal(D),

    /// Deletion marker.
    TombStone,
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_marked_ord() {
        let a = Marked::Normal(1u64);
        let b = Marked::TombStone;

        assert!(a < b);
        assert!(b > a);
        assert_eq!(a, a);
        assert_eq!(b, b);
    }
}

#[cfg(test)]
#[cfg(feature = "seqmarked-bincode")]
mod tests_bincode {

    use super::*;
    use crate::testing::bincode_config;
    use crate::testing::test_bincode_decode;

    #[test]
    fn test_marked_bincode() {
        let a = Marked::Normal(1u64);
        let encoded = bincode::encode_to_vec(&a, bincode_config()).unwrap();
        let (decoded, n): (Marked<u64>, usize) =
            bincode::decode_from_slice(&encoded, bincode_config()).unwrap();
        assert_eq!(n, 2);
        assert_eq!(a, decoded);
    }

    #[test]
    fn test_marked_bincode_decode_v010() -> anyhow::Result<()> {
        let value = Marked::Normal(1u64);
        let encoded = vec![0, 1];

        test_bincode_decode(&encoded, &value)?;

        let value = Marked::TombStone::<()>;
        let encoded = vec![1];

        test_bincode_decode(&encoded, &value)?;
        Ok(())
    }
}

#[cfg(test)]
#[cfg(feature = "seqmarked-serde")]
mod tests_serde {
    use super::*;
    use crate::testing::test_serde_decode;

    #[test]
    fn test_marked_serde() {
        let a = Marked::Normal(1u64);
        let encoded = serde_json::to_string(&a).unwrap();
        let decoded: Marked<u64> = serde_json::from_str(&encoded).unwrap();
        assert_eq!(a, decoded);
    }

    #[test]
    fn test_marked_serde_decode_v010() -> anyhow::Result<()> {
        let value = Marked::Normal(1u64);
        let encoded = r#"{"Normal":1}"#;

        test_serde_decode(encoded, &value)?;

        let value = Marked::TombStone::<()>;
        let encoded = r#""TombStone""#;

        test_serde_decode(encoded, &value)?;
        Ok(())
    }
}
