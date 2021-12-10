// Author: Nicholas LaJoie
// Advent of Code, Day 9

fn main() -> anyhow::Result<()> {
    let example: Vec<&str> = include_str!("example").trim_end().split('\n').collect();

    // Let's make a 2D Vec
    let mut field: Vec<Vec<u32>> = Vec::new();
    for nums in example {
        field.push(nums.chars().map(|d| d.to_digit(10).unwrap()).collect());
    }

    let result = get_lows(&field);

    Ok(())
}

fn get_lows(field: &Vec<Vec<u32>>) -> u32 {
    // Iterate through rows
    let mut acc = 0;
    for (row, val) in field.iter().enumerate() {
        for (col, digit) in val.iter().enumerate() {
            let mut is_low = true;
            println!("{}:{} = {}", row, col, digit);
            // Go left
            if !(col == 0) {
                println!(
                    "  Left: Check {}:{} = {}",
                    row,
                    col - 1,
                    val.get(col - 1).unwrap()
                );
            }
            // Go up
            if !(row == 0) {
                println!(
                    "  Up:   Check {}:{} = {}",
                    row - 1,
                    col,
                    field[row - 1].get(col).unwrap()
                );
            }
            // Go right
            if !(col == val.len() - 1) {
                println!(
                    "  Right: Check {}:{} = {}",
                    row,
                    col + 1,
                    val.get(col + 1).unwrap()
                );
            }
            // Go down
            if !(row == field.len() - 1) {
                println!(
                    "  Down: Check {}:{} = {}",
                    row + 1,
                    col,
                    field[row + 1].get(col).unwrap()
                );
            }
        }
    }
    0
}
