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
    fn clear_all(&mut self) {
        for octopus in &mut self.octopuses {
            octopus.clear();
        }
    }
    fn step_all(&mut self) {
        for octopus in &mut self.octopuses {
            octopus.step();
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
struct Octopus {
    location: Point,
    energy_level: u32,
    flash_count: u32,
    flashed: bool,
}

impl Octopus {
    fn clear(&mut self) {
        self.flashed = false;
    }
    fn step(&mut self) {
        match self.energy_level {
            // TODO: Will need to validate it hasn't flashed already
            9 => {
                self.energy_level = 0;
                self.flash_count = self.flash_count + 1;
                self.flashed = true;
            }
            _ => self.energy_level = self.energy_level + 1,
        }
    }
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
    //let steps = 100;
    let steps = 5;
    for step in 0..steps + 1 {
        if step == 0 {
            println!("Before any steps:");
            consortium.print();
            println!();
        } else {
            // 1. Update all octopus energies
            consortium.step_all();

            // 2. Do the rippling flashes
            // 3. Clear the flashes
            consortium.clear_all();

            // 4. Print
            println!("After step {}:", step);
            consortium.print();
            println!();
        }
    }

    Ok(())
}

#[test]
fn test_octopus_clear() {
    let mut octopus = Octopus {
        location: Point { row: 0, col: 0 },
        energy_level: 1,
        flash_count: 0,
        flashed: true,
    };
    assert_eq!(octopus.flashed, true);
    octopus.clear();
    assert_eq!(octopus.flashed, false);
}

#[test]
fn test_consortium_clear_all() {
    let mut consortium: Consortium = Consortium {
        octopuses: Vec::new(),
        width: 10,
        height: 10,
    };
    consortium.octopuses.push(Octopus {
        location: Point { row: 0, col: 0 },
        energy_level: 1,
        flash_count: 0,
        flashed: true,
    });
    consortium.octopuses.push(Octopus {
        location: Point { row: 0, col: 1 },
        energy_level: 2,
        flash_count: 0,
        flashed: false,
    });
    consortium.octopuses.push(Octopus {
        location: Point { row: 0, col: 2 },
        energy_level: 3,
        flash_count: 0,
        flashed: true,
    });

    assert_eq!(
        consortium
            .get_octopus(&Point { row: 0, col: 0 })
            .unwrap()
            .flashed,
        true
    );
    assert_eq!(
        consortium
            .get_octopus(&Point { row: 0, col: 1 })
            .unwrap()
            .flashed,
        false
    );
    assert_eq!(
        consortium
            .get_octopus(&Point { row: 0, col: 2 })
            .unwrap()
            .flashed,
        true
    );

    // Clear em
    consortium.clear_all();

    assert_eq!(
        consortium
            .get_octopus(&Point { row: 0, col: 0 })
            .unwrap()
            .flashed,
        false
    );
    assert_eq!(
        consortium
            .get_octopus(&Point { row: 0, col: 1 })
            .unwrap()
            .flashed,
        false
    );
    assert_eq!(
        consortium
            .get_octopus(&Point { row: 0, col: 2 })
            .unwrap()
            .flashed,
        false
    );
}
