//! Tic-tac-toe
//!
//! A simple implementation of tic-tac-toe (a.k.a. naughts and crosses) in rust.
//!
//! This is a work-in-progress and was written as a pedagogical project.
//!
//! # TODO
//!     Game statistics.
//!     Multidimensional board.
//!     Optimise victory check.
//!     Coloured output.
//!     Redraw the board, rather than printing a new one.
//!     Add benchmarks.
//!     Increase unit test coverage & measure coverage.
extern crate clap;

use std::fmt;
use std::io;
use std::process::exit;
use std::io::prelude::*;
use std::str::FromStr;
use std::num::ParseIntError;


////////////////////////////////////////////////////////////////////////////////
// Types
////////////////////////////////////////////////////////////////////////////////


/// A single coordinate in two-dimensional space.
#[derive(Debug, PartialEq)]
struct Point {
    x: usize,
    y: usize,
}

/// A single square in the Tic-tac-toe board.
#[derive(Copy, Clone, PartialEq)]
enum Square {
    None,       // TODO: replace with Some()
    O,
    X,
}

/// The Tic-tac-toe board.
struct Board  {
    squares: [Square; 9],
}

impl FromStr for Point {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut i = s.split(",");
        let x_str = i.next().unwrap_or("");
        let y_str = i.next().unwrap_or("");
        let x = x_str.trim().parse::<usize>()?; // Todo: could trim ( here
        let y = y_str.trim().parse::<usize>()?; //       and trim   ) here.
        Ok(Point { x: x, y: y })
    }
}

impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Square::None   => write!(f, " "),
            Square::O => write!(f, "O"),
            Square::X  => write!(f, "X"),
        }
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "
             {} | {} | {}
            -- + - + --
             {} | {} | {}
            -- + - + --
             {} | {} | {}
            ",
            self.squares[0], self.squares[1], self.squares[2],
            self.squares[3], self.squares[4], self.squares[5],
            self.squares[6], self.squares[7], self.squares[8],
        )
    }
}

impl Board {

    fn point_to_index(&self, p: &Point) -> std::result::Result<usize, usize> {
        return if p.x <= 2 && p.y <= 2 { Ok(p.x + 3 * p.y) } else { Err(1) };
    }

    fn set_square(&mut self, p: &Point, square: &Square) -> std::result::Result<usize, usize> {
        let index = self.point_to_index(p)?;
        if self.squares[index] == Square::None {
            self.squares[index] = *square;
            return Ok(0);
        } else {
            return Err(2);
        }
    }
}


////////////////////////////////////////////////////////////////////////////////
// Control flow
////////////////////////////////////////////////////////////////////////////////


fn check_for_victory(board: &Board, player: &Square, _point: &Point) -> bool {
    // The winning patterns in the 1D array are:
    //
    //      0 1 2 3 4 5 6 7 8    index
    //      X X X - - - - - -    top row
    //      - - - X X X - - -    mid row
    //      - - - - - - X X X    bot row
    //      X - - X - - X - -    lef col
    //      - X - - X - - X -    mid col
    //      - - X - - X - - X    rig col
    //      X - - - X - - - X    lef diagonal
    //      - - X - X - X - -    rig diagonal
    //
    // TODO: Optimise this by only checking for combinations involving point.
    // TODO: Make a method of the board type?
    (board.squares[0] == *player && board.squares[1] == *player && board.squares[2] == *player) ||
    (board.squares[3] == *player && board.squares[4] == *player && board.squares[5] == *player) ||
    (board.squares[6] == *player && board.squares[7] == *player && board.squares[8] == *player) ||
    (board.squares[0] == *player && board.squares[3] == *player && board.squares[6] == *player) ||
    (board.squares[1] == *player && board.squares[4] == *player && board.squares[7] == *player) ||
    (board.squares[2] == *player && board.squares[5] == *player && board.squares[8] == *player) ||
    (board.squares[0] == *player && board.squares[4] == *player && board.squares[8] == *player) ||
    (board.squares[2] == *player && board.squares[4] == *player && board.squares[6] == *player)
}

