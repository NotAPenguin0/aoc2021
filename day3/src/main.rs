use std::io::*;
use std::fs::File;

fn read_input() -> Result<Vec<String>> {
    let file = File::open("input/day3.in")?;
    let mut reader = BufReader::new(file);
    let mut contents = String::new();
    reader.read_to_string(&mut contents)?;

    Ok(
        contents.split_terminator("\n")
            .map(|s| s.replace("\r", ""))
            .collect()
    )
}

fn get_digit_counts(input: &[String]) -> Vec<i32> {
    // Each element of count represents the number of '1' bits at that position.
    // The number of zero bits is then equal to input.len() - count[i]
    input.iter().fold(vec![0i32; input.len()], |count, value| {
        // zip iterators together so we can add to them element wise
        count.iter().zip(value.chars().into_iter()).map(|(&cnt, val)| {
            if val == '1' { cnt + 1 } else { cnt }
        }).collect()
    })
}

// Get digit count for a single bit
fn get_digit_count_bit(input: &[String], bit: usize) -> i32 {
    input.iter().fold(0i32, |count, val| {
        if val.as_bytes()[bit] == '1' as u8 { count + 1} else { count }
    })
}

fn part1(input: &[String]) -> i32 {
    let counts = get_digit_counts(&input);

    let gamma_str = counts.iter().map(|&c| if c > (input.len() as i32 - c) { '1' } else { '0' }).collect::<String>();
    let epsilon_str = counts.iter().map(|&c| if c > (input.len() as i32 - c) { '0' } else { '1' }).collect::<String>(); // same but with 0 and 1 swapped

    let gamma = i32::from_str_radix(gamma_str.as_str(), 2).unwrap();
    let epsilon = i32::from_str_radix(epsilon_str.as_str(), 2).unwrap();

    gamma * epsilon
}

fn part2(input: &[String]) -> i32 {
    // Now we will do two elimination processes, one for each rating type we need to compute
    let mut life_support_vec = input.to_vec();
    let mut oxygen_vec = input.to_vec();

    for bit in 0..input[0].len() {
        let lifesupp_count = get_digit_count_bit(&life_support_vec, bit);
        let oxy_count = get_digit_count_bit(&oxygen_vec, bit);

        if life_support_vec.len() != 1 {
            let max = life_support_vec.len() as i32;
            life_support_vec.retain(|v| {
                if lifesupp_count >= max - lifesupp_count { v.as_bytes()[bit] == '1' as u8 } else { v.as_bytes()[bit] == '0' as u8 }
            })
        }

        if oxygen_vec.len() != 1 {
            let max = oxygen_vec.len() as i32;
            oxygen_vec.retain(|v| {
                if oxy_count >= max - oxy_count { v.as_bytes()[bit] == '0' as u8 } else { v.as_bytes()[bit] == '1' as u8 }
            })
        }
    }

    i32::from_str_radix(life_support_vec[0].as_str(), 2).unwrap() * i32::from_str_radix(oxygen_vec[0].as_str(), 2).unwrap()
}

fn main() {
    let input = read_input().expect("Parse error");

    println!("Answer to part 1: {}", part1(&input));
    println!("Answer to part 2: {}", part2(&input));
}