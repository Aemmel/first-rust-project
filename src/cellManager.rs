use crate::cell::Cell;
use std::rc::Weak;

#[derive(Debug)]
struct CellManager {
    pub cell: Cell,
    pub referenced_cells: Vec<Coord>,
}

#[derive(Debug)]
struct Coord {
    x: u32,
    y: u32,
}

impl Coord {
    fn convert_11_to_A1() { }
    fn convert_A1_to_11() { }
}