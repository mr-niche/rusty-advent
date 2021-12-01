// Author: Nicholas LaJoie
// Advent of Code, Day 1

fn main() -> anyhow::Result<()> {
    let answer = increases(
        include_str!("input")
            .trim_end() // this guy wasted 20 minutes...
            .split('\n')
            .map(str::parse::<i64>)
            .collect::<Result<Vec<_>, _>>()?,
    );

    Ok(())
}

fn increases(s: Vec<i64>) -> Option<i64> {
    todo!()
}
