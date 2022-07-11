const FOUR_DIGITS: usize = 4;
const KAPREKAR_CONSTANT: usize = 6174;

//TODO: `usize` removes leading zeros. Need to change from using usize to String.
pub(super) fn kconst(number: usize) -> String {
    let mut steps: usize = 0;
    let mut number: usize = number;

    while number.is_four_digits() && number.is_varied() {
        if number == KAPREKAR_CONSTANT {
            return format!("{}", steps);
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
//TODO: Return a String instead of usize.
// TODO: Fix everything that breaks.
fn subtract(number: usize) -> usize {
    let number: usize = sort_desend(number) - sort_asend(number);
    let number: usize = dbg!(format!("{:04}", number))
        .parse()
        .expect("Unable to parse formatted string");

    dbg!(number)
}

#[cfg(test)]
mod tests {

    use std::vec;

    use super::*;

    #[test]
    fn kconst_1() {
        let number: usize = 4176;
        assert_eq!(kconst(number), 1.to_string());
    }

    #[test]
    fn kconst_2() {
        let number: usize = 6264;
        assert_eq!(kconst(number), 2.to_string());
    }

    #[test]
    fn kconst_3() {
        let number: usize = 1492;
        assert_eq!(kconst(number), 7.to_string());
    }

    #[test]
    fn kconst_4() {
        let number: usize = 3165;
        assert_eq!(kconst(number), 7.to_string());
    }

    #[test]
    fn kconst_5() {
        let number: usize = 1000;
        assert_eq!(kconst(number), 5.to_string());
    }

    #[test]
    // Test passes if the lead `0` are kept after subtraction
    fn leading_zero_test() {
        let number: usize = 1000;

        let number = subtract(number);
        assert_eq!(number.is_four_digits(), true);
    }

    #[test]
    fn is_four_digits_true_1() {
        let number: usize = 1234;
        assert_eq!(number.is_four_digits(), true);
    }

    #[test]
    fn is_four_digits_true_2() {
        let number: usize = 1000;
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
    fn is_veried_true_1() {
        let number: usize = 1234;

        assert_eq!(number.is_varied(), true);
    }

    #[test]
    fn is_veried_true_2() {
        let number: usize = 1112;

        assert_eq!(number.is_varied(), true);
    }

    #[test]
    fn is_veried_true_3() {
        let number: usize = 1000;

        assert_eq!(number.is_varied(), true);
    }

    #[test]
    fn is_veried_false() {
        let number: usize = 1111;

        assert_eq!(number.is_varied(), false);
    }

    #[test]
    fn to_vec_pass() {
        let number: usize = 1234;
        let result: Vec<usize> = to_vec(number);

        assert_eq!(result, vec![1, 2, 3, 4]);
    }

    #[test]
    fn sort_asend_1() {
        let number: usize = 3512;

        assert_eq!(sort_asend(number), 1235);
    }

    #[test]
    fn sort_desend_1() {
        let number: usize = 3512;

        assert_eq!(sort_desend(number), 5321);
    }

    #[test]
    fn subtract_1() {
        let number: usize = 3512;

        assert_eq!(subtract(number), 4086);
    }

    #[test]
    fn subtract_2() {
        let number: usize = 9919;

        assert_eq!(subtract(number), 7992);
    }
}
