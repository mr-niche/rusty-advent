// Author: Nicholas LaJoie
// Advent of Code, Day 1

use itertools::Itertools;

fn main() -> anyhow::Result<()> {
    let values = include_str!("input")
        .trim_end() // this guy wasted 20 minutes...
        .split('\n')
        .map(str::parse::<i64>)
        .collect::<Result<Vec<_>, _>>()?;

    let mut answer = 0;
    for a in values.windows(2) {
        if a[1] > a[0] {
            answer = answer + 1;
        }
    }

    dbg!(answer);

    Ok(())
}
