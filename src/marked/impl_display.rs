use std::fmt;

use super::Marked;

impl<D> fmt::Display for Marked<D>
where D: fmt::Display
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Marked::Normal(data) => write!(f, "({})", data)?,
            Marked::TombStone => write!(f, "TOMBSTONE")?,
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display_normal_string() {
        let marked = Marked::Normal("hello world");
        assert_eq!(format!("{}", marked), "(hello world)");
    }

    #[test]
    fn test_display_tombstone() {
        let marked = Marked::<u64>::TombStone;
        assert_eq!(format!("{}", marked), "TOMBSTONE");
    }
}
