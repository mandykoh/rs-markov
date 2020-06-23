/// A Predictor for finding the most probable future outcomes given past history
/// based on a [Model](struct.Model.html).
///
/// Predictors do not modify the underlying model.
pub struct Predictor<'a, TSymbol>
where
    TSymbol: std::marker::Copy + std::hash::Hash + std::cmp::Eq,
{
    model: &'a crate::Model<TSymbol>,
    current_sequence: crate::Sequence<TSymbol>,
}

impl<'a, TSymbol> Predictor<'a, TSymbol>
where
    TSymbol: std::marker::Copy + std::hash::Hash + std::cmp::Eq,
{
    /// Creates a Predictor which uses the specified model.
    ///
    /// # Arguments
    ///
    /// `model` - The Markov model to base predictions on.
    pub fn new(model: &crate::Model<TSymbol>) -> Predictor<TSymbol> {
        Predictor {
            model,
            current_sequence: crate::Sequence::empty(),
        }
    }

    /// Resets this Predictor so that the next symbol predicted will be the
    /// beginning of a sequence.
    pub fn end(&mut self) {
        self.current_sequence = crate::Sequence::empty();
    }

    /// Specifies a prior symbol upon which future predictions will be based.
    ///
    /// # Arguments
    ///
    /// `symbol` - The most recent past symbol on which to base predictions.
    ///
    /// # Example
    ///
    /// ```
    /// let model = markov::Model::empty(1);
    ///
    /// let mut pre = markov::Predictor::new(&model);
    /// pre.given("the");
    /// pre.given("quick");
    /// pre.given("brown");
    ///
    /// let prediction = pre.predict(); // returns Some("fox")
    /// ```
    pub fn given(&mut self, symbol: TSymbol) {
        self.current_sequence = self.model.advance_sequence(&self.current_sequence, symbol);
    }

    /// Predicts and returns the most probable next symbol based on previous
    /// symbols either predicted or specified via [`given`](#method.given).
    ///
    /// `None` is returned when the end of a sequence is reached.
    ///
    /// # Example
    ///
    /// ```
    /// let model = markov::Model::empty(1);
    ///
    /// let mut pre = markov::Predictor::new(&model);
    /// pre.given("the");
    ///
    /// while let Some(symbol) = pre.next() {
    ///      print!(" {}", symbol);
    /// }
    /// println!();
    /// ```
    pub fn next(&mut self) -> Option<&TSymbol> {
        match self.model.predict(&self.current_sequence) {
            Some(s) => {
                self.current_sequence = self.model.advance_sequence(&self.current_sequence, *s);
                Some(s)
            }
            None => None,
        }
    }

    /// Predicts and returns the most probable next symbol based on previous
    /// symbols either predicted via [`next`](#method.next) or specified via
    /// [`given`](#method.given), without advancing to the next symbol in the
    /// sequence.
    ///
    /// `None` is returned when the end of a sequence is reached.
    pub fn predict(&self) -> Option<&TSymbol> {
        self.model.predict(&self.current_sequence)
    }
}

#[cfg(test)]
mod test {
    use crate::model::Model;
    use crate::predictor::Predictor;
    use crate::sequence::Sequence;

    #[test]
    fn it_predicts_most_probable_sequences() {
        let mut model = Model::empty(1);

        let seq = Sequence::empty();
        model.add(&seq, Some("the"));
        let seq = model.advance_sequence(&seq, "the");
        model.add(&seq, Some("quick"));
        let seq = model.advance_sequence(&seq, "quick");
        model.add(&seq, Some("brown"));
        let seq = model.advance_sequence(&seq, "brown");
        model.add(&seq, Some("fox"));

        let mut pre = Predictor::new(&model);
        assert_eq!(pre.next(), Some(&"the"));
        assert_eq!(pre.next(), Some(&"quick"));
        assert_eq!(pre.next(), Some(&"brown"));
        assert_eq!(pre.next(), Some(&"fox"));
        assert_eq!(pre.next(), None);

        let seq = Sequence::empty();
        model.add(&seq, Some("the"));
        let seq = model.advance_sequence(&seq, "the");
        model.add(&seq, Some("lazy"));
        let seq = model.advance_sequence(&seq, "lazy");
        model.add(&seq, Some("dog"));

        let seq = Sequence::empty();
        model.add(&seq, Some("the"));
        let seq = model.advance_sequence(&seq, "the");
        model.add(&seq, Some("lazy"));
        let seq = model.advance_sequence(&seq, "lazy");
        model.add(&seq, Some("penguin"));

        let mut pre = Predictor::new(&model);
        assert_eq!(pre.next(), Some(&"the"));
        assert_eq!(pre.next(), Some(&"lazy"));
        assert_eq!(pre.next(), Some(&"dog"));
        assert_eq!(pre.next(), None);

        let seq = Sequence::empty();
        model.add(&seq, Some("the"));
        let seq = model.advance_sequence(&seq, "the");
        model.add(&seq, Some("lazy"));
        let seq = model.advance_sequence(&seq, "lazy");
        model.add(&seq, Some("penguin"));

        let mut pre = Predictor::new(&model);
        assert_eq!(pre.next(), Some(&"the"));
        assert_eq!(pre.next(), Some(&"lazy"));
        assert_eq!(pre.next(), Some(&"penguin"));
        assert_eq!(pre.next(), None);
    }
}
