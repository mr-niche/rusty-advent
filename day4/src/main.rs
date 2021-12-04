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
    winner: bool,
}

impl BingoCard {
    fn new() -> BingoCard {
        let bc = Vec::new();
        BingoCard {
            bingo_card: bc,
            winner: false,
        }
    }
    fn push(&mut self, bn: BingoNumber) {
        self.bingo_card.push(bn)
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
    fn is_winner(&mut self) -> bool {
        let mut x_pos: Vec<u8> = Vec::new();
        let mut y_pos: Vec<u8> = Vec::new();
        for num in &self.bingo_card {
            if num.called == true {
                x_pos.push(num.pos.x);
                y_pos.push(num.pos.y);
            }
        }

        // Check if you have 5 of any particular x values
        for i in 0..5 {
            let x_s = x_pos.iter().filter(|&x| *x == i).count();
            let y_s = y_pos.iter().filter(|&y| *y == i).count();
            if x_s == 5 {
                //println!("x_pos win");
                self.winner = true;
                return true;
            }
            if y_s == 5 {
                //println!("y_pos win");
                self.winner = true;
                return true;
            }
        }
        false
    }
    fn score(&self) -> usize {
        let mut score: usize = 0;
        for num in &self.bingo_card {
            if num.called == false {
                score = score + usize::from(num.number);
            }
        }
        score
    }
}

#[derive(PartialEq, Debug, Default)]
struct BingoNumber {
    number: u8,
    pos: Position,
    called: bool,
}

impl BingoNumber {
    fn called(&mut self) {
        self.called = true
    }
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
    let input: Vec<&str> = include_str!("input").trim_end().split("\n\n").collect();

    // Parse input for call_sequence, BingoCards
    let bg: Option<BingoGame> = parse_input(input);

    // Now we have a bingo game, let's go through the call numbers!
    let winner = play_bingo(&mut bg.unwrap());
    dbg!(winner);

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

fn play_bingo(bg: &mut BingoGame) -> Option<usize> {
    for num in &bg.call_sequence {
        //println!("Calling...{}!", num);
        for card in &mut bg.bingo_cards {
            if card.winner == false {
                for bn in &mut card.bingo_card {
                    if bn.number == *num {
                        // Update the card, don't finish checking number
                        bn.called();
                        break;
                    }
                }
                if card.is_winner() {
                    println!("Winner with {}!", num);
                    card.print();
                    let last_called = usize::from(*num);
                    println!("Score: {}", card.score() * last_called);
                    println!();
                    //return Some(card.score() * last_called);
                }
            }
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

#[test]
fn test_bingo_number_called() {
    let mut bn: BingoNumber = { BingoNumber::default() };
    assert_eq!(bn.called, false);
    bn.called();
    assert_eq!(bn.called, true);
}
