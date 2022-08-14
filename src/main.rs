use array2d::Array2D;
use std::fmt;
use std::io;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    X,
    O,
    Empty
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
        Empty
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
        for row in self.0.rows_iter() {
            for tile in row {
                write!(f, " {tile} ").unwrap();
            }
            write!(f, "\n").unwrap();
        }
        write!(f, "\n")
    }
}

fn main() {
    let width = 7;
    let height = 6;
    let mut board = Board::new(Empty, width, height);
    let mut _pos = 0;
    let winning_score = 4;
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

    // {
    //     let e = Empty;
    //     place!(
    //         e, e, e, e, e, e, e,
    //         e, e, e, e, e, e, e,
    //         e, e, O, e, e, e, e,
    //         e, O, O, O, X, e, e,
    //         e, X, X, X, O, e, e,
    //         e, O, X, X, X, O, e;
    //     ).unwrap();
    //     clear!();
    //     match place!(
    //         X => v(1);
    //         X => v(1);
    //         X => v(1);
    //         X => v(1);
    //         X => v(1);
    //         X => v(1);
    //         O => v(3);
    //         O => v(3)
    //     ) {
    //         Err(BoardErr::BoundsError) => panic!("We're out of bounds!"),
    //         Err(BoardErr::BlockedError) => println!("You can't put that there..."),
    //         _ => ()
    //     };
    // }

    loop {
        let mut turn:usize = 0;
        loop {
            print_board!();
            match board.winner(winning_score) {
                Empty => (),
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

            println!("{tile}, which column do you want to place your piece?");
            let mut col = String::new();

            io::stdin()
                .read_line(&mut col)
                .expect("Failed to read line");

            let col: usize = match col.trim().parse() {
                Ok(num) => num,
                _ => {
                    println!("That wasn't a positive number--try again");
                    continue
                }
            };

            match place!(tile => v(col);) {
                Ok(()) => turn += 1,
                Err(BoardErr::BlockedError) => {
                    println!("Column {col} is full--try somewhere else");
                    continue;
                },
                Err(BoardErr::BoundsError) => {
                    println!("There is no column {col}--try a column between 1 and {width}");
                    continue;
                }
            }
        }

        println!("Play again? (Y/n)");
        let mut play_again = String::new();
        io::stdin()
            .read_line(&mut play_again)
            .expect("Failed to read line");

        if play_again.starts_with(&['y', 'Y']) {
            clear!();
            continue;
        }
        break;
    }
}
