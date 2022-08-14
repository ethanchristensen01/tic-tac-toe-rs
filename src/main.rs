use array2d::Array2D;
use std::fmt;
use std::io;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    X,
    O,
    Empty,
    Cat
}
use crate::Tile::*;

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Empty => write!(f, "_"),
            tile => write!(f, "{tile:?}")
        }
    }
}

struct Board (Array2D<Tile>);

#[derive(Debug)]
enum BoardErr {
    BlockedError,
    BoundsError
}

impl Board {
    fn new(tile: Tile, width: usize, height: usize) -> Board {
        Board(Array2D::filled_with(tile, height, width))
    }

    fn width(&self) -> usize {
        self.0.row_len()
    }

    fn height(&self) -> usize {
        self.0.column_len()
    }

    fn place(&mut self, tile: Tile, row: usize, column: usize) -> Result<(), BoardErr> {
        match self.0.get(row, column) {
            Some(Empty) => {
                match self.0.set(row, column, tile) {
                    Err(_) => Err(BoardErr::BoundsError),
                    _ => Ok(())
                }
            },
            Some(_) => {
                return Err(BoardErr::BlockedError)
            },
            None => {
                return Err(BoardErr::BoundsError)
            }
        }
    }

    #[allow(unused)]
    fn place_drop(&mut self, tile: Tile, column: usize) -> Result<(), BoardErr> {
        if column >= self.width() {
            return Err(BoardErr::BoundsError);
        }
        let first_tile = self.0.column_iter(column).position(|&t| t != Empty);
        match first_tile {
            Some(row) if row == 0 => Err(BoardErr::BlockedError),
            Some(row) => self.place(tile, row - 1, column),
            None => self.place(tile, self.height() - 1, column)
        }
    }

    fn winner(&self, winning_score: usize) -> Tile {
        for straight in self.all_straights() {
            match Self::winner_in_straight(winning_score, straight) {
                Empty => continue,
                tile => return tile
            }
        }
        if self.0.as_row_major().iter().all(|&t| t != Empty) {
            Cat
        } else {
            Empty
        }
    }

    fn all_straights(&self) -> Vec<Vec<Tile>> {
        // let mut straights: Vec<Vec<Tile>> = vec![];
        let mut result: Vec<Vec<Tile>> = vec![];
        result.extend(self.0.as_rows());
        result.extend(self.0.as_columns());
        result.extend(self.all_diagonals());
        result
            // straights.extend_from_slice(self.0.rows_iter());
        // straights.extend_from_slice(self.0.columns_iter().as_slice());

        // straights.into_iter()
    }

    fn all_diagonals(&self) -> Vec<Vec<Tile>> {


        let mut v: Vec<Vec<Tile>> = vec![];
        for row in 0..self.height() {
            v.push(self.diagonal_down(row, 0));
            v.push(self.diagonal_up(row, 0));
        }
        for col in 1..self.width() {
            v.push(self.diagonal_down(0, col));
            v.push(self.diagonal_up(self.height() - 1, col));
        }
        v
    }

    fn diagonal_down(&self, row: usize, col: usize) -> Vec<Tile> {
        let mut row = row;
        let mut col = col;
        let mut result = vec![];
        loop {
            match self.0.get(row, col) {
                Some(tile) => result.push(*tile),
                None => break
            }
            row += 1;
            col += 1;
        }
        result
    }

    fn diagonal_up(&self, row: usize, col: usize) -> Vec<Tile> {
        let mut row = row;
        let mut col = col;
        let mut result = vec![];
        loop {
            match self.0.get(row, col) {
                Some(tile) => result.push(*tile),
                None => break
            }
            if row == 0 {
                break;
            }
            row -= 1;
            col += 1;
        }
        result
    }

