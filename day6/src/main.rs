// Author: Nicholas LaJoie
// Advent of Code, Day 6

fn main() -> anyhow::Result<()> {
    let example: Vec<usize> = include_str!("example")
        .trim_end()
        .split(",")
        .map(str::parse::<usize>)
        .map(Result::unwrap)
        .collect();

    let input: Vec<usize> = include_str!("input")
        .trim_end()
        .split(",")
        .map(str::parse::<usize>)
        .map(Result::unwrap)
        .collect();

    let days = 18;
    let mut school_of_fish: Vec<usize> = Vec::new();
    for age in example {
        school_of_fish.push(age);
        let result = future_fish(age, days);
        println!("future_fish({}, {}) = {}", age, days, result);
    }

    Ok(())
}

fn future_fish(age: usize, days: usize) -> usize {
    let mut acc = 0;

    // How many initial children?
    acc = (days + (6 - age)) / 7;

    // How many would you expect from those children?
    acc
}
