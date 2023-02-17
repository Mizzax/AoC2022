use std::fs;
fn main() {
    let input = fs::read_to_string("input.txt").expect("Should be able to read input");
    let mut count = 0;
    let mut count_2 = 0;
    for line in input.lines() {
        let mut line = line.split_terminator(',');
        let (first, second) = (
            line.next().expect("Expect valid input"),
            line.next().expect("Expect valid input"),
        );
        let mut first = first.split('-');
        let mut second = second.split('-');
        let first: (u32, u32) = (
            first.next().expect("Expect correct input").parse().unwrap(),
            first.next().expect("Expect correct input").parse().unwrap(),
        );
        let second: (u32, u32) = (
            second
                .next()
                .expect("Expect correct input")
                .parse()
                .unwrap(),
            second
                .next()
                .expect("Expect correct input")
                .parse()
                .unwrap(),
        );
        if contains(&first, &second) {
            count += 1;
        }
        if overlap(&first, &second) {
            count_2 += 1;
        }
    }
    println!("{count}");
    println!("{count_2}");
}

fn contains(first: &(u32, u32), second: &(u32, u32)) -> bool {
    if first.0 <= second.0 && first.1 >= second.1 {
        return true;
    }
    if second.0 <= first.0 && second.1 >= first.1 {
        return true;
    }
    false
}
fn overlap(first: &(u32, u32), second: &(u32, u32)) -> bool {
    if contains(first, second) {
        return true;
    }
    if (first.1 >= second.0) && (first.0 <= second.0)
        || (second.1 >= first.0) && (second.0 <= first.0)
    {
        return true;
    }
    false
}
