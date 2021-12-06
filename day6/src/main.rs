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

    let mut school_of_fish: Vec<usize> = Vec::new();
    for age in example {
        school_of_fish.push(age);
    }

    // Print initial state
    print!("Initial state: ");
    for fish in &school_of_fish {
        print!("{},", fish);
    }
    println!();

    // Start our total with our inital # of fish
    let mut total = school_of_fish.len();
    for fish in school_of_fish {
        total = total + future_fish(fish, 80);
    }

    println!("Total: {}", total);

    Ok(())
}

fn future_fish(init: usize, days: usize) -> usize {
    let mut acc = 0;
    let mut clock = init;
    for i in 1..days + 1 {
        match clock {
            0 => {
                acc = acc + 1;
                clock = 6;
                acc = acc + future_fish(8, days - i);
            }
            _ => clock = clock - 1,
        }
    }
    acc
}
