use std::io;
use std::io::Read;

use tabcel::table::{self, Table};

fn main() {
    let mut table = Table::new();

    table.insert(
        (1, 1),
        table::Operation::None(table::OperationValue::Value(1.0)),
    );
    table.insert(
        (1, 2),
        table::Operation::None(table::OperationValue::Value(2.0)),
    );
    table.insert(
        (2, 1),
        table::Operation::Add(
            table::OperationValue::Cell((1, 1)),
            table::OperationValue::Cell((1, 2)),
        ),
    );
    table.insert(
        (1, 3),
        table::Operation::None(table::OperationValue::Value(std::f64::consts::FRAC_PI_2)),
    );
    table.insert(
        (3, 1),
        table::Operation::Sine(table::OperationValue::Cell((1, 4))),
    );

    table.table.get(&(1, 1)).unwrap().update();
    table.table.get(&(1, 2)).unwrap().update();
    table.table.get(&(1, 3)).unwrap().update();
    table.table.get(&(2, 1)).unwrap().update();
    table.table.get(&(3, 1)).unwrap().update();

    for (coord, val) in table.iter() {
        println!("({}, {}): {}", coord.0, coord.1, val.get_value());
    }
}

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
