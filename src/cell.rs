#[derive(Debug)]
pub struct Cell {
    pub value: f64, 
    pub operation: Operation,
}

impl Cell {
    fn compute(&mut self) {
        self.value = match self.operation {
            Operation::None => self.value,
            Operation::Add(n1, n2) => n1 + n2,
            Operation::Multiply(n1, n2) => n1 * n2,
            Operation::Sine(n) => n.sin(),
            Operation::Sqrt(n) => n.sqrt(),
        }
    }
}

#[derive(Debug)]
pub enum Operation {
    None,
    Add(f64, f64),
    Multiply(f64, f64),
    Sine(f64),
    Sqrt(f64),
}