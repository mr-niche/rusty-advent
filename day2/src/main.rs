// Author: Nicholas LaJoie
// Advent of Code, Day 2

use itertools::Itertools;

fn main() -> anyhow::Result<()> {
    let p1_values = include_str!("input").trim_end().split('\n');

    let mut hp = 0;
    let mut dep = 0;
    let mut aim = 0;

    for p in p1_values {
        let mut a = p.split_whitespace().tuples::<(_, _)>();
        let (dir, val) = a.next().unwrap();
        if dir == "forward" {
            hp = hp + val.parse::<i64>().unwrap();
            dep = dep + (aim * val.parse::<i64>().unwrap());
        } else if dir == "up" {
            //dep = dep - val.parse::<i64>().unwrap();
            aim = aim - val.parse::<i64>().unwrap();
        } else if dir == "down" {
            //dep = dep + val.parse::<i64>().unwrap();
            aim = aim + val.parse::<i64>().unwrap();
        }
    }

    println!("hp: {}, dp: {}, result: {}", hp, dep, hp * dep);

    Ok(())
}
