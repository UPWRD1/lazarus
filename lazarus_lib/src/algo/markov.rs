use markov::Chain;
pub trait Factor {
    fn weight(&self) -> f32;
}

pub trait Likelyhood {
    fn factors() -> Vec<Box<dyn Factor>>;
    fn chance() -> f32;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn markov_test() {
        let mut chain = Chain::new();
        chain.feed([302, 201, 32, 302]).feed([302, 403, 32]);
        for i in 0..3 {
            println!("{:?}", chain.generate());
        }
    }
}
