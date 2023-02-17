use std::fs;
fn main() {
    let input = fs::read_to_string("input.txt").expect("Should be able to read input");
    let mut stacks: [Vec<char>; 10] = Default::default();
    let mut stacks_2: [Vec<char>; 10] = Default::default();
    let mut move_input = false;
    for line in input.lines() {
        let mut it = line.chars();
        let mut cur_stack = 0;
        if !move_input {
            while let Some(c) = it.next() {
                match c {
                    '[' => {
                        cur_stack += 1;
                        stacks[cur_stack]
                            .insert(0, it.next().expect("Expected a character after ["));
                        it.next(); //']'
                        it.next(); //' '
                    }
                    ' ' => {
                        let end = it.next().expect("Expected a character");
                        if end == '1' {
                            move_input = true;
                            break;
                        }
                        it.next();
                        it.next();
                        cur_stack += 1;
                    }
                    _ => {
                        println!("READ THE CHARACTER {c}");
                        panic!();
                    } //We don't expect anything else
                }
            }
        } else if line.is_empty() {
            stacks_2 = stacks.clone();
        } else {
            let mut cur_line = line.split_whitespace();
            cur_line.next(); //move
            let num_moves: u32 = cur_line
                .next()
                .expect("Expected corrected input")
                .parse()
                .expect("Expected a number");
            cur_line.next(); //from
            let src: usize = cur_line
                .next()
                .expect("Expected a source stack")
                .parse()
                .expect("Expected a number for source stack");
            cur_line.next(); //to
            let dst: usize = cur_line
                .next()
                .expect("Expected a destination stack")
                .parse()
                .expect("Expected a number for destination stack");
            for _ in 0..num_moves {
                let temp = stacks[src]
                    .pop()
                    .expect("Expected stacks to be of adequate size");
                stacks[dst].push(temp);
            }
            let mut temp: Vec<char> = Vec::new();
            for _ in 0..num_moves {
                temp.push(
                    stacks_2[src]
                        .pop()
                        .expect("Expected stacks to be of adequate size"),
                );
            }
            temp.reverse();
            stacks_2[dst].append(&mut temp);
        }
    }
    println!("PART ONE: ");
    for vec in stacks {
        print!("{:?}", vec.last().unwrap_or(&' '));
    }
    println!();
    println!("PART TWO: ");
    for vec in stacks_2 {
        print!("{:?}", vec.last().unwrap_or(&' '));
    }
    println!();
}
