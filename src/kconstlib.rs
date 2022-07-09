const FOUR_DIGITS: usize = 4;

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
    if number.len() == FOUR_DIGITS {
        return true;
    }
    false
}

/// Checks to see if the digits varies, ie. all the digits passed in are not the same.
fn is_veried(number: usize) -> bool {
    let items: Vec<char> = number.to_string().chars().map(|x| x).collect();

    let first = items[0];
    let varied: bool = items.iter().all(|&char| char == first);

    // return the inverse of varied since we want true if it _is_ varried
    !varied
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

    #[test]
    fn is_veried_true() {
        let number = 1234;
        let result = is_veried(number);

        assert_eq!(result, true);
    }

    #[test]
    fn is_veried_true2() {
        let number = 1112;
        let result = is_veried(number);

        assert_eq!(result, true);
    }

    #[test]
    fn is_veried_false() {
        let number = 1111;
        let result = is_veried(number);

        assert_eq!(result, false);
    }
}
