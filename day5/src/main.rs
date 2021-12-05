// Author: Nicholas LaJoie
// Advent of Code, Day 5

#[derive(PartialEq, Debug, Default)]
struct Vent {
    a: Point,
    b: Point,
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
struct Point {
    x: usize,
    y: usize,
}

impl From<(usize, usize)> for Point {
    fn from((x, y): (usize, usize)) -> Self {
        Self { x, y }
    }
}

fn main() -> anyhow::Result<()> {
    let example: Vec<&str> = include_str!("example").trim_end().split('\n').collect();
    let _input: Vec<&str> = include_str!("input").trim_end().split('\n').collect();

    let mut vents: Vec<Vent> = Vec::new();
    for line in example {
        let vent = line.split(" -> ").collect::<Vec<&str>>();
        // Get Point a
        let a = vent[0]
            .split(",")
            .map(str::parse::<usize>)
            .map(Result::unwrap)
            .collect::<Vec<usize>>();

        // Get Point b
        let b = vent[1]
            .split(",")
            .map(str::parse::<usize>)
            .map(Result::unwrap)
            .collect::<Vec<usize>>();

        // Create a Vent!
        vents.push(Vent {
            a: Point { x: a[0], y: a[1] },
            b: Point { x: b[0], y: b[1] },
        });
    }
    dbg!(vents);

    Ok(())
}

#[test]
fn test_vent() {}
