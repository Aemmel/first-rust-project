use std::collections::HashMap;

use crate::cell::{self, Cell};

// #[derive(PartialEq, Eq, Hash)]
// pub struct Coord {
//     pub x: u32,
//     pub y: u32,
// }

// impl Coord {
//     pub fn new(x: u32, y: u32) -> Coord {
//         Coord { x, y }
//     }
// }
type Coord = (u32, u32);

pub struct Table {
    table: HashMap<Coord, Cell>,
    references: HashMap<Coord, Vec<Coord>>,
    referenced_by: HashMap<Coord, Vec<Coord>>,
}

impl Table {
    pub fn new() -> Table {
        Table {
            table: HashMap::new(),
            references: HashMap::new(),
            referenced_by: HashMap::new(),
        }
    }

    pub fn insert(&mut self, coord: Coord, operation: Operation) {
        let mut c = Cell::new();

        match operation {
            Operation::None(o) => {
                let co = self.match_operation_value(o);
                c.set_operation(cell::Operation::None(co));
            }
            Operation::Add(o1, o2) => {
                let co1 = self.match_operation_value(o1);
                let co2 = self.match_operation_value(o2);

                c.set_operation(cell::Operation::Add(co1, co2));
            }
            Operation::Sine(o) => {
                let co = self.match_operation_value(o);
                c.set_operation(cell::Operation::Sine(co));
            }
        }

        self.table.insert(coord, c);
    }

    fn match_operation_value(&self, op: OperationValue) -> cell::OperationValue {
        match op {
            OperationValue::Value(val) => cell::OperationValue::Value(val),
            OperationValue::Cell(coord) => {
                match self.table.get(&coord) {
                    Some(c) => cell::OperationValue::Cell(c.get_ptr_to_value()),
                    None => cell::OperationValue::Value(f64::NAN),
                }
            }
        }
    }

    pub fn iter(&self) -> std::collections::hash_map::Iter<Coord, Cell> {
        self.table.iter()
    }
}

//redundant. cell::Operation and table::Operation have same content
// only difference is, that OperationValue::Cell has different type.
// ? maybe with generic types? .. hmm..
pub enum Operation {
    None(OperationValue),
    Add(OperationValue, OperationValue),
    Sine(OperationValue),
}

pub enum OperationValue {
    Value(f64),
    Cell(Coord),
}
