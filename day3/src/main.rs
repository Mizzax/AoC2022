use std::fs;
fn main() {
    let input = fs::read_to_string("input.txt").expect("Should be able to read input");
    //PART ONE
    let iter = input.lines();
    let mut vector: Vec<char> = Vec::new();
    vector.push('0');
    for c in 'a'..='z' {
        vector.push(c);
    }
    for c in 'A'..='Z' {
        vector.push(c);
    }
    let mut sum = 0;
    for line in iter {
        let (first, second) = line.split_at(line.len() / 2);
        'outer: for f_char in first.chars() {
            for s_char in second.chars() {
                if f_char == s_char {
                    sum += vector
                        .iter()
                        .position(|&c| c == f_char)
                        .expect("Input should only be characters");
                    break 'outer;
                }
            }
        }
    }
    println!("{sum}");
    //PART TWO
    let mut iter = input.lines();
    sum = 0;
    while let (Some(first), Some(second), Some(third)) = (iter.next(), iter.next(), iter.next()) {
        'first_loop: for f_char in first.chars() {
            for s_char in second.chars() {
                if f_char == s_char {
                    for t_char in third.chars() {
                        if t_char == s_char {
                            sum += vector
                                .iter()
                                .position(|&c| c == f_char)
                                .expect("Input should only be characters");
                            break 'first_loop;
                        }
                    }
                    continue 'first_loop;
                }
            }
        }
    }
    println!("{sum}");
}
