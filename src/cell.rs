use std::fmt::Display;

pub trait CellTrait {
    fn invert(&mut self);
    fn is_alive(&self) -> bool;
    fn formatted(&self) -> String;
    fn new_alive() -> Self;
    fn new_dead() -> Self;
    fn new_with(
        is_alive: bool,
        above: Option<&Self>,
        below: Option<&Self>,
        next: Option<&Self>,
        before: Option<&Self>,
        above_before: Option<&Self>,
        above_next: Option<&Self>,
        below_before: Option<&Self>,
        below_next: Option<&Self>,
    ) -> Self;
}

#[derive(Clone, Debug)]
pub struct Cell {
    is_alive: bool,
}

impl Default for Cell {
    fn default() -> Self {
        Self::new_dead()
    }
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_alive {
            write!(f, "{}", "#")
        } else {
            write!(f, "{}", " ")
        }
    }
}

impl CellTrait for Cell {
    fn invert(&mut self) {
        self.is_alive = !self.is_alive;
    }

    fn is_alive(&self) -> bool {
        self.is_alive
    }

    fn formatted(&self) -> String {
        format!("{}", self)
    }

    fn new_alive() -> Self {
        Cell { is_alive: true }
    }

    fn new_dead() -> Self {
        Cell { is_alive: false }
    }

    fn new_with(
        mut is_alive: bool,
        above: Option<&Self>,
        below: Option<&Self>,
        next: Option<&Self>,
        before: Option<&Self>,
        above_before: Option<&Self>,
        above_next: Option<&Self>,
        below_before: Option<&Self>,
        below_next: Option<&Self>,
    ) -> Self {
        let mut alive: u8 = 0;

        if above.is_some_and(|c| c.is_alive()) {
            alive += 1;
        }
        if below.is_some_and(|c| c.is_alive()) {
            alive += 1;
        }
        if before.is_some_and(|c| c.is_alive()) {
            alive += 1;
        }
        if next.is_some_and(|c| c.is_alive()) {
            alive += 1;
        }
        if above_before.is_some_and(|c| c.is_alive()) {
            alive += 1;
        }
        if above_next.is_some_and(|c| c.is_alive()) {
            alive += 1;
        }
        if below_before.is_some_and(|c| c.is_alive()) {
            alive += 1;
        }

        if below_next.is_some_and(|c| c.is_alive()) {
            alive += 1;
        }

        if is_alive {
            is_alive = 2 == alive || 3 == alive
        } else {
            is_alive = alive == 3;
        }

        if is_alive {
            CellTrait::new_alive()
        } else {
            CellTrait::new_dead()
        }
    }
}

#[cfg(test)]
mod test_cell {
    use crate::{cell::Cell, cell::CellTrait};

    #[test]
    fn test_cell_invert() {
        let mut cell: Cell = CellTrait::new_alive();
        let is_alive = cell.is_alive();
        cell.invert();
        assert_ne!(is_alive, cell.is_alive());
    }

    #[test]
    fn test_cell_new_dead() {
        let cell: Cell = CellTrait::new_dead();
        assert_eq!(false, cell.is_alive());
    }

    #[test]
    fn test_cell_new_alive() {
        let cell: Cell = CellTrait::new_alive();
        assert_eq!(true, cell.is_alive());
    }

    #[test]
    fn test_cell_is_alive() {
        let cell: Cell = CellTrait::new_alive();
        let is_alive = cell.is_alive();
        assert_eq!(is_alive, cell.is_alive());
    }

    #[test]
    fn test_cell_formatted() {
        let cell0: Cell = CellTrait::new_alive();
        let cell1: Cell = CellTrait::new_dead();
        assert_ne!(cell0.formatted(), cell1.formatted());
    }

