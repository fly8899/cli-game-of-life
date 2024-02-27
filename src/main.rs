use crate::{board::Board, board::BoardTrait, cell::Cell, location::Location};
use std::{thread::sleep, time::Duration};

mod board;
mod cell;
mod location;

fn main() {
    let data = vec![
        Location { x: 45, y: 65 },
        Location { x: 46, y: 65 },
        Location { x: 48, y: 65 },
        Location { x: 49, y: 65 },
        Location { x: 45, y: 66 },
        Location { x: 49, y: 66 },
        Location { x: 46, y: 67 },
        Location { x: 47, y: 67 },
        Location { x: 48, y: 67 },
    ];

    let mut board: Board<Cell> = BoardTrait::new_with(100, 90, data);

    while !board.is_empty() {
        println!("{}", board);
        println!("---------------");
        sleep(Duration::from_millis(200));
        board.step();
    }
}
