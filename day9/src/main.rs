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

    // Get a vector of low-point coordinates
    let lows = get_lows(&field);

    // Find. Those. Basins!
    let mut basins: Vec<usize> = Vec::new();
    let mut visited: Vec<(usize, usize)> = Vec::new();
    for low in lows {
        // Get the size of the basin!
        basins.push(get_basin_livin(&field, &low, &mut visited)); // or get_basin_dying
    }

    // Find top three and multiply em
    basins.sort_by(|a, b| a.cmp(b).reverse());
    basins.truncate(3);
    println!("Answer: {}", basins[0] * basins[1] * basins[2]);
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

fn get_basin_livin(
    field: &Vec<Vec<u32>>,
    low: &(usize, usize),
    visited: &mut Vec<(usize, usize)>,
) -> usize {
    let mut acc = 1;
    let row = low.0;
    let col = low.1;
    let width = field[0].len();
    let height = field.len();

    // Note that we've visited this position
    visited.push(*low);

    // If it's not 9, count, and go looking for another
    if *field[row].get(col).unwrap() == 9 {
        return 0;
    } else {
        // Count left
        if !(col == 0) && !visited.contains(&(row, col - 1)) {
            acc = acc + get_basin_livin(&field, &(row, col - 1), visited);
        }
        // Count right
        if !(col == width - 1) && !visited.contains(&(row, col + 1)) {
            acc = acc + get_basin_livin(&field, &(row, col + 1), visited);
        }
        // Count up
        if !(row == 0) && !visited.contains(&(row - 1, col)) {
            acc = acc + get_basin_livin(&field, &(row - 1, col), visited);
        }
        // Count down
        if !(row == height - 1) && !visited.contains(&(row + 1, col)) {
            acc = acc + get_basin_livin(&field, &(row + 1, col), visited);
        }
        return acc;
    }
}
