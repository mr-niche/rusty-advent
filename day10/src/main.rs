// Author: Nicholas LaJoie
// Co-Auther: Kent Seneres (the man with the wisdom)
// Advent of Code, Day 10

fn main() -> anyhow::Result<()> {
    let _example: Vec<&str> = include_str!("example").trim_end().split('\n').collect();
    let input: Vec<&str> = include_str!("input").trim_end().split('\n').collect();

    let mut scores: Vec<usize> = Vec::new();
    for line in input {
        let l = String::from(line);
        let result = get_bad_char(&l);
        if result.is_none() {
            //println!("{} is incomplete", l);
            scores.push(autocomplete(&l));
        }
    }

    // Find the middle score
    scores.sort();
    let winner = scores[scores.len() / 2];
    dbg!(winner);

    Ok(())
}

fn get_bad_char(line: &String) -> Option<char> {
    let mut list: Vec<char> = Vec::new();
    for l in line.chars() {
        if is_opener(&l) {
            list.push(l);
        } else if is_closer(&l) {
            let last = &list.last().unwrap();
            let l_opener = get_opener(&l).unwrap();
            if **last == l_opener {
                // We found a pair!
                list.pop();
            } else {
                return Some(l);
            }
        }
    }
    None
}

fn autocomplete(line: &String) -> usize {
    let mut list: Vec<char> = Vec::new();
    for l in line.chars() {
        if is_opener(&l) {
            list.push(l);
        } else if is_closer(&l) {
            let last = &list.last().unwrap();
            let l_opener = get_opener(&l).unwrap();
            if **last == l_opener {
                // We found a pair!
                list.pop();
            }
        }
    }
    // Get our score
    let mut acc = 0;
    for l in list.into_iter().rev() {
        match l {
            '(' => acc = (acc * 5) + 1,
            '[' => acc = (acc * 5) + 2,
            '{' => acc = (acc * 5) + 3,
            '<' => acc = (acc * 5) + 4,
            _ => continue,
        }
    }
    acc
}

fn is_opener(character: &char) -> bool {
    match character {
        '(' => return true,
        '[' => return true,
        '{' => return true,
        '<' => return true,
        _ => return false,
    }
}

fn is_closer(character: &char) -> bool {
    match character {
        ')' => return true,
        ']' => return true,
        '}' => return true,
        '>' => return true,
        _ => return false,
    }
}

fn get_opener(character: &char) -> Option<char> {
    if is_closer(&character) {
        match character {
            ')' => return Some('('),
            ']' => return Some('['),
            '}' => return Some('{'),
            '>' => return Some('<'),
            _ => return None,
        }
    }
    None
}

fn get_closer(character: &char) -> Option<char> {
    if is_opener(&character) {
        match character {
            '(' => return Some(')'),
            '[' => return Some(']'),
            '{' => return Some('}'),
            '<' => return Some('>'),
            _ => return None,
        }
    }
    None
}

#[test]
fn test_is_opener() {
    let a = '(';
    let b = ')';
    let c = '*';
    assert_eq!(is_opener(&a), true);
    assert_eq!(is_opener(&b), false);
    assert_eq!(is_opener(&c), false);
}

#[test]
fn test_is_closer() {
    let a = '(';
    let b = ')';
    let c = '*';
    assert_eq!(is_closer(&a), false);
    assert_eq!(is_closer(&b), true);
    assert_eq!(is_closer(&c), false);
}

#[test]
fn test_get_opener() {
    let a = '(';
    let b = ')';
    let c = '*';
    assert_eq!(get_opener(&a), None);
    assert_eq!(get_opener(&b), Some('('));
    assert_eq!(get_opener(&c), None);
}

#[test]
fn test_get_closer() {
    let a = '(';
    let b = ')';
    let c = '*';
    assert_eq!(get_closer(&a), Some(')'));
    assert_eq!(get_closer(&b), None);
    assert_eq!(get_closer(&c), None);
}
