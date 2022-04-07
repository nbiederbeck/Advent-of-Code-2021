use std::collections::HashMap;
use std::fmt;

type Number = u8;
type Location = (usize, usize);
type Numbers = Vec<Number>;

#[derive(Copy, Clone)]
struct Tile {
    location: Location,
    marked: bool,
}

impl Tile {
    fn new(row: usize, col: usize) -> Self {
        Tile {
            location: (row, col),
            marked: false,
        }
    }
}

struct Board {
    tiles: HashMap<Number, Tile>,
}

impl From<&str> for Board {
    fn from(s: &str) -> Self {
        let mut tiles = HashMap::new();

        for (row, line) in s.lines().enumerate() {
            for (col, number) in line
                .split_ascii_whitespace()
                .map(str::parse::<Number>)
                .enumerate()
            {
                match number {
                    Ok(n) => {
                        let tile = Tile::new(row, col);
                        tiles.insert(n, tile);
                    }
                    _ => panic!("No Number in Input Board."),
                };
            }
        }
        Board { tiles: tiles }
    }
}

impl fmt::Debug for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut board = [[0 as u8; 5]; 5];
        for (n, t) in &self.tiles {
            let (x, y) = t.location;
            board[x][y] = *n;
        }
        write!(f, "{:?}", board)
    }
}

struct Bingo {
    boards: Vec<Board>,
}

impl From<&str> for Bingo {
    fn from(s: &str) -> Self {
        let mut boards = Vec::new();
        for board in s.split("\n\n").skip(1) {
            let b = Board::from(board);
            boards.push(b);
        }
        Bingo { boards: boards }
    }
}

fn draw_numbers(s: &str) -> Numbers {
    let mut numbers = Numbers::new();

    for number in s
        .lines()
        .nth(0)
        .expect("Input is broken")
        .split(',')
        .map(str::parse::<Number>)
    {
        match number {
            Ok(n) => numbers.push(n),
            e => panic!("no more bingo numbers drawn"),
        }
    }

    numbers
}

fn main() {
    let input = include_str!("input_test");
    let numbers = draw_numbers(input);
    let bingo = Bingo::from(input);

    for mut board in bingo.boards {
        dbg!(&board);
        for n in &numbers {
            match board.tiles.get(n) {
                Some(t) => {
                    let tile = Tile {
                        location: t.location,
                        marked: true,
                    };
                    board.tiles.insert(*n, tile);
                }
                None => {}
            }
        }
    }
    println!("Hello, world!");
}
