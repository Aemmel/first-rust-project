use std::io;
use std::io::Read;

use std::collections::HashMap;
use std::rc::Weak;
use std::rc::Rc;
use std::cell::RefCell;

use tabcel::cell::{self, Cell};

fn main() {
    // let c1 = 0;
    // let c2 = 0;

    // table.insert(Coord::new(2, 2), op::val(4.0));
    // table.insert(Coord::new(1,1), op::add(&c1, &c2));
    // table.insert((3, 3), op::sum(vec![&c1, &c2, &c3]));
    // table.insert((4, 3), op::average(range((1, 1), (1, 8))));
}

/// 
/// table verwaltet alle zellen
/// mit Koordinate und Struktur welche Zelle enthält und Zellen worauf es sich bezieht
/// Zelle an sich weiß nichts von anderen Zellen, bekommt nur deren Werte
/// Werte werden von Struktur berechnet welche verwaltet auf welche Zellen es sich bezieht
/// Operation (für jetzt) kann nur ein oder zwei Argumente bekommen (Grundrechenarten und Mathematische funktionen wie Sinus)
/// ((Rest ist späteres Problem))
/// 
/// speichere intern dann nur pointer zu den anderen Zellen. Um upzudaten, speichert jede Zelle (über andere Struktur) ein Wert, wie häufigt
/// geupdatet wurde. Benutze dafür u64 (mit MAX: 18446744073709551615 .. sollte ausreichen)... oder auch nicht. mal gucken..

struct Coord {
    x: u32,
    y: u32,
}

impl Coord {
    fn convert_11_to_A1() { }
    fn convert_A1_to_11() { }
}

// fn main() {
//     // let input = read_input();

//     // println!("{:?}", input);

//     // let cell = parse_input(input);

//     // println!("{:?}", cell);

//     // #[derive(Debug)]
//     // struct Num {
//     //     num: i32,
//     //     references: Weak<RefCell<Num>>,
//     //     referenced_by: Weak<RefCell<Num>>,
//     // }
//     // impl Num {
//     //     fn add_one(&mut self) {
//     //         self.num += 1
//     //     }
//     // }

//     // // c2 references c1, c1 referenced by c2
//     // let c1 = Rc::new(RefCell::new(Num { num: 5, references: Weak::new(), referenced_by: Weak::new() }));
//     // let c2 = Rc::new(RefCell::new(Num { num: 5, references: Weak::new(), referenced_by: Weak::new() }));
//     // let c3 = Rc::new(RefCell::new(Num { num: 5, references: Weak::new(), referenced_by: Weak::new() }));

//     // c1.borrow_mut().referenced_by = Rc::downgrade(&c2);
//     // c2.borrow_mut().references = Rc::downgrade(&c1);
//     // c3.borrow_mut().references = Rc::downgrade(&c2);

//     // println!("c1: {:?}", c1);
//     // println!("c2: {:?}", c2);
//     // println!("{}", Weak::ptr_eq(&c2.borrow().references, &c3.borrow().references));

//     // return;

//     let mut table: HashMap<(u32, u32), Rc<RefCell<Cell>> > = HashMap::new();

//     // let c1 = Cell::new(1, 1);
//     // let c2 = Cell::new(2, 2);
//     // let mut c3 = Cell::new(3, 1);
    
//     table.insert((1, 1), Rc::new(RefCell::new(Cell::new())));
//     table.insert((2, 2), Rc::new(RefCell::new(Cell::new())));
//     table.insert((3, 1), Rc::new(RefCell::new(Cell::new())));
    
//     if let Some(c3) = table.get(&(3, 1)) {
//         if let Some(c1) = table.get(&(1,1)) {
//             if let Some(c2) = table.get(&(2,2)) {
//                 c1.borrow_mut().value = 2.;
//                 c2.borrow_mut().value = 3.;

//                 c1.borrow_mut().referenced_by = Rc::downgrade(c3);
//                 c2.borrow_mut().referenced_by = Rc::downgrade(c3);

//                 c3.borrow_mut().references = Rc::downgrade(c1);
//                 c3.borrow_mut().references = Rc::downgrade(c2);

//                 c3.borrow_mut().value = c1.borrow().value + c2.borrow().value;
//                 c3.borrow_mut().operation = cell::Operation::Add;
//             }
//         }
//     }

//     println!("{:#?}", table);
// }

// fn read_input() -> Vec<char> {
//     let mut input: Vec<char> = Vec::new();

//     // we only care about ASCII for now. Get's replaced later by non-raw input based format anyway
//     for byte in io::stdin().lock().bytes() {
//         let curr = byte.unwrap() as char;
//         if curr == '\n' {
//             break;
//         }
//         input.push(curr);
//     }

//     input
// }

// // TODO: add error handling
// fn parse_input(input: Vec<char>) -> Cell {
//     let mut x_in: Option<u32> = None;
//     let mut y_in: Option<u32> = None;

//     let mut buff = String::new();
//     for c in &input {
//         if *c == ',' {
//             if let None = x_in {
//                 x_in = Some(buff.trim().parse().expect("x not a number"));
//                 buff = String::new();
//             }
//         } else if *c == ':' {
//             if let None = y_in {

//                 y_in = Some(buff.trim().parse().expect("y not a number"));
//                 buff = String::new();
//             }
//         } else {
//             buff.push(*c);
//         } 
//     }

//     let value = buff.trim().parse::<f64>().expect("content must be a float (for now)");

//     let x = match x_in {
//         Some(n) => n,
//         None => 0,
//     };
//     let y = match y_in {
//         Some(n) => n,
//         None => 0,
//     };

//     Cell { x, y, value, operation: cell::Operation::None }
// }