    fn winner_in_straight(winning_score: usize, tiles: Vec<Tile>) -> Tile {
        let mut score = 1;
        let mut current = Empty;
        for tile in tiles {
            match tile {
                same if current == same =>{
                    score += 1;
                },
                other => {
                    score = 1;
                    current = other;
                }
            };
            if score == winning_score && current != Empty {
                return current
            }
        };
        Empty
    }


}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "  ").unwrap();
        for col_num in 1..=self.0.num_columns() {
            write!(f, " {col_num} ").unwrap();
        }
        for (row_num, row_iter) in self.0.rows_iter().enumerate() {
            write!(f, "\n").unwrap();
            write!(f, "{} ", row_num + 1).unwrap();
            for tile in row_iter {
                write!(f, " {tile} ").unwrap();
            }
        }
        write!(f, "\n")
    }
}

fn prompt (query: &str) -> String {
    loop {
        println!("{}", query);

        let mut out = String::new();


        match io::stdin().read_line(&mut out) {
            Ok(_) => return out,
            Err(_) => println!("Failed to read line")
        }
    }
}
fn main() {
    let width = 3;
    let height = 3;
    let mut board = Board::new(Empty, width, height);
    let mut _pos = 0;
    let winning_score = 3;
    macro_rules! place {
        ($tile:expr => ($row:expr, $col:expr)$(;)*) => {
            board.place(
                $tile,
                $row - 1,
                $col - 1
            )
        };

        ($tile:expr => ($row:expr, $col:expr); $($tail:expr => ($tr:expr, $tc:expr));+) => {
            place!($tile => ($row, $col));
            place!($($tail => ($tr, $tc));+)
        };

        ($tile:expr => v($col:expr)$(;)*) => {
            board.place_drop(
                $tile,
                $col - 1
            )
        };

        ($tile:expr => v($col:expr); $($tail:expr => v($tc:expr));+) => {
            match place!($tile => v($col)) {
                Ok(_) => place!($($tail => v($tc));+),
                err => err
            }
        };

        ($tile:expr$(;)*) => {{
            let res = place!(
                $tile =>
                (_pos/board.width() + 1,
                (_pos) % board.width() + 1)
            );
            _pos += 1;
            res
        }};

        ($tile:expr, $($tail:expr),+$(;)*) => {{
            match place!($tile) {
                Ok(_) => place!($($tail),+),
                err => err
            }
        }};




    }

    macro_rules! clear {
        () => {
            board = Board::new(Empty, width, height)
        };
    }

    macro_rules! print_board {
        () => {
            println!("{board}")
        };
    }

    macro_rules! prompt_num {
        ($query:expr) => {
            loop {
                match prompt($query).trim().parse() {
                    Err(_) => println!("That was not a valid number--try again"),
                    Ok(num) => break num
                };
            }
        };

        ($query:expr => ($lower_bound:expr, $upper_bound:expr)) => {
            loop {
                match prompt_num!($query) {
                    num if (num >= $lower_bound && num <= $upper_bound) => break num,
                    _ => println!("Number out of bounds--try again"),
                };
            }
        };
    }

    loop {
        let mut turn:usize = 0;
        loop {
            print_board!();
            match board.winner(winning_score) {
                Empty => (),
                Cat => {
                    println!("There are no spaces left.");
                    println!("Nobody wins");
                    break
                },
                tile => {
                    println!("{tile} got {winning_score} in a row!");
                    println!("{tile} wins!");
                    break
                }
            }

            let tile = if turn % 2 == 0 {
                X
            }
            else {
                O
            };

            println!("It's {tile}'s turn");

            println!();
            let col: usize = prompt_num!("Select a column: " => (1, width));
            let row: usize = prompt_num!("Select a row: " => (1, height));


            match place!(tile => (row, col);) {
                Ok(()) => turn += 1,
                Err(BoardErr::BlockedError) => {
                    println!("That space is full--try somewhere else");
                    continue;
                },
                Err(BoardErr::BoundsError) => {
                    println!("That space is out of bounds--try somewhere else");
                    continue;
                }
            }
        }

        let play_again = prompt("Play again? (Y/n)");

        if play_again.starts_with(&['n', 'N']) {
            break;
        }
        clear!();
    }
}
