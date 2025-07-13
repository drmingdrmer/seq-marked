use std::io;

use crate::Marked;

impl<M> TryFrom<Marked<(Option<M>, Vec<u8>)>> for Marked<(Option<M>, String)> {
    type Error = io::Error;

    fn try_from(marked: Marked<(Option<M>, Vec<u8>)>) -> Result<Self, Self::Error> {
        match marked {
            Marked::TombStone => Ok(Marked::TombStone),
            Marked::Normal((meta, value)) => {
                let s = String::from_utf8(value).map_err(|e| {
                    io::Error::new(
                        io::ErrorKind::InvalidData,
                        format!("fail to convert Vec<u8> to String: {}", e),
                    )
                })?;

                Ok(Marked::Normal((meta, s)))
            }
        }
    }
}

impl<M> From<Marked<(Option<M>, String)>> for Marked<(Option<M>, Vec<u8>)> {
    /// Convert `Marked<String>` to `Marked<Vec<u8>>`
    fn from(value: Marked<(Option<M>, String)>) -> Self {
        match value {
            Marked::TombStone => Marked::TombStone,
            Marked::Normal((meta, value)) => {
                let v = value.into_bytes();
                Marked::Normal((meta, v))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_try_from_bytes_to_string_success() {
        let marked = Marked::Normal((Some("metadata"), "hello".as_bytes().to_vec()));
        let result: Result<Marked<(Option<&str>, String)>, _> = marked.try_into();

        assert!(result.is_ok());
        let converted = result.unwrap();
        assert_eq!(
            converted,
            Marked::Normal((Some("metadata"), "hello".to_string()))
        );
    }

    #[test]
    fn test_try_from_bytes_to_string_without_meta() {
        let marked = Marked::Normal((None::<String>, "world".as_bytes().to_vec()));
        let result: Result<Marked<(Option<String>, String)>, _> = marked.try_into();

        assert!(result.is_ok());
        let converted = result.unwrap();
        assert_eq!(converted, Marked::Normal((None, "world".to_string())));
    }

    #[test]
    fn test_try_from_bytes_to_string_tombstone() {
        let marked = Marked::<(Option<String>, Vec<u8>)>::TombStone;
        let result: Result<Marked<(Option<String>, String)>, _> = marked.try_into();

        assert!(result.is_ok());
        let converted = result.unwrap();
        assert_eq!(converted, Marked::TombStone);
    }

    #[test]
    fn test_try_from_bytes_to_string_invalid_utf8() {
        let invalid_bytes = vec![0xFF, 0xFE, 0xFD];
        let marked = Marked::Normal((Some("test"), invalid_bytes));
        let result: Result<Marked<(Option<&str>, String)>, _> = marked.try_into();

        assert!(result.is_err());
        let error = result.unwrap_err();
        assert_eq!(error.kind(), io::ErrorKind::InvalidData);
        assert!(error.to_string().contains("fail to convert Vec<u8> to String"));
    }

    #[test]
    fn test_from_string_to_bytes() {
        let marked = Marked::Normal((Some("metadata"), "hello".to_string()));
        let converted: Marked<(Option<&str>, Vec<u8>)> = marked.into();

        assert_eq!(
            converted,
            Marked::Normal((Some("metadata"), "hello".as_bytes().to_vec()))
        );
    }

    #[test]
    fn test_from_string_to_bytes_without_meta() {
        let marked = Marked::Normal((None::<String>, "world".to_string()));
        let converted: Marked<(Option<String>, Vec<u8>)> = marked.into();

        assert_eq!(
            converted,
            Marked::Normal((None, "world".as_bytes().to_vec()))
        );
    }

    #[test]
    fn test_from_string_to_bytes_tombstone() {
        let marked = Marked::<(Option<String>, String)>::TombStone;
        let converted: Marked<(Option<String>, Vec<u8>)> = marked.into();

        assert_eq!(converted, Marked::TombStone);
    }
}
