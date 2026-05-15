#![feature(associated_type_defaults)]
#![feature(portable_simd)]
pub mod algo;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

// mod repr {
//     #[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
//     pub(super) struct Tokens<V> {
//         tokens: Vec<V>,
//     }

//     impl<V> Tokens<V> {
//         fn next()
//     }

//     pub(super) trait IntoTokens {
//         type TokenValue;
//         fn into(self) -> Tokens<Self::TokenValue>;
//     }

//     impl IntoTokens for String {
//         type TokenValue = char;

//         fn into(self) -> Tokens<Self::TokenValue> {
//             Tokens {
//                 tokens: self.chars().collect(),
//             }
//         }
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
