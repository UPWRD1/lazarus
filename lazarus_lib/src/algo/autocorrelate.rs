use bitvec::prelude::*;

use crate::algo::elements::Element;

pub fn autocorrelate(signal: &[f32]) -> Vec<f32> {
    let n = signal.len();
    if n == 0 {
        return Vec::new();
    }

    let mut results = Vec::with_capacity(n);

    // Iterate through each possible lag (k)
    for lag in 0..n {
        let mut sum = 0.0;

        // Multiply each element by its lagged counterpart and sum them up
        for i in 0..(n - lag) {
            sum += signal[i] * signal[i + lag];
        }

        results.push(sum);
    }

    results
}

pub fn autocorrelation(signal: &BitVec<usize, Lsb0>) -> Vec<f64> {
    let n = signal.len();
    if n == 0 {
        return vec![];
    }

    // Raw u64 storage words for fast bitwise access.
    let words: &[usize] = signal.as_raw_slice();
    let num_words = words.len();

    (0..n)
        .map(|lag| {
            let overlap = n - lag; // number of valid bit positions
            let agreements = xnor_popcount(words, num_words, lag, overlap);
            // normalize: agreements in [0, overlap] → r in [-1, 1]
            (2.0 * agreements as f64 - overlap as f64) / overlap as f64
        })
        .collect()
}

/// Counts the number of bit positions where `signal` and `signal >> lag` agree,
/// over `overlap` bits, entirely using word-level XNOR + popcount.
///
/// Bit layout: the signal is stored LSB-first in `words[0]`, so shifting
/// right by `lag` bits means shifting the *word array* right by `lag` bits.
fn xnor_popcount(words: &[usize], num_words: usize, lag: usize, overlap: usize) -> usize {
    if overlap == 0 {
        return 0;
    }

    let word_shift = lag / 64; // whole-word offset
    let bit_shift = lag % 64; // sub-word bit offset
    let remaining_shift = 64usize.wrapping_sub(bit_shift) % 64;

    let mut count: usize = 0;
    let mut bits_counted = 0usize;

    // Iterate over the "original" word positions that have an overlapping
    // "shifted" counterpart.
    for i in 0..num_words {
        let j = i + word_shift; // index into shifted signal
        if j >= num_words {
            break;
        }

        // Reconstruct the shifted word at position i:
        //   shifted[i] = words[j] >> bit_shift | words[j+1] << remaining_shift
        let shifted_word = if bit_shift == 0 {
            words[j]
        } else {
            let lo = words[j] >> bit_shift;
            let hi = if j + 1 < num_words {
                words[j + 1] << remaining_shift
            } else {
                0
            };
            lo | hi
        };

        let xnor = !(words[i] ^ shifted_word); // 1 where bits agree

        // How many bits of this word pair are within [0, overlap)?
        let bits_this_word = 64usize.min(overlap - bits_counted);

        let mask: usize = if bits_this_word == 64 {
            usize::MAX
        } else {
            (1usize << bits_this_word) - 1
        };

        count += (xnor & mask).count_ones() as usize;
        bits_counted += bits_this_word;

        if bits_counted >= overlap {
            break;
        }
    }

    count
}
pub fn analyze<T>(elems: &[Element<T>]) -> Element<T> {
    todo!()
}

#[cfg(test)]
mod tests {
    use bitvec::prelude::*;
    use rand::RngExt;

    use crate::algo::autocorrelate::autocorrelation;

    #[test]
    fn test1() {
        use plotly::{Plot, Scatter};

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
        let autocorrelations = autocorrelation(&signal);
        // println!("Autocorrelations: {:?}", autocorrelations);
        let mut plot = Plot::new();
        let x_axis: Vec<usize> = (0..autocorrelations.len()).collect();

        let trace = Scatter::new(x_axis, autocorrelations.to_vec());
        plot.add_trace(trace);

        plot.write_html("out.html");
    }
}
