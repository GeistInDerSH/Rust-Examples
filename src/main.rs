mod bottles99;
mod armstrong;
mod magic8ball;
mod pythagoreantriples;
mod rock_paper_scissors;
mod multiplication_tables;

fn list_modules()
{
    println!("Available Options:");
    println!("0) 99 Bottles of beer");
    println!("1) Armstrong Number");
    println!("2) Magic 8 Ball");
    println!("3) Pythagorean Triples");
    println!("4) Rock Paper Scissors");
    println!("5) Multiplication Tables");
}

fn main()
{
    list_modules();
    println!("Enter your selection: ");
    let mut user_input = String::new();
    std::io::stdin().read_line(&mut user_input).unwrap();

    user_input.truncate(user_input.len() - 1);

    match user_input.as_str() {
        "0" => {
            bottles99::print_bottles(99);
        },
        "1" => {
            user_input = "".to_string();
            std::io::stdin().read_line(&mut user_input).unwrap();
            user_input.truncate(user_input.len() - 1);
            println!("{}", user_input);
            armstrong::print_is_armstrong(user_input.parse().unwrap());
        },
        "2" => {
            magic8ball::magic_8_ball();
        },
        "3" => {
            pythagoreantriples::pythagorean_triple();
        },
        "4" => {
            rock_paper_scissors::start();
        },
        "5" => {
            multiplication_tables::start();
        },

        _ => {println!("{}", user_input.as_str());},
    }
}
