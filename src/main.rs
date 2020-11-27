extern crate ncurses;
use ncurses::*;

mod bottles99;
mod armstrong;
mod magic8ball;
mod pythagoreantriples;
mod rock_paper_scissors;
mod multiplication_tables;

/// Takes an array of strings with the options and the current position being selected
/// and prints them out in numeric order with a right carrot to denote the currently
/// selected option
/// ```
/// 1. Option 1
/// >  Option 2
/// 3. Option 3
/// 4. Option 4
/// ```
fn print_options(options: [&'static str; 6], pos: i8)
{
    let mut i = 0;
    addstr("Available Options:\n");
    for s in options.iter()
    {
        let marker = if pos == i { ">" } else { "" };
        addstr(&format!("{0: <3} {1:}\n", marker, s));
        i += 1;
    }
}

/// lists all modules that the user can choose from when running the program.
/// Returns to number of the module that the user has selected.
fn list_modules() -> i8
{

    let options = [
        "99 Bottles of beer",
        "Armstrong Number",
        "Magic 8 Ball",
        "Pythagorean Triples",
        "Rock Paper Scissors",
        "Multiplication Tables"
    ];

    initscr();

    let mut pos: i8 = 0;
    print_options(options, pos);

    loop
    {
        let c = std::char::from_u32(getch() as u32).unwrap();
        clear();

        match c
        {
            'w' => { pos = if pos == 0 { options.len() as i8  -1 } else { pos - 1 } },
            's' => { pos = (pos + 1) % options.len() as i8; },
            '\n' => { break; },
            _ => {},
        }

        addstr("Available Options:\n");

        let mut i = 0;
        for s in options.iter()
        {
            let marker = if pos == i { ">" } else { "" };
            addstr(&format!("{0: <3} {1:}\n", marker, s));
            i += 1
        }

        refresh();
    }
    endwin();

    return pos;
}

fn main()
{
    let choice = list_modules();

    match choice {
        0 => {
            bottles99::print_bottles(99);
        },
        1 => {
            let mut user_input = String::new();
            std::io::stdin().read_line(&mut user_input).unwrap();
            user_input.truncate(user_input.len() - 1);
            armstrong::print_is_armstrong(user_input.parse().unwrap());
        },
        2 => {
            magic8ball::magic_8_ball();
        },
        3 => {
            pythagoreantriples::pythagorean_triple();
        },
        4 => {
            rock_paper_scissors::start();
        },
        5 => {
            multiplication_tables::start();
        },

        _ => {},
    }
}
