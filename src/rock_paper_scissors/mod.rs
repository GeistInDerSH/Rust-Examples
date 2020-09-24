use rand::{Rng};

#[derive(PartialEq)]
// RPS enum is an enum for holding each of the users options, and an error
enum RPS
{
    Rock = 0,
    Paper = 1,
    Scissors = 2,
    Error = 3,
}

// Score holds the score for both the user and the cpu.
struct Score
{
    user_score: u16,
    npc_score: u16,
}

impl Score
{
    // new is used to start a new challenged.
    fn new() -> Score
    {
        Score { user_score: 0, npc_score: 0, }
    }

    // npc_wins increments the npc's win counter by 1.
    fn npc_wins(&mut self)
    {
        self.npc_score += 1;
    }

    // user_wins increments the user's win counter by 1.
    fn user_wins(&mut self)
    {
        self.user_score += 1;
    }

    // print_wins prints the wins for both the user and the npc.
    fn print_wins(&self)
    {
        println!("You: {} -- NPC: {}", self.user_score, self.npc_score);
    }
}

// start is the entrypoint for this module. A new match of rock paper scissors
// is created between an npc and the user.
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
                    // User: Rock
                    // NPC: Paper
                    // NPC Wins
                    if npc == RPS::Paper
                    {
                        println!("Paper beats Rock. NPC wins");
                        score.npc_wins();
                    }
                    // User: Rock
                    // NPC: Scissors
                    // User wins
                    else
                    {
                        println!("Rock beats Scissors. You win");
                        score.user_wins();
                    }
                },
                RPS::Paper => {
                    // User: Paper
                    // NPC: Scissors
                    // NPC Wins
                    if npc == RPS::Scissors
                    {
                        println!("Scissors beats Paper. NPC wins");
                        score.npc_wins();
                    }
                    // User: Paper
                    // NPC: Rock
                    // User wins
                    else
                    {
                        println!("Scissors beats Paper. You win");
                        score.user_wins();
                    }
                },
                RPS::Scissors => {
                    // User: Scissors
                    // NPC: Rock
                    // NPC Wins
                    if npc == RPS::Rock
                    {
                        println!("Rock beats Scissors. NPC wins");
                        score.npc_wins();
                    }
                    // User: Scissors
                    // NPC: Paper
                    // User wins
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

// get_user_input reads the users input from stdin and returns which option
// (Rock, Paper, or Scissors) was entered.
fn get_user_input() -> RPS
{
    let mut user_input = String::new();
    std::io::stdin().read_line(&mut user_input).unwrap();
    user_input.truncate(user_input.len() - 1);

    match user_input.as_str()
    {
        "r" | "rock" => { return RPS::Rock; },
        "p" | "paper" => { return RPS::Paper; },
        "s" | "scissors" => { return RPS::Scissors; },
        _ => { return RPS::Error; },
    }
}

// get_npc_choice randomly selects one from Rock, Paper, & Scissors with equal
// weight.
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
