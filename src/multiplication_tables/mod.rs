pub fn start()
{
    let size = get_size_from_user();
    print_formatted(size);
}

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

fn get_size_from_user() -> u8
{
    let mut user_input = String::new();
    std::io::stdin().read_line(&mut user_input).unwrap();
    user_input.truncate(user_input.len() - 1);

    return user_input.parse().unwrap();
}
