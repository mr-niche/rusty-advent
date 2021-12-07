// Author: Nicholas LaJoie
// Advent of Code, Day 7

fn main() -> anyhow::Result<()> {
    let example: Vec<isize> = include_str!("example")
        .trim_end()
        .split(",")
        .map(str::parse::<isize>)
        .map(Result::unwrap)
        .collect();

    let input: Vec<isize> = include_str!("input")
        .trim_end()
        .split(",")
        .map(str::parse::<isize>)
        .map(Result::unwrap)
        .collect();

    //dbg!(input);

    // Get max horizontal position
    let min_val = input.iter().min().unwrap();
    let max_val = input.iter().max().unwrap();

    dbg!(min_val);
    dbg!(max_val);

    // Calculate all fuel usage
    let mut min_usage = max_val * max_val;
    for v in *min_val..*max_val {
        let fuel = fuel_usage(&input, v);
        if fuel < min_usage {
            min_usage = fuel;
        }
        //println!("Fuel to {} = {}", v, fuel);
    }
    println!("Min_usage: {}", min_usage);

    Ok(())
}

fn fuel_usage(vals: &Vec<isize>, pos: isize) -> isize {
    let mut acc = 0;
    for v in vals {
        acc = acc + (pos - v).abs();
        //println!("    acc: {}", acc);
    }
    acc
}
