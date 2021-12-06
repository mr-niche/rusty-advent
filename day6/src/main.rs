// Author: Nicholas LaJoie
// Advent of Code, Day 6

#[derive(PartialEq, Debug, Default)]
struct LanternFish {
    init: usize,
    age: usize,
}

impl LanternFish {
    fn new(i: usize) -> LanternFish {
        LanternFish { init: i, age: i }
    }
    fn update(&mut self) -> Option<LanternFish> {
        match self.age {
            0 => {
                self.age = 6;
                return Some(LanternFish::new(8));
            }
            _ => {
                self.age = self.age - 1;
                return None;
            }
        }
    }
}

fn main() -> anyhow::Result<()> {
    let example: Vec<usize> = include_str!("example")
        .trim_end()
        .split(",")
        .map(str::parse::<usize>)
        .map(Result::unwrap)
        .collect();

    let input: Vec<usize> = include_str!("input")
        .trim_end()
        .split(",")
        .map(str::parse::<usize>)
        .map(Result::unwrap)
        .collect();

    let mut school_of_fish: Vec<LanternFish> = Vec::new();
    for age in input {
        school_of_fish.push(LanternFish::new(age));
    }

    // Print initial state
    print!("Initial state: ");
    for fish in &school_of_fish {
        print!("{},", fish.age);
    }
    println!();

    // For each day...
    for day in 1..81 {
        let mut newbies: Vec<LanternFish> = Vec::new();
        // For all fish...
        for fish in &mut school_of_fish {
            let new_fish: Option<LanternFish> = fish.update();
            if new_fish.is_some() {
                newbies.push(new_fish.unwrap());
            }
        }
        for n in newbies {
            school_of_fish.push(n);
        }
        //print_school(&school_of_fish, day);
        println!("Day {}: {}", day, &school_of_fish.len());
    }

    Ok(())
}

fn print_school(school: &Vec<LanternFish>, day: usize) {
    if day == 1 {
        print!("After {} day: ", day);
    } else {
        print!("After {} days: ", day);
    }

    for fish in school {
        print!("{},", fish.age);
    }
    println!();
}
