// Author: Nicholas LaJoie
// Advent of Code, Day 8

#[derive(Debug, Default)]
struct Entry<'a> {
    signals: Vec<&'a str>,
    digits: Vec<&'a str>,
}

impl<'a> Entry<'a> {
    fn decode(&self) -> usize {
        let digit_lookup = vec![
            0b0111_0111, // 0
            0b0001_0010, // 1
            0b0101_1101, // 2
            0b0101_1011, // 3
            0b0011_0010, // 4
            0b0110_1011, // 5
            0b0110_1111, // 6
            0b0101_0010, // 7
            0b0111_1111, // 8
            0b0111_1011, // 9
        ];

        0
        // Step 1, parse signals and determine mapping

        // Step 2, use mapping to decode digits

        // Step 3, join those digits together into a single value
    }
}

fn main() -> anyhow::Result<()> {
    let example: Vec<&str> = include_str!("example").trim_end().split('\n').collect();
    let input: Vec<&str> = include_str!("input").trim_end().split('\n').collect();

    // Collect our input into an organized data structure
    let mut entries: Vec<Entry> = Vec::new();
    for line in input {
        let components: Vec<&str> = line.split(" | ").collect();
        if components.len() == 2 {
            let s: Vec<&str> = components[0].split_whitespace().collect();
            let d: Vec<&str> = components[1].split_whitespace().collect();
            entries.push(Entry {
                signals: s,
                digits: d,
            });
        }
    }

    // Figure this sh*t out
    let mut acc = 0;
    for e in entries {
        acc = acc + e.decode();
    }

    dbg!(acc);

    Ok(())
}
