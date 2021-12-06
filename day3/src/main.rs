// Author: Nicholas LaJoie
// Advent of Code, Day 3

fn main() -> anyhow::Result<()> {
    let example: Vec<&str> = include_str!("example").trim_end().split('\n').collect();
    let input: Vec<&str> = include_str!("input").trim_end().split('\n').collect();

    //dbg!(example);
    let oxy_rating = oxygen_rating(&input);
    let co2_rating = co2_rating(&input);

    // Each line is a "bite", example: "00100"
    println!("Result: {}", oxy_rating.unwrap() * co2_rating.unwrap());

    Ok(())
}

// For Oxygen Generator Rating, per bit position:
// 1. Keep bites that start w/most common bit
// 2. If equal, keep bites that start w/1
fn oxygen_rating(bites: &Vec<&str>) -> Option<u64> {
    let mut keepers: Vec<&str> = Vec::new();
    let bite_width = bites[0].len();

    // Copy the bites
    for b in bites {
        keepers.push(b);
    }

    // Outer loop - each bit field (0 -> bite_width)
    for bit_position in 0..bite_width {
        // Inner loop - determine most common bit
        let mut ones = 0;
        let mut mcb = "0";

        // Stop if we only have one keeper left
        if keepers.len() == 1 {
            break;
        }

        for b in &keepers {
            if b.get(bit_position..bit_position + 1).unwrap() == "1" {
                ones = ones + 1;
            }
        }
        if ones >= (keepers.len() - ones) {
            mcb = "1";
        }

        // And then remove the bites that _don't_ start w/that bit
        let mut remove_these: Vec<usize> = Vec::new();
        for (index, k) in keepers.iter().enumerate() {
            if k.get(bit_position..bit_position + 1).unwrap() != mcb {
                remove_these.push(index);
            }
        }
        for i in remove_these.iter().rev() {
            keepers.remove(*i);
        }
    }

    // Convert to decimal!
    let mut result = 0;
    let keeper = keepers[0];
    let base: u64 = 2;
    for pos in 0..bite_width {
        let power: u32 = (bite_width - pos - 1).try_into().unwrap();
        if keeper.get(pos..pos + 1).unwrap() == "1" {
            result = result + base.pow(power);
        }
    }
    dbg!(&keeper);
    Some(result)
}

// For CO2 Scrubber Rating, per bit position:
// 1. Keep bites that start w/least common bit
// 2. If equal, keep bites that start w/0
fn co2_rating(bites: &Vec<&str>) -> Option<u64> {
    let mut keepers: Vec<&str> = Vec::new();
    let bite_width = bites[0].len();

    // Copy the bites
    for b in bites {
        keepers.push(b);
    }

    // Outer loop - each bit field (0 -> bite_width)
    for bit_position in 0..bite_width {
        // Inner loop - determine most common bit
        let mut ones = 0;
        let mut lcb = "0";

        // Stop if we only have one keeper left
        if keepers.len() == 1 {
            break;
        }

        for b in &keepers {
            if b.get(bit_position..bit_position + 1).unwrap() == "1" {
                ones = ones + 1;
            }
        }
        if ones < (keepers.len() - ones) {
            lcb = "1";
        }

        // And then remove the bites that _don't_ start w/that bit
        let mut remove_these: Vec<usize> = Vec::new();
        for (index, k) in keepers.iter().enumerate() {
            if k.get(bit_position..bit_position + 1).unwrap() != lcb {
                remove_these.push(index);
            }
        }
        for i in remove_these.iter().rev() {
            keepers.remove(*i);
        }
    }

    // Convert to decimal!
    let mut result = 0;
    let keeper = keepers[0];
    let base: u64 = 2;
    for pos in 0..bite_width {
        let power: u32 = (bite_width - pos - 1).try_into().unwrap();
        if keeper.get(pos..pos + 1).unwrap() == "1" {
            result = result + base.pow(power);
        }
    }
    dbg!(&keeper);
    Some(result)
}
