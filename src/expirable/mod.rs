//! Expirable trait for types that can have an expiration time.
//!
//! This module provides the `Expirable` trait which allows types to define
//! and query their expiration time in milliseconds since the Unix epoch.
//! It's used to implement time-to-live (TTL) functionality for stored values.

mod expirable_impl;

/// A trait for evaluating and returning the absolute expiration time.
pub trait Expirable {
    /// Returns the optional expiration time in milliseconds since the Unix epoch (January 1, 1970).
    fn expires_at_ms_opt(&self) -> Option<u64>;

    /// Evaluates and returns the absolute expiration time in milliseconds since the Unix epoch
    /// (January 1, 1970).
    ///
    /// If there is no expiration time, it returns `u64::MAX`.
    fn expires_at_ms(&self) -> u64 {
        self.expires_at_ms_opt().unwrap_or(u64::MAX)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, Copy)]
    struct ExpirableImpl {
        expires_at_ms: Option<u64>,
    }

    impl Expirable for ExpirableImpl {
        fn expires_at_ms_opt(&self) -> Option<u64> {
            self.expires_at_ms
        }
    }

    #[test]
    fn test_expirable() {
        let e1 = ExpirableImpl {
            expires_at_ms: Some(1),
        };
        assert_eq!(e1.expires_at_ms_opt(), Some(1));
        assert_eq!(e1.expires_at_ms(), 1);

        let e2 = ExpirableImpl {
            expires_at_ms: None,
        };
        assert_eq!(e2.expires_at_ms_opt(), None);
        assert_eq!(e2.expires_at_ms(), u64::MAX);

        // Test with a reference

        {
            let e1_ref = &e1;
            assert_eq!(e1_ref.expires_at_ms_opt(), Some(1));
            assert_eq!(e1_ref.expires_at_ms(), 1);
        }

        {
            let e2_ref = &e2;
            assert_eq!(e2_ref.expires_at_ms_opt(), None);
            assert_eq!(e2_ref.expires_at_ms(), u64::MAX);
        }

        // Test with Option

        {
            let e1_opt = Some(e1);
            assert_eq!(e1_opt.expires_at_ms_opt(), Some(1));
            assert_eq!(e1_opt.expires_at_ms(), 1);
        }

        {
            let e1_opt = None::<ExpirableImpl>;
            assert_eq!(e1_opt.expires_at_ms_opt(), None);
            assert_eq!(e1_opt.expires_at_ms(), u64::MAX);
        }

        {
            let e2_opt = Some(e1);
            assert_eq!(e2_opt.expires_at_ms_opt(), Some(1));
            assert_eq!(e2_opt.expires_at_ms(), 1);
        }
    }
}
