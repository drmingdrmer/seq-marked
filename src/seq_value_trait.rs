use crate::Expirable;

/// Trait for a value with a sequence number and metadata.
///
/// [`SeqValue`] is intended for application use and does not include a tombstone concept,
/// unlike [`SeqMarked`] which is for LSM internals.
pub trait SeqValue<M, V = Vec<u8>> {
    /// Return the sequence number of the value.
    fn seq(&self) -> u64;

    /// Return the reference of the value.
    fn value(&self) -> Option<&V>;

    /// Consume the value and return the value.
    fn into_value(self) -> Option<V>;

    /// Return the reference of metadata of the value.
    fn meta(&self) -> Option<&M>;

    /// Consume self and return the sequence number and the value.
    fn unpack(self) -> (u64, Option<V>)
    where Self: Sized {
        (self.seq(), self.into_value())
    }

    /// Return the absolute expire time in millisecond since 1970-01-01 00:00:00.
    fn expires_at_ms_opt(&self) -> Option<u64>
    where M: Expirable {
        let meta = self.meta()?;
        meta.expires_at_ms_opt()
    }

    /// Returns the absolute expiration time in milliseconds since the Unix epoch (1970-01-01
    /// 00:00:00 UTC).
    ///
    /// If no expiration time is set, returns `u64::MAX`, effectively meaning the value never
    /// expires. This method provides a consistent way to handle both expiring and non-expiring
    /// values.
    fn expires_at_ms(&self) -> u64
    where M: Expirable {
        self.meta().expires_at_ms()
    }

    /// Return true if the record is expired at the given time in milliseconds since the Unix epoch
    /// (1970-01-01 00:00:00 UTC).
    fn is_expired(&self, now_ms: u64) -> bool
    where M: Expirable {
        self.expires_at_ms() < now_ms
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    struct ExpirableImpl {
        expires_at_ms: Option<u64>,
    }

    impl Expirable for ExpirableImpl {
        fn expires_at_ms_opt(&self) -> Option<u64> {
            self.expires_at_ms
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    struct SeqValueImpl {
        seq: u64,
        value: Option<u64>,
        meta: Option<ExpirableImpl>,
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

    #[test]
    fn test_seq_value_basic() {
        let sv = SeqValueImpl {
            seq: 42,
            value: Some(100),
            meta: None,
        };

        assert_eq!(sv.seq(), 42);
        assert_eq!(sv.value(), Some(&100));
        assert_eq!(sv.meta(), None);

        let (seq, value) = sv.unpack();
        assert_eq!(seq, 42);
        assert_eq!(value, Some(100));
    }

    #[test]
    fn test_seq_value_with_expiration() {
        let sv = SeqValueImpl {
            seq: 1,
            value: Some(200),
            meta: Some(ExpirableImpl {
                expires_at_ms: Some(1000),
            }),
        };

        assert_eq!(sv.expires_at_ms_opt(), Some(1000));
        assert_eq!(sv.expires_at_ms(), 1000);
        assert!(sv.is_expired(1001));
        assert!(!sv.is_expired(999));
    }

    #[test]
    fn test_seq_value_no_expiration() {
        let sv = SeqValueImpl {
            seq: 2,
            value: None,
            meta: Some(ExpirableImpl {
                expires_at_ms: None,
            }),
        };

        assert_eq!(sv.expires_at_ms_opt(), None);
        assert_eq!(sv.expires_at_ms(), u64::MAX);
        assert!(!sv.is_expired(u64::MAX - 1));
    }
}
