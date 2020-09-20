use rand::{Rng};
use std::time::Duration;
use std::thread;
use std::io::{self, Write};

pub fn magic_8_ball()
{
    loop {
        print_menu();
        let quit = get_input();
        if quit == "q" { break; }

        // Get users question and just ignore it
        println!("What is your question?:");
        let _ = get_input();

        think();

        // Get the response to the question.
        let response = get_response();

        println!("{}", response);
    }
}

fn think()
{
    print!("Thinking");
    for _ in 0..3
    {
        print!(".");
        io::stdout().flush().unwrap();
        thread::sleep(Duration::from_secs(1));
    }
    println!("");
}

fn print_menu()
{
    println!("Options:\na: Ask a question\nq: quit");
}

fn get_input() -> String
{
    let mut user_input = String::new();
    std::io::stdin().read_line(&mut user_input).unwrap();
    user_input.truncate(user_input.len() - 1);
    return user_input;
}

fn get_response() -> String
{
    let response: [&str; 20] = [
        "It is certain",
        "Without a doubt",
        "You may rely on it",
        "Yes definitely",
        "It is decidedly so",
        "As I see it, yes",
        "Most likely",
        "Yes",
        "Outlook good",
        "Signs point to yes",
        "Reply hazy try again",
        "Better not tell you now",
        "Ask again later",
        "Cannot predict now",
        "Concentrate and ask again",
        "Don’t count on it",
        "Outlook not so good",
        "My sources say no",
        "Very doubtful",
        "My reply is no"
    ];

    let rng: u8 = rand::thread_rng().gen();
    let i: u8 = rng % 20;

    return response[i as usize].to_string();

}
