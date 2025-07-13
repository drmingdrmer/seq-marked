use std::fmt;

use super::SeqMarked;

impl<D> fmt::Display for SeqMarked<D>
where D: fmt::Display
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{seq={}, {}}}", self.seq, self.marked)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display_normal_string() {
        let seq_marked = SeqMarked::new_normal(42, "hello world");
        assert_eq!(format!("{}", seq_marked), "{seq=42, (hello world)}");
    }

    #[test]
    fn test_display_normal_number() {
        let seq_marked = SeqMarked::new_normal(10, 12345);
        assert_eq!(format!("{}", seq_marked), "{seq=10, (12345)}");
    }

    #[test]
    fn test_display_tombstone() {
        let seq_marked = SeqMarked::<&str>::new_tombstone(25);
        assert_eq!(format!("{}", seq_marked), "{seq=25, TOMBSTONE}");
    }
}
