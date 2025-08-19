use crate::SeqMarked;

impl<D> SeqMarked<&D> {
    pub fn cloned(self) -> SeqMarked<D>
    where D: Clone {
        self.map(|d| d.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Marked;

    #[test]
    fn test_cloned() {
        let a = &1u64;
        let seq_marked = SeqMarked::<&u64>::new_normal(1, a);
        let cloned = seq_marked.cloned();
        assert_eq!(cloned.seq, 1);
        assert_eq!(cloned.marked, Marked::Normal(1));
    }
}
