//! # Harshad
//! A Kaprekar library
mod kconstlib;

/// An implementation of Kaprekar's constant
///
/// Takes four digits and returns the number of iteration
/// until 6174 is found.
pub fn kconst(number: usize) -> String {
    kconstlib::kconst(number)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
