trait Length {
    fn len(&self) -> usize;
}

impl Length for usize {
    fn len(&self) -> usize {
        self.to_string().len()
    }
}

/// Checks if the number entered contains four digits.
fn is_four_digits(number: usize) -> bool {
    if number.len() == 4 {
        return true;
    }
    false
}

/// Sorts the all digits from largest to smallest
fn sort(number: usize) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn is_four_digits_true() {
        let number = 1234;
        let result = is_four_digits(number);
        assert_eq!(result, true);
    }

    #[test]
    fn is_four_digits_false_3() {
        let number = 123;
        let result = is_four_digits(number);
        assert_eq!(result, false);
    }

    #[test]
    fn is_four_digits_false_6() {
        let number = 123456;
        let result = is_four_digits(number);
        assert_eq!(result, false);
    }
}
