use std::cell::RefCell;
use std::rc::{Rc, Weak};

type StrongRef = Rc<RefCell<f64>>;
type WeakRef = Weak<RefCell<f64>>;

#[derive(Debug)]
pub struct Cell {
    value: StrongRef,
    operation: Operation,
}

impl Cell {
    pub fn new() -> Cell {
        Cell {
            value: Rc::new(RefCell::new(0.0)),
            operation: Operation::None(OperationValue::Value(0.0)),
        }
    }

    /// Sets value directly to `val`
    /// since setting value directly would violate any Operation other than None,
    /// set `operation` to `Operation::None`
    pub fn set_value(&mut self, val: f64) {
        // *self.value.borrow_mut() = val;
        self.operation = Operation::None(OperationValue::Value(val));
        self.update();
    }

    /// Sets `operation`
    /// `op` needs to be initialized with references already
    pub fn set_operation(&mut self, op: Operation) {
        self.operation = op;
    }

    /// Get `value` as `f64`
    pub fn get_value(&self) -> f64 {
        *self.value.borrow()
    }

    /// Return a weak pointer to `value` to initialize Operation
    pub fn get_ptr_to_value(&self) -> WeakRef {
        Rc::downgrade(&self.value)
    }

    // TODO: make error handling. so that unwrap of Weak pointer can fail, if that cell is empty. This would result in the value being ERROR or so
    pub fn update(&self) {
        *self.value.borrow_mut() = match &self.operation {
            Operation::None(o) => self.match_operation_value(o),
            Operation::Add(o1, o2) => {
                let n1 = self.match_operation_value(o1);
                let n2 = self.match_operation_value(o2);

                n1 + n2
            }
            Operation::Sine(o) => self.match_operation_value(o).sin(),
        }
    }

    fn match_operation_value(&self, op: &OperationValue) -> f64 {
        match op {
            OperationValue::Value(val) => *val,
            OperationValue::Cell(c) => {
                // indicate non-existent cell by NaN
                // has exact behaviour I want and does not need a refactor
                match c.upgrade() {
                    Some(p) => *p.borrow(),
                    None => f64::NAN,
                }
            },
        }
    }
}

#[derive(Debug)]
pub enum Operation {
    None(OperationValue),
    Add(OperationValue, OperationValue),
    Sine(OperationValue),
}

#[derive(Debug)]
pub enum OperationValue {
    Value(f64),
    Cell(WeakRef),
}
