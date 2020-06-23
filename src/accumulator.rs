/// An Accumulator for updating a [Model](struct.Model.html) with training data.
pub struct Accumulator<'a, TSymbol>
where
    TSymbol: std::marker::Copy + std::hash::Hash + std::cmp::Eq,
{
    model: &'a mut crate::Model<TSymbol>,
    current_sequence: crate::Sequence<TSymbol>,
}

impl<'a, TSymbol> Accumulator<'a, TSymbol>
where
    TSymbol: std::marker::Copy + std::hash::Hash + std::cmp::Eq,
{
    /// Creates an Accumulator to update the specified model.
    ///
    /// # Arguments
    ///
    /// `model` - The Markov model to update.
    pub fn new(model: &mut crate::Model<TSymbol>) -> Accumulator<TSymbol> {
        Accumulator {
            model,
            current_sequence: crate::Sequence::empty(),
        }
    }

    /// Adds a symbol to the current sequence.
    ///
    /// # Arguments
    ///
    /// `symbol` - The next symbol in the current sequence.
    ///
    /// # Example
    ///
    /// ```
    /// let mut model = markov::Model::empty(1);
    ///
    /// let mut acc = markov::Accumulator::new(&mut model);
    /// acc.add("the");
    /// acc.add("quick");
    /// acc.add("brown");
    /// acc.add("fox");
    /// acc.end();
    /// ```
    pub fn add(&mut self, symbol: TSymbol) {
        self.model.add(&self.current_sequence, Some(symbol));
        self.current_sequence = self.model.advance_sequence(&self.current_sequence, symbol);
    }

    /// Indicates the end of the current sequence and resets this Accumulator
    /// for a new sequence.
    pub fn end(&mut self) {
        self.model.add(&self.current_sequence, None);
        self.current_sequence = crate::Sequence::empty();
    }

    /// Predicts and returns the most probable next symbol based on previous
    /// symbols added via [`add`](#method.add).
    ///
    /// `None` is returned when the end of a sequence is reached.
    pub fn predict(&self) -> Option<&TSymbol> {
        self.model.predict(&self.current_sequence)
    }
}

#[cfg(test)]
mod test {
    use crate::accumulator::Accumulator;
    use crate::model::Model;
    use crate::sequence::Sequence;

    #[test]
    fn it_accumulates_symbols_into_model() {
        let mut model = Model::empty(1);

        let mut acc = Accumulator::new(&mut model);
        acc.add('a');
        acc.add('b');
        acc.add('c');
        acc.end();
        acc.add('a');
        acc.add('d');
        acc.add('e');

        let seq = Sequence::empty();
        assert_eq!(model.sample(&seq, 0.0), Some(&'a'));
        let seq = model.advance_sequence(&seq, 'a');
        assert_eq!(model.sample(&seq, 0.0), Some(&'b'));
        let seq = model.advance_sequence(&seq, 'b');
        assert_eq!(model.sample(&seq, 0.0), Some(&'c'));

        let seq = Sequence::empty();
        assert_eq!(model.sample(&seq, 0.0), Some(&'a'));
        let seq = model.advance_sequence(&seq, 'a');
        assert_eq!(model.sample(&seq, 0.5), Some(&'d'));
        let seq = model.advance_sequence(&seq, 'd');
        assert_eq!(model.sample(&seq, 0.0), Some(&'e'));
    }
}
