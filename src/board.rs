use std::fmt::{Debug, Display};

use crate::location::Location;

use super::cell::CellTrait;

pub trait BoardTrait<T>
where
    T: CellTrait + Clone + Default + Debug,
{
    fn get(&self, x: usize, y: usize) -> Option<&T>;
    fn insert(&mut self, x: usize, y: usize, data: T);
    fn step(&mut self);
    fn is_empty(&self) -> bool;
    fn formatted(&self) -> String;
    fn new_with(x: usize, y: usize, data: Vec<Location>) -> Self;
    fn new(x: usize, y: usize) -> Self;
}

pub struct Board<T>
where
    T: CellTrait + Clone + Default + Debug,
{
    rows: Vec<Vec<T>>,
}

impl<T> Display for Board<T>
where
    T: CellTrait + Clone + Default + Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Ok(for row in &self.rows {
            let mut s = "".to_owned();
            for cell in row {
                s.push_str(format!("{}", cell.formatted()).as_str())
            }

            write!(f, "{}\n", s).unwrap();
        })
    }
}

impl<T> BoardTrait<T> for Board<T>
where
    T: CellTrait + Clone + Default + Debug,
{
    fn get(&self, x: usize, y: usize) -> Option<&T> {
        let row = self.rows.get(y)?;
        row.get(x)
    }

    fn insert(&mut self, x: usize, y: usize, data: T) {
        let option = self.rows.get(y);

        if option.is_none() {
            return;
        }

        let mut row = option.unwrap().to_owned();

        if row.len() <= x {
            return;
        }

        row[x] = data;
        self.rows[y] = row;
    }

    fn step(&mut self) {
        let mut board = Board::new(self.rows[0].len(), self.rows.len());

        for (y, row) in self.rows.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                let next = self.get(x + 1, y);

                let below = self.get(x, y + 1);

                let before = if let Some(ci) = x.checked_sub(1) {
                    self.get(ci, y)
                } else {
                    None
                };

                let above = if let Some(bi) = y.checked_sub(1) {
                    self.get(x, bi)
                } else {
                    None
                };

                let above_before = if let Some(bi) = y.checked_sub(1) {
                    if let Some(ci) = x.checked_sub(1) {
                        self.get(ci, bi)
                    } else {
                        None
                    }
                } else {
                    None
                };

                let above_next = if let Some(bi) = y.checked_sub(1) {
                    self.get(x + 1, bi)
                } else {
                    None
                };

                let below_before = if let Some(ci) = x.checked_sub(1) {
                    self.get(ci, y + 1)
                } else {
                    None
                };

                let below_next = self.get(x + 1, y + 1);

                let new = T::new_with(
                    cell.is_alive(),
                    above,
                    below,
                    next,
                    before,
                    above_before,
                    above_next,
                    below_before,
                    below_next,
                );

                board.insert(x, y, new);
            }
        }

        self.rows = board.rows;
    }

    fn is_empty(&self) -> bool {
        self.rows
            .iter()
            .filter(|r| r.iter().any(|c| c.is_alive()))
            .count()
            == 0
    }

    fn formatted(&self) -> String {
        format!("{}", self)
    }

    fn new_with(x: usize, y: usize, data: Vec<Location>) -> Self {
        let mut board = Self::new(x, y);

        for c in data.into_iter() {
            board.insert(c.x, c.y, T::new_alive())
        }

        board
    }

    fn new(x: usize, y: usize) -> Self {
        let x_vec: Vec<T> = vec![T::new_dead(); x];
        let y_vec: Vec<Vec<T>> = vec![x_vec.clone(); y];

        Board { rows: y_vec }
    }
}

#[cfg(test)]
mod test_board {
    use crate::{board::Board, board::BoardTrait, cell::Cell, cell::CellTrait, location::Location};

    #[test]
    fn test_board_get() {
        let board: Board<Cell> = BoardTrait::new_with(10, 10, vec![Location { x: 0, y: 0 }]);
        assert!(board.get(0, 0).is_some());

        let board: Board<Cell> = BoardTrait::new_with(10, 10, vec![Location { x: 11, y: 11 }]);
        assert!(board.get(11, 11).is_none());
    }

    #[test]
    fn test_board_insert() {
        let mut board: Board<Cell> = BoardTrait::new_with(10, 10, vec![]);
        board.insert(4, 5, CellTrait::new_alive());
        assert!(board.get(4, 5).is_some_and(|v| v.is_alive()));

        let mut board: Board<Cell> = BoardTrait::new_with(10, 10, vec![]);
        board.insert(11, 11, CellTrait::new_alive());
        assert!(board.is_empty());
    }

    #[test]
    fn test_board_is_empty() {
        let board: Board<Cell> = BoardTrait::new_with(10, 10, vec![]);
        assert!(board.is_empty());

        let board: Board<Cell> = BoardTrait::new_with(10, 10, vec![Location { x: 1, y: 1 }]);
        assert!(!board.is_empty());
    }

    /**
     * Each cell with one or no neighbors dies, as if by solitude.
     */
    #[test]
    fn test_board_step_rule1() {
        let mut board: Board<Cell> = BoardTrait::new_with(10, 10, vec![Location { x: 0, y: 0 }]);
        board.step();
        assert!(board.is_empty());
    }

    /**
     * Each cell with three neighbors becomes populated.
     * Each cell with two or three neighbors survives.
     */
    #[test]
    fn test_board_step_rule2() {
        let mut board: Board<Cell> = BoardTrait::new_with(
            10,
            10,
            vec![
                Location { x: 3, y: 2 },
                Location { x: 3, y: 3 },
                Location { x: 3, y: 4 },
            ],
        );

        board.step();

        let cells = vec![board.get(2, 3), board.get(3, 3), board.get(4, 3)];

        assert!(cells.into_iter().all(|c| c.is_some_and(|c| c.is_alive())));
    }

    /**
     * Each cell with four or more neighbors dies, as if by overpopulation.
     */
    #[test]
    fn test_board_step_rule3() {
        let mut board: Board<Cell> = BoardTrait::new_with(
            10,
            10,
            vec![
                Location { x: 3, y: 3 },
                Location { x: 4, y: 3 },
                Location { x: 5, y: 3 },
                Location { x: 3, y: 4 },
                Location { x: 4, y: 4 },
            ],
        );

        board.step();

        assert!(board.get(4, 4).is_some_and(|c| !c.is_alive()));
    }

    #[test]
    fn test_board_formatted() {
        let mut board: Board<Cell> = BoardTrait::new_with(10, 10, vec![Location { x: 0, y: 0 }]);
        let s1 = board.formatted();

        board.step();
        let s2 = board.formatted();

        assert_ne!(s1, s2)
    }
}
