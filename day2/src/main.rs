use std::io::*;
use std::fs::File;

struct SubmarineLocation {
    horizontal: i32,
    depth: i32,
    aim: i32
}

#[derive(Debug)]
enum Command {
    Forward,
    Down,
    Up,
    Unknown
}

struct Instruction {
    command: Command,
    modifier: i32
}

fn read_input() -> Result<Vec<Instruction>> {
    let file = File::open("input/day2.in")?;
    let mut reader = BufReader::new(file);
    let mut contents = String::new();
    reader.read_to_string(&mut contents)?;

    Ok(
        contents.split_terminator("\n")
            .map(|line| {
                let split: Vec<&str> = line.split_whitespace().collect();
                Instruction {
                    command: match split[0] {
                        "forward" => Command::Forward,
                        "down" => Command::Down,
                        "up" => Command::Up,
                        _ => Command::Unknown
                    },
                    modifier: split[1].parse().expect("parse error")
                }
            })
            .collect()
    )
}

fn follow_instructions(instructions: &[Instruction], start: SubmarineLocation) -> SubmarineLocation {
    instructions.iter().fold(start, |location, instruction| {
        match instruction.command {
            Command::Forward => SubmarineLocation{horizontal: location.horizontal + instruction.modifier, ..location},
            Command::Down => SubmarineLocation{depth: location.depth + instruction.modifier, ..location},
            Command::Up => SubmarineLocation{depth: location.depth - instruction.modifier, ..location},
            Command::Unknown => location
        }
    })
}

fn follow_instructions_aim(instructions: &[Instruction], start: SubmarineLocation) -> SubmarineLocation {
    instructions.iter().fold(start, |location, instruction| {
        match instruction.command {
            Command::Forward => SubmarineLocation{horizontal: location.horizontal + instruction.modifier, depth: location.depth + location.aim * instruction.modifier, ..location},
            Command::Down => SubmarineLocation{aim: location.aim + instruction.modifier, ..location},
            Command::Up => SubmarineLocation{aim: location.aim - instruction.modifier, ..location},
            Command::Unknown => location
        }
    })
}

fn part1(input: &[Instruction]) -> i32 {
    let end = follow_instructions(input, SubmarineLocation{horizontal: 0, depth: 0, aim: 0 });
    end.horizontal * end.depth
}

fn part2(input: &[Instruction]) -> i32 {
    let end = follow_instructions_aim(input, SubmarineLocation{horizontal: 0, depth: 0, aim: 0 });
    end.horizontal * end.depth
}

fn main() {
    let input = read_input().expect("parse error");

    println!("Answer to part 1: {}", part1(&input));
    println!("Answer to part 2: {}", part2(&input));
}
