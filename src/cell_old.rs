use std::fmt;
use std::rc::Weak;
use std::cell::RefCell;

//#[derive(Debug)]
// ? public x, y, content?
#[derive(Debug)]
pub struct Cell {
    // pub x: u32, //? does it need to know x or y value? I don't think so...
    // pub y: u32,
    pub value: f64,
    pub operation: Operation,
    pub references: Weak<RefCell<Cell>>, // TODO make Vec
    pub referenced_by: Weak<RefCell<Cell>>,
}

impl Cell {
    pub fn new() -> Cell {
        Cell {
            // x,
            // y,
            value: 0.0,
            operation: Operation::None,
            references: Weak::new(),
            referenced_by: Weak::new(),
        }
    }
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} [{:?}]", self.value, self.operation)
    }
}

#[derive(Debug)]
pub enum Operation {
    None,
    Add,
}

pub mod ops {
    use super::Cell;

    pub fn add(c1: &Cell, c2: &Cell) -> f64 {
        c1.value + c2.value
    }
}