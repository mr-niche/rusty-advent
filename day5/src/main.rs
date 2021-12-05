// Author: Nicholas LaJoie
// Advent of Code, Day 5

#[derive(PartialEq, Debug, Default)]
struct Vent {
    a: Point,
    b: Point,
}

impl Vent {
    fn is_flat(&self) -> bool {
        if self.a.x == self.b.x || self.a.y == self.b.y {
            return true;
        }
        false
    }
    fn get_points(&self) -> Vec<Point> {
        // Get the points contained between two points
        let rise = self.b.y - self.a.y;
        let run = self.b.x - self.a.x;
        let mut slope = 0;
        if rise != 0 {
            // Inverse our rise because it goes down
            slope = run / rise;
        }

        let mut points: Vec<Point> = Vec::new();

        // Case 1: We have a vertical line, where our equation becomes 'x = ?'
        // Case 2: We have a horizontal line, where our equation becomes 'y = ?'
        // Case 3: We have a diagonal of some sort, where our equation becomes 'y = mx + b'

        // Case 1
        if run == 0 {
            let x = self.a.x;
            if rise > 0 {
                for y in self.a.y..self.b.y + 1 {
                    points.push((x, y).into());
                }
            } else {
                for y in self.b.y..self.a.y + 1 {
                    points.push((x, y).into());
                }
            }
            return points;
        }
        // Case 2
        if rise == 0 {
            let y = self.a.y;
            if run > 0 {
                for x in self.a.x..self.b.x + 1 {
                    points.push((x, y).into());
                }
            } else {
                for x in self.b.x..self.a.x + 1 {
                    points.push((x, y).into());
                }
            }
            return points;
        }
        // Case 3 (we only care about slope of 1)
        // Example: 0,0 -> 1,1 -> 2,2 ... 8,8
        // Example: 6,4 -> 5,3 -> 4,2 -> 3,1 -> 2,0
        if slope == 1 {
            let mut y = self.a.y;
            // Increment x and y
            if self.a.x < self.b.x {
                for x in self.a.x..self.b.x + 1 {
                    points.push((x, y).into());
                    y = y + 1;
                }
            }
            // Decrement x and y
            else {
                y = self.b.y;
                for x in self.b.x..self.a.x + 1 {
                    points.push((x, y).into());
                    y = y + 1;
                }
            }
            return points;
        }
        // Example: 5,5 -> 6,4 -> 7,3 -> 8,2
        // Example: 8,0 -> 7,1 -> 6,2 ... 0,8
        if slope == -1 {
            let mut y = self.a.y;
            // Increment x and decrement y
            if self.a.x < self.b.x {
                for x in self.a.x..self.b.x + 1 {
                    points.push((x, y).into());
                    y = y - 1;
                }
            }
            // Decrement x and increment y
            else {
                y = self.b.y;
                for x in self.b.x..self.a.x + 1 {
                    points.push((x, y).into());
                    y = y - 1;
                }
            }
            return points;
        }
        points
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
struct Point {
    x: isize,
    y: isize,
}

impl From<(isize, isize)> for Point {
    fn from((x, y): (isize, isize)) -> Self {
        Self { x, y }
    }
}

#[derive(PartialEq, Debug, Default)]
struct OceanField {
    origin: Point,
    extent: Point,
    field: Vec<usize>,
}

impl OceanField {
    fn print(&self) {
        let width = self.extent.x + 1;
        let height = self.extent.y + 1;
        for y in 0..height {
            for x in 0..width {
                let index: usize = ((y * width) + x).try_into().unwrap();
                let val = self.field[index];
                if val == 0 {
                    print!(".");
                } else {
                    print!("{}", val);
                }
            }
            println!();
        }
    }
    fn update_field(&mut self, index: usize) {
        let cur_val = self.field.get(index);
        self.field[index] = cur_val.unwrap() + 1;
    }
}

fn main() -> anyhow::Result<()> {
    let example: Vec<&str> = include_str!("example").trim_end().split('\n').collect();
    let input: Vec<&str> = include_str!("input").trim_end().split('\n').collect();

    let mut vents: Vec<Vent> = Vec::new();
    let mut big_x: isize = 0;
    let mut big_y: isize = 0;
    for line in example {
        let vent = line.split(" -> ").collect::<Vec<&str>>();
        // Get Point a
        let a = vent[0]
            .split(",")
            .map(str::parse::<isize>)
            .map(Result::unwrap)
            .collect::<Vec<isize>>();

        // Get Point b
        let b = vent[1]
            .split(",")
            .map(str::parse::<isize>)
            .map(Result::unwrap)
            .collect::<Vec<isize>>();

        // Create a Vent!
        vents.push(Vent {
            a: Point { x: a[0], y: a[1] },
            b: Point { x: b[0], y: b[1] },
        });

        // Update our field height/widths
        if a[0] > big_x {
            big_x = a[0];
        }
        if b[0] > big_x {
            big_x = b[0];
        }
        if a[1] > big_y {
            big_y = a[1];
        }
        if b[1] > big_y {
            big_y = b[1];
        }
    }

    // Create an Ocean Field, and fill it up with 0s
    let mut ocean_field = OceanField::default();
    ocean_field.origin = (0, 0).into();
    ocean_field.extent = (big_x, big_y).into();

    for _i in 0..big_y + 1 {
        for _j in 0..big_x + 1 {
            ocean_field.field.push(0);
        }
    }

    // Now, iterate through our vents (the flat ones), and update the field
    for vent in vents {
        // Update the field
        for point in vent.get_points() {
            let index: usize = ((point.y * (big_x + 1)) + point.x).try_into().unwrap();
            ocean_field.update_field(index);
        }
    }
    ocean_field.print();

    // Count!
    let mut sum = 0;
    for val in ocean_field.field {
        if val > 1 {
            sum = sum + 1;
        }
    }
    println!("Sum: {}", sum);

    Ok(())
}

#[test]
fn test_vent_is_flat() {
    let vertical = Vent {
        a: (1, 1).into(),
        b: (1, 3).into(),
    };
    let horizontal = Vent {
        a: (1, 1).into(),
        b: (3, 1).into(),
    };
    let diagonal = Vent {
        a: (1, 1).into(),
        b: (3, 5).into(),
    };
    assert_eq!(vertical.is_flat(), true);
    assert_eq!(horizontal.is_flat(), true);
    assert_eq!(diagonal.is_flat(), false);
}
