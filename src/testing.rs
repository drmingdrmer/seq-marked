//! Testing utilities for seqmarked.

#![allow(dead_code)]

use crate::Expirable;
use crate::SeqMarked;
use crate::SeqValue;

/// Expirable implementation for testing purposes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct ExpirableImpl {
    pub(crate) expires_at_ms: Option<u64>,
}

impl Expirable for ExpirableImpl {
    fn expires_at_ms_opt(&self) -> Option<u64> {
        self.expires_at_ms
    }
}

/// A sequence value implementation for testing purposes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct SeqValueImpl {
    pub(crate) seq: u64,
    pub(crate) value: Option<u64>,
    pub(crate) meta: Option<ExpirableImpl>,
}

impl SeqValue<ExpirableImpl, u64> for SeqValueImpl {
    fn seq(&self) -> u64 {
        self.seq
    }

    fn value(&self) -> Option<&u64> {
        self.value.as_ref()
    }

    fn into_value(self) -> Option<u64> {
        self.value
    }

    fn meta(&self) -> Option<&ExpirableImpl> {
        self.meta.as_ref()
    }
}

/// Create a string.
pub(crate) fn ss(x: impl ToString) -> String {
    x.to_string()
}

/// Create a String vector.
pub(crate) fn ss_vec(x: impl IntoIterator<Item = impl ToString>) -> Vec<String> {
    let r = x.into_iter().map(|x| x.to_string());
    r.collect()
}

/// Create a byte vector.
pub(crate) fn bb(x: impl ToString) -> Vec<u8> {
    x.to_string().into_bytes()
}

/// Create concatenated byte vector.
pub(crate) fn bbs(x: impl IntoIterator<Item = impl ToString>) -> Vec<u8> {
    let r = x.into_iter().map(|x| x.to_string().into_bytes());
    vec_chain(r)
}

/// Concat multiple Vec into one.
pub(crate) fn vec_chain<T>(vectors: impl IntoIterator<Item = Vec<T>>) -> Vec<T> {
    let mut r = vec![];
    for v in vectors {
        r.extend(v);
    }
    r
}

/// Create a `SeqMarked::Normal`.
pub(crate) fn norm<D>(seq: u64, d: D) -> SeqMarked<D> {
    SeqMarked::new_normal(seq, d)
}

/// Create a `SeqMarked::TombStone`.
pub(crate) fn ts<D>(seq: u64) -> SeqMarked<D> {
    SeqMarked::new_tombstone(seq)
}

#[cfg(feature = "seqmarked-bincode")]
pub fn bincode_config() -> impl bincode::config::Config {
    bincode::config::standard().with_big_endian().with_variable_int_encoding()
}

#[cfg(feature = "seqmarked-bincode")]
pub fn test_bincode_decode<T>(encoded: &[u8], value: &T) -> anyhow::Result<()>
where T: bincode::Encode + bincode::Decode<()> + PartialEq + std::fmt::Debug {
    let got_encoded = bincode::encode_to_vec(value, bincode_config())?;
    println!("let encoded = vec!{:?};", got_encoded);

    let (decoded, n): (T, usize) = bincode::decode_from_slice(encoded, bincode_config())?;

    assert_eq!(n, encoded.len());
    assert_eq!(&decoded, value);
    Ok(())
}

#[cfg(feature = "seqmarked-serde")]
pub fn test_serde_decode<T>(encoded: &str, value: &T) -> anyhow::Result<()>
where T: serde::Serialize + serde::de::DeserializeOwned + PartialEq + std::fmt::Debug {
    let got_encoded = serde_json::to_string(value)?;
    println!("let encoded = r#\"{}\"#;", got_encoded);

    let decoded: T = serde_json::from_str(&encoded)?;
    assert_eq!(decoded, *value);
    Ok(())
}
