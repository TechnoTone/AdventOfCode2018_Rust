use std::collections::HashSet;
use std::io::{self, Read, Write};

type Result<T> = ::std::result::Result<T, Box<::std::error::Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1a(&input)?;
    // part2(&input)?;
    Ok(())
}

// fn part1(input: &str) -> Result<()> {
//     let mut twos = 0;
//     let mut threes = 0;

//     for line in input.lines() {
//         //writeln!(io::stdout(), "{}", line)?;

//         let mut prev: u8 = 0;
//         let mut count: u8 = 0;
//         let mut hasTwo = false;
//         let mut hasThree = false;

//         let mut lineBytes = line.as_bytes();
//         lineBytes.sort();
//         for c in lineBytes.iter() {
//             if c == &prev {
//                 count += 1;
//             } else {
//                 if count == 2u8 {
//                     writeln!(io::stdout(), "{}:{}", prev, count);
//                     hasTwo = true;
//                 }
//                 if count == 3u8 {
//                     writeln!(io::stdout(), "{}:{}", prev, count);
//                     hasThree = true;
//                 }
//                 prev = *c;
//                 count = 1;
//             }
//             if hasTwo {
//                 twos += 1;
//             }
//             if hasThree {
//                 threes += 1;
//             }
//         }
//     }

//     writeln!(io::stdout(), "Twos: {}", twos);
//     writeln!(io::stdout(), "Threes: {}", threes);
//     writeln!(io::stdout(), "Part1: {}", twos * threes);

//     Ok(())
// }

fn part1a(input: &str) -> Result<()> {
    let mut frequencies = [0u8; 256];
    let (mut twos, mut threes) = (0, 0);
    for line in input.lines() {
        for f in frequencies.iter_mut() {
            *f = 0;
        }
        for b in line.as_bytes().iter().map(|&b| b as usize) {
            frequencies[b] = frequencies[b].saturating_add(1);
        }
        if frequencies.iter().any(|&f| f == 2) {
            twos += 1;
        }
        if frequencies.iter().any(|&f| f == 3) {
            threes += 1;
        }
    }
    writeln!(io::stdout(), "{}", twos * threes)?;
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
