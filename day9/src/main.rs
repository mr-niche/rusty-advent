// Author: Nicholas LaJoie
// Advent of Code, Day 9

fn main() -> anyhow::Result<()> {
    let _example: Vec<&str> = include_str!("example").trim_end().split('\n').collect();
    let input: Vec<&str> = include_str!("input").trim_end().split('\n').collect();

    // Let's make a 2D Vec
    let mut field: Vec<Vec<u32>> = Vec::new();
    for nums in input {
        field.push(nums.chars().map(|d| d.to_digit(10).unwrap()).collect());
    }

    let result = get_lows(&field);
    dbg!(result);

    Ok(())
}

fn get_lows(field: &Vec<Vec<u32>>) -> u32 {
    // Iterate through rows
    let mut acc = 0;
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
                acc = acc + digit + 1;
            }
        }
    }
    acc
}
