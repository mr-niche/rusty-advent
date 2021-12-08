// Author: Nicholas LaJoie
// Advent of Code, Day 8

#[derive(Debug, Default)]
struct Entry<'a> {
    signals: Vec<&'a str>,
    digits: Vec<&'a str>,
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
        for d in e.digits {
            match d.len() {
                2 => acc = acc + 1,
                3 => acc = acc + 1,
                4 => acc = acc + 1,
                7 => acc = acc + 1,
                _ => continue,
            }
        }
    }

    dbg!(acc);

    Ok(())
}
