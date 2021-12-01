use std::io::*;
use std::fs::File;
use std::io::BufReader;

fn read_input() -> Result<Vec<u32>> {
    let file = File::open("input/in.txt")?;
    let mut reader = BufReader::new(file);
    let mut contents = String::new();
    reader.read_to_string(&mut contents)?;

    // Note to self: as far as I understand this, the type of parse() can be inferred from the return type of read_input().
    Ok(contents.split_whitespace()
        .map(|s| s.parse().expect("parse error"))
        .collect())
}

// TODO: Maybe find a more clean way to write this function than a raw loop?
fn part1(input: &Vec<u32>) -> u32 {
    let mut prev = u32::MAX;
    let mut count = 0; // Is this type deduced from the return type? If so, cool.
    for num in input {
        if num > &prev {
            count += 1
        }
        prev = *num;
    }

    count
}

fn part2(input: &Vec<u32>) -> u32 {
    let mut count = 0;
    let mut prev = u32::MAX;

    let mut i: usize = 0;
    while i + 2 < input.len() {
        let sum = input[i] + input[i + 1] + input[i + 2];
        if sum > prev {
            count += 1;
        }
        prev = sum;

        i += 1;
    }

    count
}

fn main() {
    let input = read_input().expect("Parse error");

    println!("Answer to part 1: {}", part1(&input));
    println!("Answer to part 2: {}", part2(&input));
}
