use std::time::Instant;

fn silver(input: &Vec<char>) -> i64 {
    input
        .iter()
        .map(|x| {
            match x {
                '(' => 1,
                ')' => -1,
                _ => unreachable!(),
            }
        })
        .sum()
}

fn gold(input: &Vec<char>) -> usize {
    let mut position = 0;

    for i in 0..input.len() {
        if input[i] == '(' {
            position += 1;
        } else {
            position -= 1;
        }

        if position == -1 {
            return i + 1;
        }
    }

    unreachable!()
}

fn main() {
    let now = Instant::now();

    let input: Vec<char> = include_str!("input").trim().chars().collect();

    println!("Silver: {}", silver(&input));
    println!("Gold: {}", gold(&input));

    println!("Time: {}ms", now.elapsed().as_millis());
}
