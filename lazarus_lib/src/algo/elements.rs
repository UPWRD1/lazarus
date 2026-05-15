use std::simd::u32x64;

use rand::RngExt;

pub trait Element {
    // fn value(&self);
}

struct Repetition<T, const N: usize>
where
    T: Element,
{
    of: T,
    len: usize,
}

struct Sequence<const N: usize> {
    of: [Box<dyn Element>; N],
}

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

pub fn autocorrelate_simd(signal: impl Into<std::simd::u32x64>, lag: u32) -> u32 {
    let signal = signal.into();
    let lag = std::simd::u32x64::splat(lag);

    // Shift the signal to create the lagged version
    let shifted = signal >> lag;

    // XNOR finds matching bits.
    // We mask it because the shift introduced 0s at the top, which aren't part of the signal overlap.
    let valid_bits_mask = !u32x64::splat(0) >> lag;
    let matching_bits = !(signal ^ shifted) & valid_bits_mask;

    // Count how many bits matched over the overlapping window
    matching_bits
        .to_array()
        .iter()
        .map(|a| a.count_ones())
        .sum()
}

pub fn analyze(elems: &[Box<dyn Element>]) -> Box<dyn Element> {
    todo!()
}

#[cfg(test)]
mod tests {
    use rand::RngExt;

    use crate::algo::elements::autocorrelate_simd;

    #[test]
    fn test1() {
        // Example 64-bit signal
        let mut rng = rand::rng();
        let signal: std::simd::u32x64 = rng.random();
        println!("Signal: {:?}", signal.to_array());
        println!("Lag 0 (Perfect match): {}", autocorrelate_simd(signal, 0)); // Outputs 64
        println!("Lag 1 match count:     {}", autocorrelate_simd(signal, 1));
        println!("Lag 2 match count:     {}", autocorrelate_simd(signal, 2));

        // let ac = crate::algo::elements::autocorrelate(&signal);

        // println!("Original signal: {:?}", signal);
        // println!("Autocorrelation: ");
        for lag in 0..32 {
            let value = autocorrelate_simd(signal, lag);
            println!("Lag {}: {:.4}", lag, value);
        }
    }
}
