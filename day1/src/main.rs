// Author: Nicholas LaJoie
// Advent of Code, Day 1

use itertools::Itertools;

fn main() -> anyhow::Result<()> {
    let values = include_str!("input")
        .trim_end() // this guy wasted 20 minutes...
        .split('\n')
        .map(str::parse::<i64>)
        .collect::<Result<Vec<_>, _>>()?;

    //dbg!(part1(values).unwrap());
    dbg!(part2(values).unwrap());

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
    let mut answer = 0;
    let mut windows = Vec::<i64>::new();
    for a in v.windows(3) {
        windows.push(a.iter().sum());
    }
    Some(part1(windows).unwrap())
}
