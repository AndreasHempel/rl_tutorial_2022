use std::fmt::Debug;

#[derive(Debug)]
pub struct TableEntry<S> {
    name: S,
    weight: i32,
}

impl<S> TableEntry<S> {
    pub fn new(name: S, weight: i32) -> Self {
        TableEntry { name, weight }
    }
}

#[derive(Debug)]
pub struct RandomTable<S> {
    entries: Vec<TableEntry<S>>,
    total_weight: i32,
}

impl<S: Clone + Debug> RandomTable<S> {
    pub fn new() -> Self {
        RandomTable {
            entries: Vec::new(),
            total_weight: 0,
        }
    }

    pub fn add(mut self, name: S, weight: i32) -> Self {
        self.total_weight += weight;
        self.entries.push(TableEntry::new(name, weight));
        self
    }

    pub fn roll<R: rand::Rng>(&self, rng: &mut R) -> Option<S> {
        if self.total_weight == 0 {
            return None;
        }

        let mut roll = rng.gen_range(0..self.total_weight);
        for entry in self.entries.iter() {
            if roll < entry.weight {
                return Some(entry.name.clone());
            }
            roll -= entry.weight;
        }
        panic!(
            "Did not find an entry in {:?} with a roll of {}",
            self, roll
        );
    }
}
