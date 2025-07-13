use std::io;

use crate::Marked;
use crate::SeqMarked;

impl<M> TryFrom<SeqMarked<(Option<M>, Vec<u8>)>> for SeqMarked<(Option<M>, String)> {
    type Error = io::Error;

    fn try_from(value: SeqMarked<(Option<M>, Vec<u8>)>) -> Result<Self, Self::Error> {
        let (seq, marked) = value.into_parts();

        let marked = Marked::<(Option<M>, String)>::try_from(marked)?;

        Ok(SeqMarked::new(seq, marked))
    }
}

impl<M> From<SeqMarked<(Option<M>, String)>> for SeqMarked<(Option<M>, Vec<u8>)> {
    fn from(value: SeqMarked<(Option<M>, String)>) -> Self {
        let (seq, marked) = value.into_parts();

        let marked = Marked::<(Option<M>, Vec<u8>)>::from(marked);

        SeqMarked::new(seq, marked)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_try_from_bytes_to_string_success() {
        let seq_marked = SeqMarked::new_normal(42, (Some("metadata"), "hello".as_bytes().to_vec()));
        let result: Result<SeqMarked<(Option<&str>, String)>, _> = seq_marked.try_into();

        assert!(result.is_ok());
        let converted = result.unwrap();
        assert_eq!(converted.seq(), 42);
        assert_eq!(
            converted.data_ref(),
            Some(&(Some("metadata"), "hello".to_string()))
        );
    }

    #[test]
    fn test_try_from_bytes_to_string_without_meta() {
        let seq_marked = SeqMarked::new_normal(10, (None::<String>, "world".as_bytes().to_vec()));
        let result: Result<SeqMarked<(Option<String>, String)>, _> = seq_marked.try_into();

        assert!(result.is_ok());
        let converted = result.unwrap();
        assert_eq!(converted.seq(), 10);
        assert_eq!(converted.data_ref(), Some(&(None, "world".to_string())));
    }

    #[test]
    fn test_try_from_bytes_to_string_tombstone() {
        let seq_marked = SeqMarked::<(Option<String>, Vec<u8>)>::new_tombstone(5);
        let result: Result<SeqMarked<(Option<String>, String)>, _> = seq_marked.try_into();

        assert!(result.is_ok());
        let converted = result.unwrap();
        assert_eq!(converted.seq(), 5);
        assert!(converted.is_tombstone());
    }

    #[test]
    fn test_try_from_bytes_to_string_invalid_utf8() {
        let invalid_bytes = vec![0xFF, 0xFE, 0xFD];
        let seq_marked = SeqMarked::new_normal(20, (Some("test"), invalid_bytes));
        let result: Result<SeqMarked<(Option<&str>, String)>, _> = seq_marked.try_into();

        assert!(result.is_err());
        let error = result.unwrap_err();
        assert_eq!(error.kind(), io::ErrorKind::InvalidData);
    }

    #[test]
    fn test_from_string_to_bytes() {
        let seq_marked = SeqMarked::new_normal(30, (Some("metadata"), "hello".to_string()));
        let converted: SeqMarked<(Option<&str>, Vec<u8>)> = seq_marked.into();

        assert_eq!(converted.seq(), 30);
        assert_eq!(
            converted.data_ref(),
            Some(&(Some("metadata"), "hello".as_bytes().to_vec()))
        );
    }

    #[test]
    fn test_from_string_to_bytes_without_meta() {
        let seq_marked = SeqMarked::new_normal(15, (None::<String>, "world".to_string()));
        let converted: SeqMarked<(Option<String>, Vec<u8>)> = seq_marked.into();

        assert_eq!(converted.seq(), 15);
        assert_eq!(
            converted.data_ref(),
            Some(&(None, "world".as_bytes().to_vec()))
        );
    }

    #[test]
    fn test_from_string_to_bytes_tombstone() {
        let seq_marked = SeqMarked::<(Option<String>, String)>::new_tombstone(25);
        let converted: SeqMarked<(Option<String>, Vec<u8>)> = seq_marked.into();

        assert_eq!(converted.seq(), 25);
        assert!(converted.is_tombstone());
    }
}
