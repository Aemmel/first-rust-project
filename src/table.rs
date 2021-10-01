use std::collections::HashMap;

use crate::cell::{self, Cell};

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
        let cell_op: cell::Operation;
        
        // remove potential old references, since insert always overwrites a (potentially empty) cell
        self.update_references_remove(&coord);
        
        match operation {
            Operation::None(o) => {
                let co = self.match_operation_value(o, &coord);
                cell_op = cell::Operation::None(co);
            }
            Operation::Add(o1, o2) => {
                let co1 = self.match_operation_value(o1, &coord);
                let co2 = self.match_operation_value(o2, &coord);
                
                cell_op = cell::Operation::Add(co1, co2)
            }
            Operation::Sine(o) => {
                let co = self.match_operation_value(o, &coord);
                cell_op = cell::Operation::Sine(co);
            }
        }
        
        // self.table.insert(coord, c);
        self.table.entry(coord).or_insert_with(Cell::new).set_operation(cell_op);
        
        self.update_cell(&coord);
    }
    
    pub fn iter(&self) -> std::collections::hash_map::Iter<Coord, Cell> {
        self.table.iter()
    }

    pub fn get_value_of_cell(&self, coord: &Coord) -> Option<f64> {
        self.table.get(coord).map(|c| c.get_value())
    }

    fn update_cell(&mut self, to_update: &Coord) {
        let mut updater = CellUpdater{ to_update: vec![] };
        
        updater.gather_cells_to_update(to_update, &self.referenced_by);
        updater.remove_duplicates();
        
        // important to go from front to back
        for i in &updater.to_update {
            println!("{:?}", *i);
            if let Some(c) = self.table.get(i) {
                c.update();
            }
        }
    }

    fn match_operation_value(
        &mut self,
        op: OperationValue,
        new_cell: &Coord,
    ) -> cell::OperationValue {
        match op {
            OperationValue::Value(val) => cell::OperationValue::Value(val),
            OperationValue::Cell(coord) => {
                // ensure cell is valid. Default value is 0.0
                if !self.table.contains_key(&coord) {
                    self.insert(coord, Operation::None(OperationValue::Value(0.0)));
                }

                self.update_references_insert(new_cell, &coord);
                cell::OperationValue::Cell(self.table.get(&coord).unwrap().get_ptr_to_value())
            }
        }
    }

    /// remove cell `to_update` and update references
    fn update_references_remove(&mut self, to_update: &Coord) {
        // Let's say we have (where (n) is cell n)
        // (1) contain =(2)+1
        // and (2) = 5
        // and (3)=(1)+1
        // then:    references(1) = [(2)],  referenced_by(1) = [(3)]
        //          references(2) = [ ],    referenced_by(2) = [(1)]
        //          references(3) = [(1)],  referenced_by(3) = [ ]
        //
        // so to remove e.g. (1), (1) contains nothing, then references(1) needs to be completely deleted
        // and everywhere where (1) appears in referenced_by (here for (2)) needs to remove the entry (1)

        if let Some(refs) = self.references.get(to_update) {
            for c in refs {
                // self.referenced_by.remove(c);
                if let Some(index) = self
                    .referenced_by
                    .get(c)
                    .unwrap()
                    .iter()
                    .position(|coord| *coord == *c)
                {
                    self.referenced_by.get_mut(c).unwrap().swap_remove(index);
                }
            }
        }

        self.references.remove(to_update);
    }

    /// insert cell `to_update` which references cell `references` and update the table
    fn update_references_insert(&mut self, to_update: &Coord, references: &Coord) {
        self.references
            .entry(*to_update)
            .or_insert_with(Vec::new)
            .push(*references);

        for c in self.references.get(to_update).unwrap() {
            self.referenced_by
                .entry(*c)
                .or_insert_with(Vec::new)
                .push(*to_update);
        }
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

struct CellUpdater {
    to_update: Vec<Coord>,
}

impl CellUpdater {
    fn gather_cells_to_update(
        &mut self,
        cell_to_update: &Coord,
        referenced_by: &HashMap<Coord, Vec<Coord>>,
    ) {
        self.to_update.push(*cell_to_update);
        
        if let Some(refs) = referenced_by.get(cell_to_update) {
            for c in refs {
                self.gather_cells_to_update(c, referenced_by);
            }
        }
    }

    fn remove_duplicates(&mut self) {
        // we need to remove duplicates in the right order
        // if a cell appears in two places in the queue, then only the LAST appearance should be saved
        // the ones before need to be deleted

        // slow way
        let mut to_remove = Vec::<usize>::new();
        for i in (0..self.to_update.len()).rev() {
            let mut curr_index = i;

            while let Some(new_pos) = self.to_update[0..curr_index].iter().rev().position(|cell| *cell == self.to_update[i]) {
                // new_pos now index of reversed slice to_update[0..curr_index]
                let new_pos = curr_index - 1 - new_pos;

                if to_remove.iter().position(|&j| j == new_pos).is_none() {
                    to_remove.push(new_pos);
                }
                    curr_index = new_pos;
            }
        }

        for i in &to_remove {
            self.to_update.remove(*i);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_duplicates() {
        let mut c = CellUpdater{ to_update: vec![(1, 1), (4, 4), (2, 2), (4, 4), (3, 3), (4, 4), (5, 5)] };
        c.remove_duplicates();

        assert_eq!(c.to_update, vec![(1, 1), (2, 2), (3, 3), (4, 4), (5, 5)])
    }
}