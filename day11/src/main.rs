// Author: Nicholas LaJoie
// Advent of Code, Day 11

use colored::*;

#[derive(Debug, Clone, PartialEq, Default)]
struct Consortium {
    octopuses: Vec<Octopus>,
    width: i32,
    height: i32,
}

impl Consortium {
    fn print(&self) {
        for row in 0..self.height {
            for col in 0..self.width {
                let p: Point = (row.try_into().unwrap(), col.try_into().unwrap()).into();
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
    fn get_flash_count(&self) -> u32 {
        let mut acc = 0;
        for octopus in &self.octopuses {
            acc = acc + octopus.flash_count;
        }
        acc
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

impl Point {
    fn get_adjacent_points(&self, width: i32, height: i32) -> Vec<Point> {
        let row: i32 = self.row.try_into().unwrap();
        let col: i32 = self.col.try_into().unwrap();
        // Start by going right, go clockwise
        let potential_points: Vec<(i32, i32)> = vec![
            (row, col + 1),
            (row + 1, col + 1),
            (row + 1, col),
            (row + 1, col - 1),
            (row, col - 1),
            (row - 1, col - 1),
            (row - 1, col),
            (row - 1, col + 1),
        ];
        let mut points: Vec<Point> = Vec::new();

        for point in potential_points {
            if point.0 >= 0 && point.1 >= 0 && point.0 < height && point.1 < width {
                points.push(Point {
                    row: point.0.try_into().unwrap(),
                    col: point.1.try_into().unwrap(),
                });
            }
        }
        points
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
            println!("Flash count: {}", consortium.get_flash_count());
            println!();
        }
    }

    Ok(())
}

#[test]
fn test_get_adjacent_points() {
    let width = 10;
    let height = 10;

    // Corner case (ha!) - Upper left
    let mut point = Point { row: 0, col: 0 };
    let mut points = point.get_adjacent_points(width, height);
    let mut expected: Vec<Point> = vec![(0, 1).into(), (1, 1).into(), (1, 0).into()];
    assert_eq!(expected, points);

    // Middle case
    point = Point { row: 3, col: 5 };
    points = point.get_adjacent_points(width, height);
    expected = vec![
        (3, 6).into(),
        (4, 6).into(),
        (4, 5).into(),
        (4, 4).into(),
        (3, 4).into(),
        (2, 4).into(),
        (2, 5).into(),
        (2, 6).into(),
    ];
    assert_eq!(expected, points);

    // Corner case (ha!) - Lower right
    point = Point { row: 9, col: 9 };
    points = point.get_adjacent_points(width, height);
    expected = vec![(9, 8).into(), (8, 8).into(), (8, 9).into()];
    assert_eq!(expected, points);
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
