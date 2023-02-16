use std::collections::BTreeSet;
use std::fs;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Elf {
    cals: u32,
    id: u32,
}

fn main() {
    //PART ONE
    let input = fs::read_to_string("input.txt").expect("Should be able to read input");
    let mut best: (u32, u32) = (1, 0);
    let mut cur: (u32, u32) = (1, 0);
    let mut count = 1;
    for word in input.lines() {
        if word.is_empty() {
            count += 1;
            cur = (count, 0);
        } else {
            let cal: u32 = word.parse().expect("Expected number");
            cur = (cur.0, cur.1 + cal);
            if best.1 < cur.1 {
                best = cur;
            }
        }
    }
    println!("Best choice is: {:?}", best);

    //PART TWO
    let mut rankings: BTreeSet<Elf> = BTreeSet::new();
    let mut count = 1;
    let mut cur = Elf { id: 1, cals: 0 };
    for word in input.lines() {
        if word.is_empty() {
            count += 1;
            rankings.insert(cur);
            cur = Elf { id: count, cals: 0 };
        } else {
            let cal: u32 = word.parse().expect("Expected number");
            cur.cals += cal;
        }
    }
    let mut sum = 0;
    for _ in 0..3 {
        sum += rankings
            .pop_last()
            .expect("Enough Elves should be in the input")
            .cals;
    }
    println!("{sum}");
}
