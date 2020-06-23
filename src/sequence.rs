#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct Sequence<TSymbol>
where
    TSymbol: std::marker::Copy + std::hash::Hash + std::cmp::Eq,
{
    symbols: Vec<TSymbol>,
}

impl<TSymbol> Sequence<TSymbol>
where
    TSymbol: std::marker::Copy + std::hash::Hash + std::cmp::Eq,
{
    pub(crate) fn empty() -> Sequence<TSymbol> {
        Sequence { symbols: vec![] }
    }

    pub(crate) fn with_next(&self, next_symbol: TSymbol, order: usize) -> Sequence<TSymbol> {
        let last_symbols = if self.symbols.len() < order {
            &self.symbols[..]
        } else {
            &self.symbols[(self.symbols.len() + 1 - order)..]
        };

        let mut next_symbols: Vec<TSymbol> = Vec::with_capacity(order);
        next_symbols.extend_from_slice(last_symbols);
        next_symbols.push(next_symbol);

        Sequence {
            symbols: next_symbols,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::sequence::Sequence;

    #[test]
    fn it_allows_next_sequence_to_be_derived() {
        let mut seq: Sequence<char> = Sequence::empty();

        assert_eq!(seq.symbols, vec![]);

        seq = seq.with_next('a', 3);
        assert_eq!(seq.symbols, vec!['a']);

        seq = seq.with_next('b', 3);
        assert_eq!(seq.symbols, vec!['a', 'b']);

        seq = seq.with_next('c', 3);
        assert_eq!(seq.symbols, vec!['a', 'b', 'c']);
    }

    #[test]
    fn it_limits_length_to_specified_order() {
        let mut seq: Sequence<char> = Sequence {
            symbols: vec!['a', 'b', 'c'],
        };

        seq = seq.with_next('d', 3);
        assert_eq!(seq.symbols, vec!['b', 'c', 'd']);

        seq = seq.with_next('e', 2);
        assert_eq!(seq.symbols, vec!['d', 'e']);
    }
}
