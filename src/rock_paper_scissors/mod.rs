use rand::{Rng};

#[derive(PartialEq)]
enum RPS
{
    Rock = 0,
    Paper = 1,
    Scissors = 2,
    Error = 3,
}

struct Score
{
    user_score: u16,
    npc_score: u16,
}

impl Score
{
    fn new() -> Score
    {
        Score { user_score: 0, npc_score: 0, }
    }

    fn npc_wins(&mut self)
    {
        self.npc_score += 1;
    }

    fn user_wins(&mut self)
    {
        self.user_score += 1;
    }

    fn print_wins(&self)
    {
        println!("You: {} -- NPC: {}", self.user_score, self.npc_score);
    }
}

pub fn start()
{
    let mut score = Score::new();

    loop
    {
        let npc = get_npc_choice();

        score.print_wins();
        println!("Enter your choice:\n\tRock: r\n\tPaper: p\n\tScissors: s\n\tQuit: q");
        let user = get_user_input();

        if user == RPS::Error { break; }

        if npc == user
        {
            println!("Draw!");
        } else
        {
            match user
            {
                RPS::Rock => {
                    if npc == RPS::Paper
                    {
                        println!("Paper beats Rock. NPC wins");
                        score.npc_wins();
                    }
                    else
                    {
                        println!("Rock beats Scissors. You win");
                        score.user_wins();
                    }
                },
                RPS::Paper => {
                    if npc == RPS::Scissors
                    {
                        println!("Scissors beats Paper. NPC wins");
                        score.npc_wins();
                    }
                    else
                    {
                        println!("Scissors beats Paper. You win");
                        score.user_wins();
                    }
                },
                RPS::Scissors => {
                    if npc == RPS::Rock
                    {
                        println!("Rock beats Scissors. NPC wins");
                        score.npc_wins();
                    }
                    else
                    {
                        println!("Scissors beats Paper. You win");
                        score.user_wins();
                    }
                },
                _ => { break; }
            }
        }

    }
}

fn get_user_input() -> RPS
{
    let mut user_input = String::new();
    std::io::stdin().read_line(&mut user_input).unwrap();
    user_input.truncate(user_input.len() - 1);

    match user_input.as_str()
    {
        "r" => { return RPS::Rock; },
        "p" => { return RPS::Paper; },
        "s" => { return RPS::Scissors; },
        _ => { return RPS::Error; },
    }
}

fn get_npc_choice() -> RPS
{
    let rng: i8 = rand::thread_rng().gen();

    match rng % 3
    {
        0 => { return RPS::Rock; },
        1 => { return RPS::Paper; },
        2 => { return RPS::Scissors; },
        _ => { return RPS::Error; },
    }
}
