// Author: Nicholas LaJoie
// Advent of Code, Day 11

use colored::*;

#[derive(Debug, Clone, PartialEq, Default)]
struct Consortium {
    octopuses: Vec<Octopus>,
    width: usize,
    height: usize,
}

impl Consortium {
    fn print(&self) {
        for row in 0..self.height {
            for col in 0..self.width {
                let p: Point = (row, col).into();
                let e = self.get_octopus(&p).unwrap().energy_level;
                match e {
                    0 => print!("{}", "0".bold()),
                    _ => print!("{}", e.to_string().dimmed()),
                }
            }
            println!();
        }
    }
    fn get_octopus(&self, location: &Point) -> Option<&Octopus> {
        for octopus in &self.octopuses {
            if *location == octopus.location {
                return Some(octopus);
            }
        }
        None
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
struct Octopus {
    location: Point,
    energy_level: u32,
    flash_count: u32,
    flashed: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
struct Point {
    row: usize,
    col: usize,
}

impl From<(usize, usize)> for Point {
    fn from((row, col): (usize, usize)) -> Self {
        Self { row, col }
    }
}

fn main() -> anyhow::Result<()> {
    // Parse input
    let example: Vec<&str> = include_str!("example").trim_end().split('\n').collect();
    let mut consortium: Consortium = Consortium {
        octopuses: Vec::new(),
        width: 10,
        height: 10,
    };

    // Build a Consortium!
    for (row, nums) in example.iter().enumerate() {
        // Break the row into digits
        let digits: Vec<u32> = nums.chars().map(|d| d.to_digit(10).unwrap()).collect();

        // Insert each octopus into the consortium
        for (col, energy) in digits.iter().enumerate() {
            consortium.octopuses.push(Octopus {
                location: (row, col).into(),
                energy_level: *energy,
                flash_count: 0,
                flashed: false,
            });
        }
    }

    // Now let's process the steps
    let steps = 100;
    for step in 0..steps + 1 {
        if step == 0 {
            println!("Before any steps:");
            consortium.print()
        } else {
            //println!("After step {}:", step);
            // consortium.print()
            continue;
        }
    }

    Ok(())
}
