// Author: Nicholas LaJoie
// Advent of Code, Day 3

use itertools::Itertools;

fn main() -> anyhow::Result<()> {
    let keep = ['1', '0'].iter();
    let mut example = include_str!("input")
        .trim_end()
        .chars()
        .collect::<Vec<char>>();

    let mut bit_stream: Vec<u64> = Vec::new();
    for e in example {
        if e == '1' {
            bit_stream.push(1);
        } else if e == '0' {
            bit_stream.push(0);
        }
    }

    dbg!(p1_calc(bit_stream, 12));

    Ok(())
}

fn p1_calc(values: Vec<u64>, size: usize) -> Option<u64> {
    // gamma rate = most common bit across all bits
    // epsilon rate = least common bit across all bits

    // How many total values in a single column exist?
    let threshold: u64 = ((values.len() / size) / 2).try_into().unwrap();

    let mut gamma: Vec<u64> = Vec::new();
    let mut epsilon: Vec<u64> = Vec::new();
    for group in 0..size {
        let ones: u64 = values.iter().skip(group).step_by(size).sum();
        if ones > threshold {
            gamma.push(1);
            epsilon.push(0);
        } else {
            gamma.push(0);
            epsilon.push(1);
        }
    }

    let mut gamma_dec = 0;
    let mut epsilon_dec = 0;
    let base: u64 = 2;
    let mut pos: u64 = (size - 1).try_into().unwrap();
    for i in gamma {
        gamma_dec = gamma_dec + (i * base.pow(pos.try_into().unwrap()));
        if pos > 0 {
            pos = pos - 1;
        }
    }
    pos = (size - 1).try_into().unwrap();
    for i in epsilon {
        epsilon_dec = epsilon_dec + (i * base.pow(pos.try_into().unwrap()));
        if pos > 0 {
            pos = pos - 1;
        }
    }
    dbg!(gamma_dec);
    dbg!(epsilon_dec);
    Some(gamma_dec * epsilon_dec)
}
