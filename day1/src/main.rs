use std::io::*;
use std::fs::File;
use std::time::Instant;

fn read_input() -> Result<Vec<u32>> {
    let file = File::open("input/day1.in")?;
    let mut reader = BufReader::new(file);
    let mut contents = String::new();
    reader.read_to_string(&mut contents)?;

    // Note to self: as far as I understand this, the type of parse() can be inferred from the return type of read_input().
    Ok(contents.split_whitespace()
        .map(|s| s.parse().expect("parse error"))
        .collect())
}

fn part1(input: &[u32]) -> u32 { // Use slice as parameter instead of reference to Vec
    let mut prev = u32::MAX;
    input.iter().fold(0, |counter, &value| {
        let counter = if value > prev { counter + 1 } else { counter };
        prev = value;
        counter
    })
}

fn part2(input: &[u32]) -> u32 {
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

    let mut start = Instant::now();
    println!("Answer to part 1: {} (took {} µs)", part1(&input), start.elapsed().as_micros());
    start = Instant::now();
    println!("Answer to part 2: {} (took {} µs)", part2(&input), start.elapsed().as_micros());
}
