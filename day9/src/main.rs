// Author: Nicholas LaJoie
// Advent of Code, Day 9

fn main() -> anyhow::Result<()> {
    // Process our input so we have a stream of integers & height & width
    let e: Vec<_> = include_str!("example").trim_end().split('\n').collect();
    let width: usize = e[0].chars().count();

    // Let's get creative...
    let mut field: Vec<isize> = Vec::new();

    // Build a box around the field with '-1's
    for i in 0..width + 2 {
        field.push(-1);
    }
    for line in e {
        field.push(-1);
        for d in line.chars() {
            field.push(d.to_digit(10).unwrap().try_into().unwrap());
        }
        field.push(-1);
    }
    for i in 0..width + 2 {
        field.push(-1);
    }

    dbg!(&field);
    dbg!(field.len());

    let input: Vec<u32> = include_str!("input")
        .trim_end()
        .chars()
        .filter(|c| *c != '\n')
        .map(|d| d.to_digit(10).unwrap())
        .collect();

    //let result = process_lows(&input, &width);
    //dbg!(result);

    Ok(())
}

fn wrap_field(field: &Vec<&str>) -> Option<Vec<isize>> {
    let new_field: Vec<isize> = Vec::new();

    let width: usize = field[0].chars().count();
}

fn process_lows(stream: &Vec<u32>, width: &usize) -> u32 {
    // Don't check left or above if element is 0
    // Don't check left if element is multiple of width (10, 20, etc.)
    // Don't check above if element is less than width
    // Don't check right if element is multiple of width (9, 19, 29)
    // Don't check below if (element + width) > num_elements
    let mut acc: u32 = 0;
    for (index, val) in stream.iter().enumerate() {
        let mut is_low = true;

        // ABOVE
        if index >= *width {
            if val >= &stream[index - width] {
                is_low = false;
            }
        }

        // BELOW
        if (index + width) < stream.len() {
            if val >= &stream[index + width] {
                is_low = false;
            }
        }

        // LEFT
        if !(index == 0) && !(index % width == 0) {
            if val >= &stream[index - 1] {
                is_low = false;
            }
        }

        // RIGHT
        if !(index == (width - 1)) && !(index % width == (width - 1)) {
            if val >= &stream[index + 1] {
                is_low = false;
            }
        }

        if is_low == true {
            acc = acc + val + 1;
        }
    }
    acc
}
