use std::collections::HashSet;
use std::io::{self, Read, Write};

type Result<T> = ::std::result::Result<T, Box<::std::error::Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let inputs = read_inputs(&input);

    part1(&inputs)?;
    part2(&inputs)?;
    Ok(())
}

fn read_inputs(input: &str) -> Vec<i32> {
    let mut vector: Vec<i32> = vec![];

    for line in input.lines() {
        let i: i32 = line.parse().unwrap();
        vector.push(i);
    }

    return vector;
}

fn part1(inputs: &Vec<i32>) -> Result<()> {
    let mut freq = 0;
    for change in inputs {
        freq += change;
    }
    writeln!(io::stdout(), "{}", freq)?;
    Ok(())
}

fn part2(inputs: &Vec<i32>) -> Result<()> {
    let mut freq = 0;
    let mut seen = HashSet::new();
    seen.insert(0);

    loop {
        for change in inputs {
            freq += change;
            if seen.contains(&freq) {
                writeln!(io::stdout(), "{}", freq)?;
                return Ok(());
            }
            seen.insert(freq);
        }
    }
}
