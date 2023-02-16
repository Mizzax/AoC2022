use std::fs;
#[derive(PartialEq, Eq)]
enum Choice {
    Rock,
    Paper,
    Scissors,
}
#[derive(PartialEq, Eq)]
enum Outcome {
    Win,
    Loss,
    Draw,
}

fn score(c: &Choice) -> u32 {
    match c {
        Choice::Rock => 1,
        Choice::Paper => 2,
        Choice::Scissors => 3,
    }
}

fn battle(me: &Choice, elf: &Choice) -> Outcome {
    if me == elf {
        return Outcome::Draw;
    }
    if me == &Choice::Rock && elf == &Choice::Scissors
        || me == &Choice::Paper && elf == &Choice::Rock
        || me == &Choice::Scissors && elf == &Choice::Paper
    {
        return Outcome::Win;
    }
    Outcome::Loss
}
fn main() {
    let input = fs::read_to_string("input.txt").expect("Should be able to read input");
    //PART ONE
    let mut it = input.split_whitespace();
    let mut total_score = 0;
    loop {
        let elf_choice: Choice = match it.next() {
            Some("A") => Choice::Rock,
            Some("B") => Choice::Paper,
            Some("C") => Choice::Scissors,
            _ => break,
        };
        let your_choice: Choice = match it.next().expect("There should be two inputs per line") {
            "X" => Choice::Rock,
            "Y" => Choice::Paper,
            "Z" => Choice::Scissors,
            _ => break,
        };
        total_score += match battle(&your_choice, &elf_choice) {
            Outcome::Win => 6,
            Outcome::Draw => 3,
            Outcome::Loss => 0,
        };
        total_score += score(&your_choice);
    }
    println!("{total_score}");
    //PART TWO
    it = input.split_whitespace();
    let mut total_score = 0;
    loop {
        let elf_choice: Choice = match it.next() {
            Some("A") => Choice::Rock,
            Some("B") => Choice::Paper,
            Some("C") => Choice::Scissors,
            _ => break,
        };
        let round_outcome: Outcome = match it.next().expect("There should be two inputs per line") {
            "X" => Outcome::Loss,
            "Y" => Outcome::Draw,
            "Z" => Outcome::Win,
            _ => break,
        };
        let your_choice: Choice;
        if battle(&Choice::Rock, &elf_choice) == round_outcome {
            your_choice = Choice::Rock;
        } else if battle(&Choice::Paper, &elf_choice) == round_outcome {
            your_choice = Choice::Paper;
        } else {
            your_choice = Choice::Scissors;
        }
        total_score += match round_outcome {
            Outcome::Win => 6,
            Outcome::Draw => 3,
            Outcome::Loss => 0,
        };
        total_score += score(&your_choice);
    }
    println!("{total_score}");
}