    #[test]
    fn test_cell_new_with() {
        //3 alive and self not active
        let binding1 = &CellTrait::new_alive();
        let above: Option<&Cell> = Some(binding1);
        let binding2 = &CellTrait::new_alive();
        let below: Option<&Cell> = Some(binding2);
        let binding3 = &CellTrait::new_alive();
        let next: Option<&Cell> = Some(binding3);
        let binding4 = &CellTrait::new_dead();
        let before: Option<&Cell> = Some(binding4);
        let binding5 = &CellTrait::new_dead();
        let above_before: Option<&Cell> = Some(binding5);
        let binding6 = &CellTrait::new_dead();
        let above_next: Option<&Cell> = Some(binding6);
        let binding7 = &CellTrait::new_dead();
        let below_before: Option<&Cell> = Some(binding7);
        let binding8 = &CellTrait::new_dead();
        let below_next: Option<&Cell> = Some(binding8);

        let cell: Cell = CellTrait::new_with(
            false,
            above,
            below,
            next,
            before,
            above_before,
            above_next,
            below_before,
            below_next,
        );

        assert!(cell.is_alive());

        //empty
        let binding1 = &CellTrait::new_dead();
        let above: Option<&Cell> = Some(binding1);
        let binding2 = &CellTrait::new_dead();
        let below: Option<&Cell> = Some(binding2);
        let binding3 = &CellTrait::new_dead();
        let next: Option<&Cell> = Some(binding3);
        let binding4 = &CellTrait::new_dead();
        let before: Option<&Cell> = Some(binding4);
        let binding5 = &CellTrait::new_dead();
        let above_before: Option<&Cell> = Some(binding5);
        let binding6 = &CellTrait::new_dead();
        let above_next: Option<&Cell> = Some(binding6);
        let binding7 = &CellTrait::new_dead();
        let below_before: Option<&Cell> = Some(binding7);
        let binding8 = &CellTrait::new_dead();
        let below_next: Option<&Cell> = Some(binding8);

        let cell: Cell = CellTrait::new_with(
            true,
            above,
            below,
            next,
            before,
            above_before,
            above_next,
            below_before,
            below_next,
        );

        assert!(!cell.is_alive());

        //2 alive
        let binding1 = &CellTrait::new_alive();
        let above: Option<&Cell> = Some(binding1);
        let binding2 = &CellTrait::new_alive();
        let below: Option<&Cell> = Some(binding2);
        let binding3 = &CellTrait::new_dead();
        let next: Option<&Cell> = Some(binding3);
        let binding4 = &CellTrait::new_dead();
        let before: Option<&Cell> = Some(binding4);
        let binding5 = &CellTrait::new_dead();
        let above_before: Option<&Cell> = Some(binding5);
        let binding6 = &CellTrait::new_dead();
        let above_next: Option<&Cell> = Some(binding6);
        let binding7 = &CellTrait::new_dead();
        let below_before: Option<&Cell> = Some(binding7);
        let binding8 = &CellTrait::new_dead();
        let below_next: Option<&Cell> = Some(binding8);

        let cell: Cell = CellTrait::new_with(
            true,
            above,
            below,
            next,
            before,
            above_before,
            above_next,
            below_before,
            below_next,
        );

        assert!(cell.is_alive());

        //3 alive
        let binding1 = &CellTrait::new_alive();
        let above: Option<&Cell> = Some(binding1);
        let binding2 = &CellTrait::new_alive();
        let below: Option<&Cell> = Some(binding2);
        let binding3 = &CellTrait::new_alive();
        let next: Option<&Cell> = Some(binding3);
        let binding4 = &CellTrait::new_dead();
        let before: Option<&Cell> = Some(binding4);
        let binding5 = &CellTrait::new_dead();
        let above_before: Option<&Cell> = Some(binding5);
        let binding6 = &CellTrait::new_dead();
        let above_next: Option<&Cell> = Some(binding6);
        let binding7 = &CellTrait::new_dead();
        let below_before: Option<&Cell> = Some(binding7);
        let binding8 = &CellTrait::new_dead();
        let below_next: Option<&Cell> = Some(binding8);

        let cell: Cell = CellTrait::new_with(
            true,
            above,
            below,
            next,
            before,
            above_before,
            above_next,
            below_before,
            below_next,
        );

        assert!(cell.is_alive());

        //4 alive
        let binding1 = &CellTrait::new_alive();
        let above: Option<&Cell> = Some(binding1);
        let binding2 = &CellTrait::new_alive();
        let below: Option<&Cell> = Some(binding2);
        let binding3 = &CellTrait::new_alive();
        let next: Option<&Cell> = Some(binding3);
        let binding4 = &CellTrait::new_alive();
        let before: Option<&Cell> = Some(binding4);
        let binding5 = &CellTrait::new_dead();
        let above_before: Option<&Cell> = Some(binding5);
        let binding6 = &CellTrait::new_dead();
        let above_next: Option<&Cell> = Some(binding6);
        let binding7 = &CellTrait::new_dead();
        let below_before: Option<&Cell> = Some(binding7);
        let binding8 = &CellTrait::new_dead();
        let below_next: Option<&Cell> = Some(binding8);

        let cell: Cell = CellTrait::new_with(
            true,
            above,
            below,
            next,
            before,
            above_before,
            above_next,
            below_before,
            below_next,
        );

        assert!(!cell.is_alive())
    }
}
