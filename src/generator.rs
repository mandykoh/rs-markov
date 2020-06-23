/// A Generator for generating probable outcomes using a
/// [Model](struct.Model.html).
///
/// Generators do not modify the underlying model.
pub struct Generator<'a, TSymbol>
where
    TSymbol: std::marker::Copy + std::hash::Hash + std::cmp::Eq,
{
    model: &'a crate::Model<TSymbol>,
    current_sequence: crate::Sequence<TSymbol>,
    next_rand: Box<dyn FnMut() -> f64>,
}

impl<'a, TSymbol> Generator<'a, TSymbol>
where
    TSymbol: std::marker::Copy + std::hash::Hash + std::cmp::Eq,
{
    /// Creates a Generator which uses the specified model.
    ///
    /// # Arguments
    ///
    /// `model` - The Markov model to base generated data on.
    ///
    /// `rand_source` - A function for returning values in the [0.0, 1.0) range,
    /// used to generate the output.
    ///
    /// # Example
    ///
    /// ```
    /// use rand::Rng;
    ///
    /// let model = markov::Model::<i32>::empty(1);
    ///
    /// let mut rng = rand::thread_rng();
    /// let mut gen = markov::Generator::new(&model, Box::new(move || rng.gen::<f64>()));
    /// ```
    pub fn new(
        model: &'a crate::Model<TSymbol>,
        rand_source: Box<dyn FnMut() -> f64>,
    ) -> Generator<'a, TSymbol> {
        Generator {
            model,
            current_sequence: crate::Sequence::empty(),
            next_rand: rand_source,
        }
    }

    /// Resets this Generator so that the next symbol generated will be the
    /// beginning of a sequence.
    pub fn end(&mut self) {
        self.current_sequence = crate::Sequence::empty();
    }

    /// Generates and returns the next symbol based on the previously generated
    /// symbols.
    ///
    /// `None` is returned when the end of a sequence is reached.
    ///
    /// # Example
    ///
    /// ```
    /// use rand::Rng;
    ///
    /// let model = markov::Model::<i32>::empty(1);
    ///
    /// let mut rng = rand::thread_rng();
    /// let mut gen = markov::Generator::new(&model, Box::new(move || rng.gen::<f64>()));
    ///
    /// while let Some(symbol) = gen.next() {
    ///      print!(" {}", symbol);
    /// }
    /// println!();
    /// ```
    pub fn next(&mut self) -> Option<&TSymbol> {
        match self
            .model
            .sample(&self.current_sequence, (self.next_rand)())
        {
            Some(s) => {
                self.current_sequence = self.model.advance_sequence(&self.current_sequence, *s);
                Some(s)
            }
            None => None,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::generator::Generator;
    use crate::model::Model;
    use crate::sequence::Sequence;

    #[test]
    fn it_generates_sequences() {
        let mut model = Model::empty(1);

        let seq = Sequence::empty();
        model.add(&seq, Some("the"));
        let seq = model.advance_sequence(&seq, "the");
        model.add(&seq, Some("quick"));
        let seq = model.advance_sequence(&seq, "quick");
        model.add(&seq, Some("brown"));
        let seq = model.advance_sequence(&seq, "brown");
        model.add(&seq, Some("fox"));

        let seq = Sequence::empty();
        model.add(&seq, Some("the"));
        let seq = model.advance_sequence(&seq, "the");
        model.add(&seq, Some("lazy"));
        let seq = model.advance_sequence(&seq, "lazy");
        model.add(&seq, Some("dog"));

        let mut gen = Generator::new(&model, Box::new(|| 0.0));

        assert_eq!(gen.next(), Some(&"the"));
        assert_eq!(gen.next(), Some(&"quick"));
        assert_eq!(gen.next(), Some(&"brown"));
        assert_eq!(gen.next(), Some(&"fox"));
        assert_eq!(gen.next(), None);

        gen.end();
        gen.next_rand = Box::new(|| 0.5);

        assert_eq!(gen.next(), Some(&"the"));
        assert_eq!(gen.next(), Some(&"lazy"));
        assert_eq!(gen.next(), Some(&"dog"));
        assert_eq!(gen.next(), None);
    }
}
