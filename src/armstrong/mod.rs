/// print_is_armstrong prints if the given number is an armstrong number.
/// An armstrong number is one where the sum of the numbers to the power of the
/// length of the number is equal to the number.
/// # Example
/// ```rust
/// let foo: i64 = 153;
/// assert_eq!(foo, 1*1*1 + 5*5*5 + 3*3*3);
/// ```
pub fn print_is_armstrong(n: i64)
{
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

    // Print that if it is an armstrong number iff
    // the sum is equal to the number given.
    let result = if sum == n { "" } else { "not " };
    println!("{} is {}an armstrong number!", n, result);
}
