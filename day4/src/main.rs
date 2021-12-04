// Author: Nicholas LaJoie
// Advent of Code, Day 4

#[derive(PartialEq, Debug, Default)]
struct BingoGame {
    call_sequence: Vec<u8>,
    //bingo_cards: Vec<BingoCard>,
}

impl BingoGame {
    pub fn new() -> BingoGame {
        // Call Sequence init
        let cs = Vec::new();
        BingoGame { call_sequence: cs }
    }
}

#[derive(PartialEq, Debug, Default)]
struct BingoCard {
    bingo_card: Vec<BingoNumber>,
}

#[derive(PartialEq, Debug)]
struct BingoNumber {
    number: u8,
    pos: Position,
    called: bool,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Position {
    x: u8,
    y: u8,
}

impl From<(u8, u8)> for Position {
    fn from((x, y): (u8, u8)) -> Self {
        Self { x, y }
    }
}

fn main() -> anyhow::Result<()> {
    // Load input
    let example: Vec<&str> = include_str!("example").trim_end().split("\n\n").collect();
    let input: Vec<&str> = include_str!("input").trim_end().split("\n\n").collect();

    // Parse input for call_sequence, BingoCards
    let bg = parse_input(example);
    dbg!(bg);

    //dbg!(example);

    Ok(())
}

// FUNCTIONS!
fn parse_input(input: Vec<&str>) -> Option<BingoGame> {
    // Our input comes in as:
    // line 1 - the call_sequence
    // line 2..end - a &str of values of a bingo card
    let mut bg = BingoGame::new();

    // Get the call sequence
    let call_sequence = input[0].split(',').collect::<Vec<&str>>();
    for num in call_sequence {
        bg.call_sequence.push(num.parse::<u8>().unwrap());
    }

    None
}

// TESTS!
#[test]
fn test_build_position() {
    let p: Position = (0, 5).into();
    assert_eq!(p.x, 0);
    assert_eq!(p.y, 5);
}
