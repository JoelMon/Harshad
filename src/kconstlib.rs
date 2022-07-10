const FOUR_DIGITS: usize = 4;
const KAPREKAR_CONSTANT: usize = 6174;

//TODO: FIX LOGIC, SUBSTRACTION IS WRONG
fn kconst(number: usize) -> usize {
    let mut steps: usize = 0;
    let mut number = number;

    while number.is_four_digits() && number.is_varied() {
        if number == KAPREKAR_CONSTANT {
            return steps;
        } else {
            steps += 1;
            number = subtract(number);
        }
    }
    panic!("Something went wrong");
}

trait Length {
    /// Returns the total number of digits.
    fn len(&self) -> usize;
}

impl Length for usize {
    fn len(&self) -> usize {
        self.to_string().len()
    }
}

trait Varied {
    /// Checks to see if the digits varies, ie. all the digits passed in are not the same digit.
    fn is_varied(&self) -> bool;
}
impl Varied for usize {
    fn is_varied(&self) -> bool {
        let items: Vec<char> = self.to_string().chars().map(|x| x).collect();

        let first = items[0];
        let varied: bool = items.iter().all(|&char| char == first);

        // return the inverse of varied since we want true if it _is_ varied
        !varied
    }
}

trait FourDigits {
    /// Checks if the number entered contains four digits.
    fn is_four_digits(&self) -> bool;
}

impl FourDigits for usize {
    fn is_four_digits(&self) -> bool {
        if self.len() == FOUR_DIGITS {
            return true;
        }
        false
    }
}

/// Converts `usize` interger into a `Vec<usize>`
fn to_vec(number: usize) -> Vec<usize> {
    let item: Vec<usize> = number
        .to_string()
        .chars()
        .map(|i| String::from(i).parse().unwrap())
        .collect();
    item
}

/// Converts `Vec<usize>` to `usize`
fn to_usize(number: Vec<usize>) -> usize {
    let number: String = number.iter().map(|x| x.to_string()).collect();
    number
        .parse()
        .expect("number was not able to be parsed into a usize")
}

/// Sorts the all digits from largest to smallest
fn sort_asend(number: usize) -> usize {
    let mut num = to_vec(number);
    num.sort();
    to_usize(num)
}

/// Sorts the all digits from smallest to biggest
fn sort_desend(number: usize) -> usize {
    let mut num = to_vec(number);
    num.sort_by(|a, b| b.cmp(a));
    to_usize(num)
}

/// Takes a `usize` sorts it and subtracts
fn subtract(number: usize) -> usize {
    sort_desend(number) - sort_asend(number)
}

#[cfg(test)]
mod tests {

    use std::vec;

    use super::*;

    #[test]
    fn kconst_1() {
        let number: usize = 1492;
        assert_eq!(kconst(number), 3);
    }

    #[test]
    fn is_four_digits_true() {
        let number: usize = 1234;
        assert_eq!(number.is_four_digits(), true);
    }

    #[test]
    fn is_four_digits_false_3() {
        let number: usize = 123;
        assert_eq!(number.is_four_digits(), false);
    }

    #[test]
    fn is_four_digits_false_6() {
        let number: usize = 123456;
        assert_eq!(number.is_four_digits(), false);
    }

    #[test]
    fn is_veried_true() {
        let number: usize = 1234;

        assert_eq!(number.is_varied(), true);
    }

    #[test]
    fn is_veried_true2() {
        let number: usize = 1112;

        assert_eq!(number.is_varied(), true);
    }

    #[test]
    fn is_veried_false() {
        let number: usize = 1111;

        assert_eq!(number.is_varied(), false);
    }

    #[test]
    fn to_vec_pass() {
        let number = 1234;
        let result = to_vec(number);

        assert_eq!(result, vec![1, 2, 3, 4]);
    }

    #[test]
    fn sort_asend_1() {
        let number = 3512;
        let result = sort_asend(number);

        assert_eq!(result, 1235);
    }

    #[test]
    fn sort_desend_1() {
        let number = 3512;
        let result = sort_desend(number);

        assert_eq!(result, 5321);
    }

    #[test]
    fn subtract_1() {
        let number = 3512;
        let result = subtract(number);

        assert_eq!(result, 4086);
    }

    #[test]
    fn subtract_2() {
        let number = 9919;
        let result = subtract(number);

        assert_eq!(result, 7992);
    }
}
