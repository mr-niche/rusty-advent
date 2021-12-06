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

    // How many days will be calculate for?
    let max_day = 80;

    // Pre-calculate future_fish(8, days) values
    let mut fish_days: Vec<isize> = Vec::new();
    for day in 1..max_day + 1 {
        let calc = (day - 2) / 7;
        if day < 10 {
            fish_days.push(0);
        } else {
            fish_days.push(calc);
        }
    }

    for (i, f) in fish_days.iter().enumerate() {
        println!("future_fish(8, {}) = {}", i + 1, f);
    }

    // Start our total with our inital # of fish
    //let mut total = school_of_fish.len();
    //println!("Start with {} fish...", total);
    //println!();
    //for fish in school_of_fish {
    //    println!("Init: {}", fish);
    //    let this_fish = future_fish(fish, 80);
    //    total = total + this_fish;
    //    println!("fish produced {}", this_fish);
    //    println!();
    //}

    //println!("...end with: {} !", total);

    Ok(())
}

fn future_fish(mut age: usize, mut days: usize) -> usize {
    let mut acc = 0;

    let init_num = (days + (6 - age)) / 7;

    println!(
        "future_fish({}, {}) will produce {} fish",
        age, days, init_num
    );
    acc = init_num;

    // Calculate the rest
    for i in 0..init_num {
        let mut remaining = days - (age + 1) - (7 * i);
        if remaining > 7 {
            let mut v = 0;
            let mut d = remaining;
            while d > 8 {
                v = v + (d - 3) / 7;
                d = d - 7;
            }
            acc = acc + v;
            println!("future_fish(8, {}) = {}", remaining, v);
        }
    }
    acc
}
