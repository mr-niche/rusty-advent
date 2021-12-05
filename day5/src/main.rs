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

#[derive(PartialEq, Debug, Default)]
struct OceanField {
    origin: Point,
    extent: Point,
    field: Vec<usize>,
}

impl OceanField {
    fn new() -> OceanField {
        let of = Vec::new();
        OceanField {
            origin: Point::default(),
            extent: Point::default(),
            field: of,
        }
    }
    fn print(&self) {
        for i in 0..self.extent.y {
            for j in 0..self.extent.x {
                let val = self.field.iter().next().unwrap();
                if *val == 0 {
                    print!(".");
                } else {
                    println!("{}", val);
                }
            }
            println!();
        }
    }
}

fn main() -> anyhow::Result<()> {
    let example: Vec<&str> = include_str!("example").trim_end().split('\n').collect();
    let _input: Vec<&str> = include_str!("input").trim_end().split('\n').collect();

    let mut vents: Vec<Vent> = Vec::new();
    let mut big_x: usize = 0;
    let mut big_y: usize = 0;
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
    //dbg!(vents);

    // Create an Ocean Field, and fill it up with 0s
    let mut ocean_field = OceanField::default();
    ocean_field.origin = (0, 0).into();
    ocean_field.extent = (big_x, big_y).into();

    for i in 0..big_y {
        for j in 0..big_x {
            ocean_field.field.push(0);
        }
    }
    ocean_field.print();

    // Now, iterate through our vents (the flat ones), and update the field

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
