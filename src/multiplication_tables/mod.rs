/// start is the entrypoint to the module. It will get the length of the table
/// and then print the table in a formatted output.
pub fn start()
{
    let size = get_size_from_user();
    print_formatted(size);
}

/// print_formatted prints the multiplication table with 7 spaces between each
/// column. This is done to ensure that the numbers fit inside the box without
/// overflowing, thus each column is aligned.
fn print_formatted(s: u8)
{
    for i in 0..s+1
    {
        if i == 0
        {
            print!("{0: <7} |", "");
        }
        else
        {
            print!("{0: <7} |", i);
        }
    }

    println!();

    for i in 1..s+1
    {
        for j in 0..s+1
        {
            if j == 0
            {
                print!("{0: <7} |", i);
            }
            else
            {
                print!("{0: <7} |", j * i);
            }
        }
        println!();
    }
}

/// get_size_from_user reads in the size of input from the user for the
/// multiplication table.
fn get_size_from_user() -> u8
{
    let mut user_input = String::new();
    std::io::stdin().read_line(&mut user_input).unwrap();
    user_input.truncate(user_input.len() - 1);

    return user_input.parse().unwrap();
}
