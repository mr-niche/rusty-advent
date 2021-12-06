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
    for age in input {
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
    println!("Start with {} fish...", total);
    println!();
    for fish in school_of_fish {
        println!("Init: {}", fish);
        let this_fish = future_fish(fish, 80 + 1);
        total = total + this_fish;
        println!("fish produced {}", this_fish);
        println!();
    }

    println!("...end with: {} !", total);

    Ok(())
}

fn future_fish(mut age: usize, mut days: usize) -> usize {
    let mut acc = 0;

    //println!("future_fish({}, {})", age, days);
    if days >= age {
        let mut num = 0;

        if age == 8 {
            num = (days - 3) / 7;
        } else {
            num = (days + (5 - age)) / 7;
        }

        acc = acc + num;

        // But how many will _those_ fish produce??
        for i in 0..num {
            let remaining = days - (age + 1) - (7 * i);
            if remaining > 0 {
                acc = acc + future_fish(8, remaining);
            }
        }
    }
    acc
}
