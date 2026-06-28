use itertools::Itertools;
use std::{collections::HashMap, sync::Arc};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Element<T> {
    Unit { of: T },
    Repetition { of: Arc<Self> },
    Sequence { of: Vec<Arc<Self>> },
}

impl<T: ToString> ToString for Element<T> {
    fn to_string(&self) -> String {
        match self {
            Element::Unit { of } => of.to_string(),
            Element::Repetition { of } => format!("Rep[{}]", of.to_string()),
            Element::Sequence { of } => {
                format!("Seq[{}]", of.iter().map(|v| v.to_string()).join(","))
            }
        }
    }
}

struct Lens<T: Clone> {
    width: usize,
    filters: Vec<Box<dyn Filter<T>>>,
}

impl<T: Clone> Lens<T> {
    fn view(self, data: &[Element<T>]) -> Viewing<T> {
        let content = data
            .windows(self.width)
            // .circular_array_windows::<WIDTH>()
            .enumerate()
            .map(|(i, window)| {
                (
                    i,
                    self.filters
                        .iter()
                        .filter_map(|f| f.filter(window.clone()))
                        .collect_vec(),
                )
            })
            .collect_vec();

        let mut results: HashMap<usize, Vec<Element<_>>> = HashMap::new();

        for (i, mut v) in content {
            if v.is_empty() {
                continue;
            }
            results
                .entry(i)
                .and_modify(|map_v| map_v.append(&mut v))
                .or_insert_with(|| v);
        }

        Viewing {
            width: self.width,
            results,
        }
    }
}

trait Filter<T> {
    fn filter(&self, window: &[Element<T>]) -> Option<Element<T>>;
}

struct RepeatFilter;

impl<T: PartialEq + Clone> Filter<T> for RepeatFilter {
    fn filter(&self, window: &[Element<T>]) -> Option<Element<T>> {
        if window.iter().all_equal() {
            Some(Element::Repetition {
                of: Arc::new(window.first().unwrap().clone()),
            })
        } else {
            None
        }
    }
}

struct SequenceFilter;

impl<T: PartialEq + Clone + std::fmt::Debug> Filter<T> for SequenceFilter {
    fn filter(&self, window: &[Element<T>]) -> Option<Element<T>> {
        if window
            .windows(2)
            .map(|slice| slice[0] != slice[1])
            .all_equal()
        {
            Some(Element::Sequence {
                of: window.into_iter().map(|v| Arc::new(v.clone())).collect(),
            })
        } else {
            None
        }
    }
}

struct Viewing<T> {
    width: usize,
    results: HashMap<usize, Vec<Element<T>>>,
}

impl<T: ToString> Viewing<T> {
    fn pretty_print(&self) {
        println!("Viewing size {}", self.width);
        for (i, v) in self.results.iter() {
            println!("pos: {i}");
            let elements = v.iter().map(|e| e.to_string()).join(", ");
            print!("\t {elements}");
            println!("\n")
        }
    }
}

#[cfg(test)]
mod tests {
    use bitvec::vec::BitVec;
    use itertools::Itertools;
    use rand::RngExt;

    use crate::algo::elements::{Element, Lens, RepeatFilter, SequenceFilter};

    pub fn random_signal() -> BitVec {
        let mut rng = rand::rng();
        const LEN: usize = 128;
        let signal: BitVec = {
            let mut s = BitVec::new();

            for _ in 0..LEN {
                let b = rng.random_range(0..2) == 0;
                s.push(b)
            }
            s
        };
        println!("Signal: {:?}", signal);
        signal
    }

    #[test]
    fn repfilter() {
        let signal = random_signal();
        let element_ified = signal.iter().map(|v| Element::Unit { of: v }).collect_vec();

        for i in 3..10 {
            let lens: Lens<_> = Lens {
                width: i,
                filters: vec![Box::new(RepeatFilter)],
            };
            let res = lens.view(&element_ified);

            res.pretty_print();
        }
    }

    #[test]
    fn seqfilter() {
        let signal = random_signal();
        let element_ified = signal.iter().map(|v| Element::Unit { of: v }).collect_vec();

        for i in 4..5 {
            let lens: Lens<_> = Lens {
                width: i,
                filters: vec![Box::new(SequenceFilter)],
            };
            let res = lens.view(&element_ified);

            res.pretty_print();
        }
    }
}
