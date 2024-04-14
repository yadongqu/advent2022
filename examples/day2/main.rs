
#[derive(PartialEq, Clone, Copy)]
enum Choice {
    Rock,
    Paper,
    Scissor,
}

enum Status {
    Win,
    Lose,
    Draw
}

const WINNING_PAIR:[(Choice, Choice); 3] = [(Choice::Rock, Choice::Scissor),(Choice::Paper, Choice::Rock), (Choice::Scissor, Choice::Paper)];


fn calculate_result(player1: Choice, player2: Choice) -> Status {
   if player1 == player2 {
     return Status::Draw;
   }
   if WINNING_PAIR.contains(&(player1, player2)) {
    return Status::Win;
   }
   return Status::Lose;
}

fn calculate_score(player1: Choice, player2: Choice) -> u32 {
    let mut score:u32 = match calculate_result(player1, player2) {
        Status::Win => 6,
        Status::Draw => 3,
        Status::Lose => 0
    };
    score += match player1 {
        Choice::Rock => 1,
        Choice::Paper => 2,
        Choice::Scissor => 3
    };
    score
}

fn calculate_choice(choice: &str) -> Choice {
    match  choice {
        "A" | "X" => Choice::Rock,
        "B" | "Y" => Choice::Paper,
        "C" | "Z" => Choice::Scissor,
        _ => panic!("something is wrong")
    }
}

fn part1(content: &str) -> u32 {
    let sum = content.split("\n").fold(0, |mut sum, line| {
        let choices : Vec<&str> = line.split(" ").collect();
        let opponent = calculate_choice(choices[0]);
        let you = calculate_choice(choices[1]);
        sum += calculate_score(you, opponent);
        return sum;
    });
    return sum;
}

fn status_from_play(play: &str) -> Status {
    match play {
        "X" => Status::Lose,
        "Y" => Status::Draw,
        "Z" => Status::Win,
        _ => panic!("something is wrong")
    }
}

fn choice_from_status_and_opponent(opponent: &Choice, status: &Status) -> Choice {
    match status {
        Status::Draw => return opponent.clone(),
        Status::Win => match &opponent {
            Choice::Rock => Choice::Paper,
            Choice::Scissor => Choice::Rock,
            Choice::Paper => Choice::Scissor,
        },
        Status::Lose => match &opponent {
            Choice::Rock => Choice::Scissor,
            Choice::Scissor => Choice::Paper,
            Choice::Paper => Choice::Rock
        }
    }
}

fn part2(content: &str) -> u32 {
    let sum = content.split("\n").fold(0, |mut sum, line| {
        let choices : Vec<&str> = line.split(" ").collect();
        let opponent = calculate_choice(choices[0]);
        let status = status_from_play(choices[1]);
        sum += match &status {
            Status::Draw => 3,
            Status::Lose => 0,
            Status::Win => 6,
        };
        sum += match choice_from_status_and_opponent(&opponent, &status) {
            Choice::Rock => 1,
            Choice::Paper => 2,
            Choice::Scissor => 3
        };
        return sum;
    });
    return sum;
}


fn main() {
    
    let content = include_str!("example.txt");
    let score = part1(content);
    println!("{score}");
    let score2 = part2(content);
    println!("{score2}");
}