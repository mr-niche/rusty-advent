// Author: Nicholas LaJoie
// Advent of Code, Day 4

#[derive(PartialEq, Debug, Default)]
struct BingoGame {
    call_sequence: Vec<u8>,
    bingo_cards: Vec<BingoCard>,
}

impl BingoGame {
    fn new() -> BingoGame {
        let cs = Vec::new();
        let bc = Vec::new();
        BingoGame {
            call_sequence: cs,
            bingo_cards: bc,
        }
    }
}

#[derive(PartialEq, Debug, Default)]
struct BingoCard {
    bingo_card: Vec<BingoNumber>,
}

impl BingoCard {
    fn new() -> BingoCard {
        let bc = Vec::new();
        BingoCard { bingo_card: bc }
    }
    fn push(&mut self, bn: BingoNumber) {
        self.bingo_card.push(bn)
    }
    fn play(&mut self, called: u8) {
        for num in &self.bingo_card {
            if num.number == called {
                println!("Found {}", called);
                return;
            }
        }
    }
    fn print(&self) {
        let mut y = 0;
        for num in &self.bingo_card {
            if y < 4 {
                print!("{} ", num.number);
                y = y + 1;
            } else if y == 4 {
                println!("{}", num.number);
                y = 0;
            }
        }
    }
}

#[derive(PartialEq, Debug, Default)]
struct BingoNumber {
    number: u8,
    pos: Position,
    called: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
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
    let _input: Vec<&str> = include_str!("input").trim_end().split("\n\n").collect();

    // Parse input for call_sequence, BingoCards
    let bg: Option<BingoGame> = parse_input(example);
    //dbg!(bg);

    // Now we have a bingo game, let's go through the call numbers!
    let winner = play_bingo(&bg.unwrap());

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

    for card in input.iter().skip(1) {
        // Create a card
        let mut bc = BingoCard::new();
        let mut x = 0;
        let mut y = 0;
        for line in card.split('\n') {
            for num in line.split_whitespace().collect::<Vec<&str>>() {
                // Create a Bingo number!
                let bn = BingoNumber {
                    number: num.parse::<u8>().unwrap(),
                    pos: (x, y).into(),
                    called: false,
                };
                bc.push(bn);
                x = x + 1;
            }
            y = y + 1;
            x = 0;
        }
        bg.bingo_cards.push(bc);
    }
    Some(bg)
}

fn play_bingo(bg: &BingoGame) -> Option<BingoCard> {
    for num in &bg.call_sequence {
        println!("Calling...{}!", num);
        for card in &bg.bingo_cards {
            card.print();
            println!();
            //dbg!(board);
        }
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