fn turn(board:&mut Board, player: &Square) -> bool {
    let mut point: Point;

    println!("{}", board);

    loop {

        // Ask for input.
        print!("Player {}: ", player);
        io::stdout().flush().ok().expect("Could not flush stdout.");

        // Read stdin for input.
        // TODO: enforce a maximum size here and allocate on the stack.
        let mut input = String::with_capacity(4);
        match io::stdin().read_line(&mut input) {
            Ok(_r) => (),
            Err(e) => panic!("Cannot read stdin: {}", e)
        }

        // Parse the input to a point.
        match input.trim().as_ref() {
            "q" | "quit" | "e" | "exit" => exit(0),
            "h" | "help" | "i" | "info" => { help(); continue; },
            _                           => match Point::from_str(&input) {
                Ok (p) => point = p,
                Err(_) => {
                    println!("Please enter a valid point, e.g. 0, 1.");
                    continue;
                },
            }
        }

        // Try to set the square on the board.
        match board.set_square(&point, &player) {
            Ok(_r) => break,
            Err(1) => println!("That square is not on the board."),
            Err(2) => println!("That square is already taken."),
            Err(e) => panic!("Unexpected error when setting a square: {}", e)
        }
    }

    check_for_victory(&board, &player, &point)
}

fn game() {
    let players = [Square::O, Square::X];
    let mut board = Board { squares: [Square::None; 9] };
    let mut winner: std::option::Option<Square> = None;

    println!("Welcome to rusty tictactoe! :-)");

    for i in 0..board.squares.len() {
        let player = players[i % 2];
        if turn(&mut board, &player) {
            winner = Some(player);
            break;
        }
    }

    match winner {
        None         => println!("Stalemate, game over."),
        Some(player) => println!("Player {} has won! Congratulations!", player),
    }

    println!("Thank you for playing.");
}

fn help() {
    println!("
    To win, place three circles (or crosses) in a line.

    Specify a square to place a circle or cross by entering 'X, Y',
    where X and Y are the x-coordinate (column) and y-coordinate (row)
    respectively.

    Options:
        q/quit/e/exit      Exit the game.
        h/help/i/info      Display this text.
    ");
}

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {}

fn main() {
    let _args = Args::parse();
    game();
}


////////////////////////////////////////////////////////////////////////////////
// Tests
////////////////////////////////////////////////////////////////////////////////


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_point_from_str() {
        assert_eq!(Point::from_str("0, 0"), Ok(Point { x: 0, y: 0 }),
            "A string of '0, 0' should be parsed to a Point {{ x:0, y:0 }}.");
        assert_eq!(Point::from_str(" 1, 2 "), Ok(Point { x: 1, y: 2 }),
            "Leading and trailing whitespace should be stripped.");
        assert!(Point::from_str("1,").is_err(),
            "A missing Y value should produce an error.");
        assert!(Point::from_str(",4").is_err(),
            "A missing X value should produce an error.");
        assert!(Point::from_str("").is_err(),
            "An empty string should produce an error.");
    }

    #[test]
    fn test_board_point_to_index() {
        let board = Board { squares: [Square::None; 9] };
        assert_eq!(board.point_to_index(&Point {x:0, y:0}), Ok(0),
            "A Point {{x:0, y:0}} should index the first element.");
        assert_eq!(board.point_to_index(&Point {x:2, y:0}), Ok(2),
            "A Point {{x:2, y:0}} should index the third element.");
        assert_eq!(board.point_to_index(&Point {x:2, y:2}), Ok(8),
            "A Point {{x:2, y:2}} should index the last element.");
        assert_eq!(board.point_to_index(&Point {x:0, y:3}), Err(1),
            "An out-of-bounds y-coordinate should be an error.");
        assert_eq!(board.point_to_index(&Point {x:3, y:0}), Err(1),
            "An out-of-bounds x-coordinate should be an error.");
        assert_eq!(board.point_to_index(&Point {x:3, y:3}), Err(1),
            "Two out-of-bounds coordinates should be an error.");
    }

    #[test]
    fn test_board_set_square() {
        let mut board = Board { squares: [Square::None; 9] };
        assert_eq!(board.set_square(&Point {x:0, y:0}, &Square::O), Ok(0),
            "Setting an empty square inside the board be OK.");
        assert_eq!(board.set_square(&Point {x:3, y:3}, &Square::O), Err(1),
            "Setting a square outside the board should fail.");
    }

    #[test]
    fn test_check_for_victory() {
        use Square::{None, O, X};
        let boards_with_no_victory = [
            Board { squares: [Square::None; 9] },
            Board { squares: [None, None, None, None, None, None, None, None, X] },
        ];
        let players = [O, X];
        let points = [
            Point { x: 0, y: 0 }, Point { x: 1, y: 0 }, Point { x: 2, y: 0 },
            Point { x: 0, y: 1 }, Point { x: 1, y: 1 }, Point { x: 2, y: 1 },
            Point { x: 0, y: 2 }, Point { x: 1, y: 2 }, Point { x: 2, y: 2 },
        ];
        for board in boards_with_no_victory.iter() {
            for player in players.iter() {
                for point in points.iter() {
                    assert!(!check_for_victory(&board, &player, &point),
                        "should be false.");
                }
            }
        }
    }
}
