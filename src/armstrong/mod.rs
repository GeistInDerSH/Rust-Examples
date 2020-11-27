/// Check to see if the given number is an armstrong number.
///
/// An armstrong number is one where the sum of the numbers to the power of the
/// length of the number is equal to the number.
///
/// # Example:
/// ```
/// let foo: i64 = 153;
/// assert_eq!(foo, 1*1*1 + 5*5*5 + 3*3*3);
/// ```
fn is_armstrong(n: i64) -> bool {
    // Convert the number to a string so we can get each position of the number
    let n_str = n.to_string();

    // Get the length of the string
    let len = n_str.len();

    let mut sum: i64 = 0;
    for i in 0..len
    {
        // Get the nth numer in the string
        let c = n_str.chars().nth(i).unwrap();

        // Convert that char to a number
        let val = c.to_digit(10).unwrap() as i64;

        // Add the number to the power of the string length
        sum += val.pow(len as u32);
    }

    sum == n
}

/// Prints if the given number is an armstrong number.
pub fn print_is_armstrong(n: i64)
{
    let result = if is_armstrong(n) { "" } else { "not " };
    println!("{} is {}an armstrong number!", n, result);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn is_armstrong_0() {
        let armstrong_number: i64 = 0;
        let result: bool = is_armstrong(armstrong_number);
        assert_eq!(true, result);
    }

    #[test]
    fn is_not_armstrong_120() {
        let armstrong_number: i64 = 120;
        let result: bool = is_armstrong(armstrong_number);
        assert_eq!(false, result);
    }

    #[test]
    fn is_armstrong_153() {
        let armstrong_number: i64 = 153;
        let result: bool = is_armstrong(armstrong_number);
        assert_eq!(true, result);
    }

    #[test]
    fn is_not_armstrong_241() {
        let armstrong_number: i64 = 241;
        let result: bool = is_armstrong(armstrong_number);
        assert_eq!(false, result);
    }

    #[test]
    fn is_armstrong_370() {
        let armstrong_number: i64 = 370;
        let result: bool = is_armstrong(armstrong_number);
        assert_eq!(true, result);
    }

    #[test]
    fn is_armstrong_1634() {
        let armstrong_number: i64 = 1634;
        let result: bool = is_armstrong(armstrong_number);
        assert_eq!(true, result);
    }

}
