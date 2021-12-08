// Author: Nicholas LaJoie
// Advent of Code, Day 8

use itertools::Itertools;
use std::collections::HashMap;

#[derive(Debug, Default)]
struct Entry<'a> {
    signals: Vec<&'a str>,
    digits: Vec<&'a str>,
}

impl<'a> Entry<'a> {
    fn decode(&self) -> usize {
        // Segment mapping
        //     a  b  c  d  e  f  g
        // 0b0 -- -- -- -- -- -- --
        //     64 32 16  8  4  2  1

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

        let mut digits: HashMap<usize, &str> = HashMap::new();

        // Step 1, parse signals and determine what we know
        for signal in &self.signals {
            match signal.len() {
                2 => digits.insert(1, signal),
                3 => digits.insert(7, signal),
                4 => digits.insert(4, signal),
                7 => digits.insert(8, signal),
                _ => continue,
            };
        }
        println!();

        // Step 2, deduce the rest!

        let one = digits.get(&1).unwrap().clone();
        let four = digits.get(&4).unwrap().clone();
        let seven = digits.get(&7).unwrap().clone();
        let eight = digits.get(&8).unwrap().clone();

        // Side 'a' is 7 - 1
        let mut side_a = 'a';
        for c in seven.chars() {
            if !one.contains(c) {
                side_a = c;
            }
        }
        //println!("{} - {} = {}", seven, one, side_a);

        // 9 is a length 6 signal that contains all elements of 4
        for signal in &self.signals {
            if signal.len() == 6 {
                let mut is_nine = true;
                for c in four.chars() {
                    if !signal.contains(c) {
                        is_nine = false;
                        break;
                    }
                }
                if is_nine == true {
                    //println!("Four: {}, Nine: {}", four, signal);
                    digits.insert(9, signal);
                }
            }
        }

        // 2 is the only length 5 signal that isn't contained in 9
        let nine = digits.get(&9).unwrap().clone();
        for signal in &self.signals {
            if signal.len() == 5 {
                let mut is_two = false;
                for c in signal.chars() {
                    if !nine.contains(c) {
                        is_two = true;
                        break;
                    }
                }
                if is_two == true {
                    //println!("{} is not contained in {}", signal, nine);
                    digits.insert(2, signal);
                }
            }
        }

        // Side 'c' is the common value between 2 and 1
        let mut side_c = 'c';
        let two = digits.get(&2).unwrap().clone();
        for c in two.chars() {
            if one.contains(c) {
                side_c = c;
                break;
            }
        }
        //println!("Common between {} and {} is {}", two, one, side_c);

        // 6 is 8 - side_c
        let mut temp_six: Vec<char> = Vec::new();
        for c in eight.chars() {
            if !(c == side_c) {
                temp_six.push(c);
            }
        }
        let s = String::from_iter(temp_six);
        digits.insert(6, s.as_str());

        // 0 is the only remaining length 6 signal
        let six = digits.get(&6).unwrap().clone();
        let nine = digits.get(&9).unwrap().clone();
        for signal in &self.signals {
            if signal.len() == 6 {
                let mut not_six = false;
                let mut not_nine = false;
                for c in six.chars() {
                    if !signal.contains(c) {
                        not_six = true;
                    }
                }
                for c in nine.chars() {
                    if !signal.contains(c) {
                        not_nine = true;
                    }
                }
                if not_six == true && not_nine == true {
                    digits.insert(0, signal);
                }
            }
        }

        // 5 is contained in 6
        for signal in &self.signals {
            if signal.len() == 5 {
                let mut is_five = true;
                for c in signal.chars() {
                    if !six.contains(c) {
                        is_five = false;
                        break;
                    }
                }
                if is_five == true {
                    digits.insert(5, signal);
                }
            }
        }

        // 3 is the only remaining length 5 signal
        let five = digits.get(&5).unwrap().clone();
        for signal in &self.signals {
            if signal.len() == 5 {
                let mut not_two = false;
                let mut not_five = false;
                for c in two.chars() {
                    if !signal.contains(c) {
                        not_two = true;
                    }
                }
                for c in five.chars() {
                    if !signal.contains(c) {
                        not_five = true;
                    }
                }
                if not_two == true && not_five == true {
                    digits.insert(3, signal);
                }
            }
        }

        dbg!(digits);

        // THIS IS WRONG: 6 is 8 - 7 + 'a'
        //let mut six: Vec<char> = Vec::new();
        //for c in eight.chars() {
        //    if !seven.contains(c) {
        //        six.push(c);
        //    }
        //}
        //println!("{} - {} = {}", eight, seven, six.iter().collect::<String>());
        //six.push(side_a);
        //println!("Six = {}", six.iter().collect::<String>());

        // Step 3, join those digits together into a single value
        0
    }
}

fn main() -> anyhow::Result<()> {
    let example: Vec<&str> = include_str!("example").trim_end().split('\n').collect();
    let input: Vec<&str> = include_str!("input").trim_end().split('\n').collect();

    // Collect our input into an organized data structure
    let mut entries: Vec<Entry> = Vec::new();
    for line in example {
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
