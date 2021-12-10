// Author: Nicholas LaJoie
// Advent of Code, Day 9

fn main() -> anyhow::Result<()> {
    let example: Vec<&str> = include_str!("example").trim_end().split('\n').collect();
    let _input: Vec<&str> = include_str!("input").trim_end().split('\n').collect();

    // Let's make a 2D Vec
    let mut field: Vec<Vec<u32>> = Vec::new();
    for nums in example {
        field.push(nums.chars().map(|d| d.to_digit(10).unwrap()).collect());
    }

    // Get a vector of low-point coordinates
    let lows = get_lows(&field);

    // TODO: Find. Those. Basins!

    Ok(())
}

// Return a vector of low point coordinates
fn get_lows(field: &Vec<Vec<u32>>) -> Vec<(usize, usize)> {
    let mut lows: Vec<(usize, usize)> = Vec::new();
    // Iterate through rows
    for (row, val) in field.iter().enumerate() {
        for (col, digit) in val.iter().enumerate() {
            let mut neighbors: Vec<u32> = Vec::new();
            // Go left
            if !(col == 0) {
                neighbors.push(*val.get(col - 1).unwrap());
            }
            // Go up
            if !(row == 0) {
                neighbors.push(*field[row - 1].get(col).unwrap());
            }
            // Go right
            if !(col == val.len() - 1) {
                neighbors.push(*val.get(col + 1).unwrap());
            }
            // Go down
            if !(row == field.len() - 1) {
                neighbors.push(*field[row + 1].get(col).unwrap());
            }
            // Check it
            if digit < neighbors.iter().min().unwrap() {
                lows.push((row, col));
            }
        }
    }
    lows
}
