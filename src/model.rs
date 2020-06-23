use std::collections::HashMap;

/// A model based on Markov chains.
pub struct Model<TSymbol>
where
    TSymbol: std::marker::Copy + std::hash::Hash + std::cmp::Eq,
{
    order: usize,
    tables_by_seq: HashMap<crate::Sequence<TSymbol>, crate::Table<TSymbol>>,
}

impl<TSymbol> Model<TSymbol>
where
    TSymbol: std::marker::Copy + std::hash::Hash + std::cmp::Eq,
{
    /// Creates an empty Markov model.
    ///
    /// An empty model contains no information about symbols or their
    /// frequencies.
    ///
    /// # Arguments
    ///
    /// * `order` - The order of the model. A first order Markov model
    /// (order: 1) tracks probabilities of future symbols based on one prior
    /// symbol). A second order model (order: 2) tracks probabilities of
    /// future symbols based on two prior symbols, and so on.
    pub fn empty(order: usize) -> Model<TSymbol> {
        Model {
            order,
            tables_by_seq: Default::default(),
        }
    }

    pub(crate) fn add(&mut self, seq: &crate::Sequence<TSymbol>, next_symbol: Option<TSymbol>) {
        match self.tables_by_seq.get_mut(seq) {
            Some(t) => {
                t.add(next_symbol);
            }
            None => {
                let mut t = crate::Table::empty();
                t.add(next_symbol);
                self.tables_by_seq.insert(seq.clone(), t);
            }
        };
    }

    pub(crate) fn advance_sequence(
        &self,
        seq: &crate::Sequence<TSymbol>,
        next_symbol: TSymbol,
    ) -> crate::Sequence<TSymbol> {
        seq.with_next(next_symbol, self.order)
    }

    pub(crate) fn predict(&self, seq: &crate::Sequence<TSymbol>) -> Option<&TSymbol> {
        match self.tables_by_seq.get(seq) {
            Some(t) => t.most_frequent(),
            None => None,
        }
    }

    pub(crate) fn sample(
        &self,
        seq: &crate::Sequence<TSymbol>,
        sample_value: f64,
    ) -> Option<&TSymbol> {
        match self.tables_by_seq.get(seq) {
            Some(t) => t.sample(sample_value),
            None => None,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::model::Model;
    use crate::sequence::Sequence;

    #[test]
    fn it_adds_tables_for_each_new_added_sequence() {
        let mut m = Model::empty(1);

        assert!(m.tables_by_seq.is_empty());

        let seq = Sequence::empty();
        m.add(&seq, Some('a'));

        assert_eq!(m.tables_by_seq.len(), 1);

        let t = m.tables_by_seq.get(&seq);
        assert!(t.is_some());
        assert_eq!(t.unwrap().most_frequent(), Some(&'a'));

        let seq = m.advance_sequence(&seq, 'a');
        m.add(&seq, Some('b'));

        assert_eq!(m.tables_by_seq.len(), 2);

        let t = m.tables_by_seq.get(&seq);
        assert!(t.is_some());
        assert_eq!(t.unwrap().most_frequent(), Some(&'b'));
    }

    #[test]
    fn it_adds_symbols_to_existing_tables_for_known_sequences() {
        let mut m = Model::empty(1);

        let seq = Sequence::empty();
        m.add(&seq, Some('a'));
        m.add(&seq, Some('b'));
        m.add(&seq, Some('b'));

        assert_eq!(m.tables_by_seq.len(), 1);

        let t = m.tables_by_seq.get(&seq);
        assert!(t.is_some());
        assert_eq!(t.unwrap().most_frequent(), Some(&'b'));
    }
}
