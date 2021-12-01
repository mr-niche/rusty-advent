// Author: Nicholas LaJoie
// Advent of Code, Day 1

fn main() -> anyhow::Result<()> {
    let p1_values = include_str!("input") // our input
        .trim_end() // get rid of the trailing newline!
        .split('\n') // split on newlines
        .map(str::parse::<i64>) // convert strings to integers
        .collect::<Result<Vec<i64>, _>>()?; // collect everything into a vector

    let p2_values = p1_values.clone(); // create a copy of our input for part 2

    dbg!(part1(p1_values).unwrap());
    dbg!(part2(p2_values).unwrap());

    Ok(())
}

fn part1(v: Vec<i64>) -> Option<i64> {
    let mut answer = 0;
    for a in v.windows(2) {
        if a[1] > a[0] {
            answer = answer + 1;
        }
    }
    Some(answer)
}

fn part2(v: Vec<i64>) -> Option<i64> {
    let mut windows = Vec::<i64>::new();
    for a in v.windows(3) {
        windows.push(a.iter().sum());
    }
    Some(part1(windows).unwrap())
}
