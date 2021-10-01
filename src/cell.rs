use std::cell::RefCell;
use std::rc::{Rc, Weak};

type StrongRef = Rc<RefCell<f64>>;
type WeakRef = Weak<RefCell<f64>>;

#[derive(Debug)]
pub enum CellValue {
    Num(f64),
    /// `Unity` type behaves as expected from mathematics
    /// i.e. for addition `Num` + `Unity` = `Num` (`Unity` behaves like a zero)
    /// for multiplication `Num` * `Unity`= `Num` (`Unity` behaves like a one)
    Unity,
}

impl std::ops::Add for CellValue {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        match self {
            CellValue::Num(v1) => match other {
                CellValue::Num(v2) => CellValue::Num(v1 + v2), // both Num
                CellValue::Unity => CellValue::Num(v1),        // one Num, one Unity
            },
            CellValue::Unity => match other {
                CellValue::Num(v2) => CellValue::Num(v2), // one Unity, one Num
                CellValue::Unity => CellValue::Unity,     // both Unity
            },
        }
    }
}

impl std::ops::Mul for CellValue {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        match self {
            CellValue::Num(v1) => match other {
                CellValue::Num(v2) => CellValue::Num(v1 * v2), // both Num
                CellValue::Unity => CellValue::Num(v1),        // one Num, one Unity
            },
            CellValue::Unity => match other {
                CellValue::Num(v2) => CellValue::Num(v2), // one Unity, one Num
                CellValue::Unity => CellValue::Unity,     // both Unity
            },
        }
    }
}

impl std::ops::Sub for CellValue {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        match self {
            CellValue::Num(v1) => match other {
                CellValue::Num(v2) => CellValue::Num(v1 - v2), // both Num
                CellValue::Unity => CellValue::Num(v1),        // one Num, one Unity
            },
            CellValue::Unity => match other {
                CellValue::Num(v2) => CellValue::Num(-v2), // one Unity, one Num
                CellValue::Unity => CellValue::Unity,      // both Unity
            },
        }
    }
}

impl std::ops::Div for CellValue {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        match self {
            CellValue::Num(v1) => match other {
                CellValue::Num(v2) => CellValue::Num(v1 / v2), // both Num
                CellValue::Unity => CellValue::Num(v1),        // one Num, one Unity
            },
            CellValue::Unity => match other {
                CellValue::Num(v2) => CellValue::Num(1. / v2), // one Unity, one Num
                CellValue::Unity => CellValue::Unity,          // both Unity
            },
        }
    }
}

impl PartialEq for CellValue {
    // we want Num(v1) == Num(v1)
    // Num(_) != Unity
    // Unity == Unity
    fn eq(&self, other: &Self) -> bool {
        if let CellValue::Num(v1) = *self {
            if let CellValue::Num(v2) = *other {
                return v1 == v2;
            }
        }
        if let CellValue::Unity = *self {
            if let CellValue::Unity = *other {
                return true;
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn ops_cellvalue_num_num() {
        assert_eq!(CellValue::Num(2.) + CellValue::Num(3.), CellValue::Num(5.));
        assert_eq!(CellValue::Num(2.) - CellValue::Num(3.), CellValue::Num(-1.));
        assert_eq!(CellValue::Num(2.) * CellValue::Num(3.), CellValue::Num(6.));
        assert_eq!(
            CellValue::Num(2.) / CellValue::Num(3.),
            CellValue::Num(2. / 3.)
        );
    }

    #[test]
    fn ops_cellvalue_num_unity() {
        assert_eq!(CellValue::Num(2.) + CellValue::Unity, CellValue::Num(2.));
        assert_eq!(CellValue::Num(2.) - CellValue::Unity, CellValue::Num(2.));
        assert_eq!(CellValue::Num(2.) * CellValue::Unity, CellValue::Num(2.));
        assert_eq!(CellValue::Num(2.) / CellValue::Unity, CellValue::Num(2.));
    }

    #[test]
    fn ops_cellvalue_unity_num() {
        assert_eq!(CellValue::Unity + CellValue::Num(3.), CellValue::Num(3.));
        assert_eq!(CellValue::Unity - CellValue::Num(3.), CellValue::Num(-3.));
        assert_eq!(CellValue::Unity * CellValue::Num(3.), CellValue::Num(3.));
        assert_eq!(
            CellValue::Unity / CellValue::Num(3.),
            CellValue::Num(1. / 3.)
        );
    }

    #[test]
    fn ops_cellvalue_unity_unity() {
        assert_eq!(CellValue::Unity + CellValue::Unity, CellValue::Unity);
        assert_eq!(CellValue::Unity - CellValue::Unity, CellValue::Unity);
        assert_eq!(CellValue::Unity * CellValue::Unity, CellValue::Unity);
        assert_eq!(CellValue::Unity / CellValue::Unity, CellValue::Unity);
    }

    #[test]
    fn cmp_cellvalue() {
        assert!(CellValue::Num(2.) == CellValue::Num(2.));
        assert!(CellValue::Num(2.) != CellValue::Num(3.));
        assert!(CellValue::Unity != CellValue::Num(2.));
        assert!(CellValue::Num(2.) != CellValue::Unity);
        assert!(CellValue::Unity == CellValue::Unity);
    }
}

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
                *c.upgrade().unwrap().borrow()
            }
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
