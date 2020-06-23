use std::collections::HashMap;

pub(crate) struct Table<TSymbol> {
    total_symbols: usize,
    entries: Vec<TableEntry<TSymbol>>,
    entry_indices: HashMap<Option<TSymbol>, usize>,
}

impl<TSymbol> Table<TSymbol>
where
    TSymbol: std::marker::Copy + std::hash::Hash + std::cmp::Eq,
{
    pub(crate) fn empty() -> Table<TSymbol> {
        Table {
            total_symbols: 0,
            entries: vec![],
            entry_indices: Default::default(),
        }
    }

    pub(crate) fn add(&mut self, s: Option<TSymbol>) {
        match self.entry_indices.get(&s) {
            Some(i) => {
                let index = *i;
                let entry = &mut self.entries[index];
                entry.frequency += 1;
                self.sort_entry(index);
            }

            None => {
                let index = self.entries.len();

                self.entries.push(TableEntry {
                    frequency: 1,
                    symbol: s,
                });

                self.entry_indices.insert(s, index);
            }
        };

        self.total_symbols += 1;
    }

    pub(crate) fn most_frequent(&self) -> Option<&TSymbol> {
        match self.entries.first() {
            Some(e) => e.symbol.as_ref(),
            None => None,
        }
    }

    pub(crate) fn sample(&self, sample_value: f64) -> Option<&TSymbol> {
        let mut remaining = (sample_value * self.total_symbols as f64) as usize;

        for entry in &self.entries {
            if remaining < entry.frequency {
                return entry.symbol.as_ref();
            }
            remaining -= entry.frequency;
        }

        None
    }

    fn sort_entry(&mut self, index: usize) {
        let mut j = index;

        for i in (0..index).rev() {
            if self.entries[j].frequency <= self.entries[i].frequency {
                break;
            }

            let tmp = self.entries[i];
            self.entries[i] = self.entries[j];
            self.entries[j] = tmp;
            j = i;
        }

        for i in j..=index {
            self.entry_indices.insert(self.entries[i].symbol, i);
        }
    }
}

#[derive(Copy, Clone)]
struct TableEntry<TSymbol> {
    frequency: usize,
    symbol: Option<TSymbol>,
}

#[cfg(test)]
mod test {
    use crate::table::Table;

    #[test]
    fn it_initialises_an_empty_table() {
        let t = Table::<i32>::empty();

        assert!(t.entries.is_empty());
        assert!(t.entry_indices.is_empty());
        assert_eq!(t.total_symbols, 0);
    }

    #[test]
    fn it_tracks_frequency_of_added_symbols() {
        let mut t = Table::empty();

        t.add(Some('a'));

        let entry = t.entries[*t.entry_indices.get(&Some('a')).unwrap()];
        assert_eq!(entry.frequency, 1);
        assert_eq!(entry.symbol, Some('a'));

        t.add(Some('b'));

        let entry = t.entries[*t.entry_indices.get(&Some('a')).unwrap()];
        assert_eq!(entry.frequency, 1);
        assert_eq!(entry.symbol, Some('a'));

        let entry = t.entries[*t.entry_indices.get(&Some('b')).unwrap()];
        assert_eq!(entry.frequency, 1);
        assert_eq!(entry.symbol, Some('b'));

        t.add(Some('a'));

        let entry = t.entries[*t.entry_indices.get(&Some('a')).unwrap()];
        assert_eq!(entry.frequency, 2);
        assert_eq!(entry.symbol, Some('a'));

        let entry = t.entries[*t.entry_indices.get(&Some('b')).unwrap()];
        assert_eq!(entry.frequency, 1);
        assert_eq!(entry.symbol, Some('b'));
    }

    #[test]
    fn it_tracks_total_added_symbols() {
        let mut t = Table::empty();

        t.add(Some('a'));
        assert_eq!(t.total_symbols, 1);

        t.add(Some('b'));
        assert_eq!(t.total_symbols, 2);

        t.add(Some('a'));
        assert_eq!(t.total_symbols, 3);
    }

    #[test]
    fn it_exposes_most_frequent_symbol() {
        let mut t = Table::empty();

        assert_eq!(t.most_frequent(), None);

        t.add(Some('a'));
        assert_eq!(t.most_frequent(), Some(&'a'));

        t.add(Some('b'));
        assert_eq!(t.most_frequent(), Some(&'a'));

        t.add(Some('c'));
        assert_eq!(t.most_frequent(), Some(&'a'));

        t.add(Some('b'));
        assert_eq!(t.most_frequent(), Some(&'b'));

        t.add(Some('c'));
        assert_eq!(t.most_frequent(), Some(&'b'));

        t.add(Some('c'));
        assert_eq!(t.most_frequent(), Some(&'c'));
    }

    #[test]
    fn it_allows_sampling_of_symbols() {
        let mut t = Table::empty();

        assert_eq!(t.sample(0.0), None);

        t.add(Some('a'));
        assert_eq!(t.sample(0.0), Some(&'a'));

        t.add(Some('b'));
        assert_eq!(t.sample(0.0), Some(&'a'));
        assert_eq!(t.sample(0.5), Some(&'b'));

        t.add(Some('c'));
        assert_eq!(t.sample(0.0), Some(&'a'));
        assert_eq!(t.sample(0.34), Some(&'b'));
        assert_eq!(t.sample(0.67), Some(&'c'));

        t.add(Some('b'));
        assert_eq!(t.sample(0.0), Some(&'b'));
        assert_eq!(t.sample(0.5), Some(&'a'));
        assert_eq!(t.sample(0.75), Some(&'c'));

        t.add(Some('c'));
        assert_eq!(t.sample(0.0), Some(&'b'));
        assert_eq!(t.sample(0.4), Some(&'c'));
        assert_eq!(t.sample(0.8), Some(&'a'));

        t.add(Some('c'));

        assert_eq!(t.sample(0.0), Some(&'c'));
    }
}
