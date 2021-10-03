use std::{error::Error, io};
use termion::{event::Key, input::MouseTerminal, raw::IntoRawMode, screen::AlternateScreen};
use tui::{
    backend::TermionBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Gauge, Paragraph, Row, Table, TableState},
    Terminal,
};

use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use termion::input::TermRead;

pub enum Event<I> {
    Input(I),
    Tick,
}

/// A small event handler that wrap termion input and tick events. Each event
/// type is handled in its own thread and returned to a common `Receiver`
pub struct Events {
    rx: mpsc::Receiver<Event<Key>>,
    input_handle: thread::JoinHandle<()>,
    tick_handle: thread::JoinHandle<()>,
}

#[derive(Debug, Clone, Copy)]
pub struct Config {
    pub tick_rate: Duration,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            tick_rate: Duration::from_millis(250),
        }
    }
}

impl Events {
    pub fn new() -> Events {
        Events::with_config(Config::default())
    }

    pub fn with_config(config: Config) -> Events {
        let (tx, rx) = mpsc::channel();
        let input_handle = {
            let tx = tx.clone();
            thread::spawn(move || {
                let stdin = io::stdin();
                for evt in stdin.keys() {
                    if let Ok(key) = evt {
                        if let Err(err) = tx.send(Event::Input(key)) {
                            eprintln!("{}", err);
                            return;
                        }
                    }
                }
            })
        };
        let tick_handle = {
            thread::spawn(move || loop {
                if let Err(err) = tx.send(Event::Tick) {
                    eprintln!("{}", err);
                    break;
                }
                thread::sleep(config.tick_rate);
            })
        };
        Events {
            rx,
            input_handle,
            tick_handle,
        }
    }

    pub fn next(&self) -> Result<Event<Key>, mpsc::RecvError> {
        self.rx.recv()
    }
}

struct Selected {
    x: u32,
    y: u32,
}

impl Selected {
    fn new(x: u32, y: u32) -> Selected {
        Selected { x, y }
    }

    fn up(&mut self) {
        if self.y != 0 {
            self.y -= 1;
        }
    }

    fn down(&mut self) {
        self.y += 1;
    }

    fn left(&mut self) {
        if self.x != 0 {
            self.x -= 1;
        }
    }

    fn right(&mut self) {
        self.x += 1;
    }

    fn is_selected(&self, x: u32, y: u32) -> bool {
        if x == self.x && y == self.y {
            return true;
        }

        false
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // Terminal initialization
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let events = Events::with_config(Config {
        tick_rate: Duration::from_millis(100),
    });

    const CELL_WIDTH: u16 = 15;
    const CELL_HEIGHT: u16 = 3;

    let mut selected = Selected::new(5, 5);

    loop {
        terminal.draw(|f| {
            // header
            let mut header_rects = vec![];
            let mut header_texts = vec![];
            for x in 1..50 {
                if (CELL_WIDTH - 1) * x + CELL_WIDTH < f.size().width {
                    header_rects.push(Rect::new(
                        (CELL_WIDTH - 1) * x,
                        CELL_HEIGHT - 1,
                        CELL_WIDTH,
                        CELL_HEIGHT,
                    ));
                    header_texts.push(
                        Paragraph::new(vec![Spans::from(Span::from(format!("Col {}", x)))])
                            .style(
                                Style::default()
                                    .fg(Color::White)
                                    .add_modifier(Modifier::BOLD),
                            )
                            .block(
                                Block::default().borders(Borders::ALL).border_style(
                                    Style::default()
                                        .fg(Color::White)
                                        .add_modifier(Modifier::BOLD),
                                ),
                            ),
                    );
                }
            }
            for c in &header_rects {
                f.render_widget(header_texts.remove(0), *c);
            }

            // row numbering
            let mut rows_rects = vec![];
            let mut rows_texts = vec![];
            for y in 2..50 {
                if (CELL_HEIGHT - 1) * y + CELL_HEIGHT < f.size().height - 2 * CELL_HEIGHT {
                    rows_rects.push(Rect::new(0, (CELL_HEIGHT - 1) * y, CELL_WIDTH, CELL_HEIGHT));
                    rows_texts.push(
                        Paragraph::new(vec![Spans::from(Span::from(format!("Row {}", y - 1)))])
                            .style(
                                Style::default()
                                    .fg(Color::White)
                                    .add_modifier(Modifier::BOLD),
                            )
                            .block(
                                Block::default().borders(Borders::ALL).border_style(
                                    Style::default()
                                        .fg(Color::White)
                                        .add_modifier(Modifier::BOLD),
                                ),
                            ),
                    );
                }
            }
            for c in &rows_rects {
                f.render_widget(rows_texts.remove(0), *c);
            }

            // main body with cells
            let mut rects = vec![];
            let mut texts = vec![];
            // TODO: something like `for y in table_size` or so, so that we can omit the if condition
            for y in 2..50 {
                for x in 1..50 {
                    let mut s = f.size();
                    s.height -= 2 * CELL_HEIGHT;
                    // HEIGHT-1 to remove standard margin
                    if (CELL_HEIGHT - 1) * y + CELL_HEIGHT < s.height
                        && (CELL_WIDTH - 1) * x + CELL_WIDTH < s.width
                    {
                        rects.push(Rect::new(
                            (CELL_WIDTH - 1) * x,
                            (CELL_HEIGHT - 1) * y,
                            CELL_WIDTH,
                            CELL_HEIGHT,
                        ));
                        texts.push(
                            Paragraph::new(vec![Spans::from(Span::from(format!("{}, {}", x, y)))])
                                .block(Block::default().borders(Borders::ALL)),
                        );

                        if selected.is_selected(x as u32, y as u32) {
                            let temp = texts
                                .remove(texts.len() - 1)
                                .block(
                                    Block::default()
                                        .borders(Borders::ALL)
                                        .border_style(Style::default().fg(Color::Cyan)),
                                )
                                .style(Style::default().fg(Color::Cyan));
                            texts.push(temp);
                        }
                    }
                }
            }
            for i in 0..rects.len() {
                f.render_widget(texts.remove(0), rects[i]);
            }
        })?;

        match events.next()? {
            Event::Input(input) => match input {
                Key::Char('q') => break,
                Key::Up | Key::Char('w') => selected.up(),
                Key::Down | Key::Char('s') => selected.down(),
                Key::Left | Key::Char('a') => selected.left(),
                Key::Right | Key::Char('d') => selected.right(),
                _ => (),
            },
            Event::Tick => (),
        }
    }

    Ok(())
}

// fn main() {
//     let mut table = Table::new();

//     table.insert(
//         (1, 1),
//         table::Operation::None(table::OperationValue::Value(1.0)),
//     );
//     table.insert(
//         (1, 2),
//         table::Operation::None(table::OperationValue::Value(2.0)),
//     );
//     table.insert(
//         (2, 1),
//         table::Operation::Add(
//             table::OperationValue::Cell((1, 1)),
//             table::OperationValue::Cell((1, 2)),
//         ),
//     );
//     table.insert(
//         (1, 3),
//         table::Operation::None(table::OperationValue::Value(std::f64::consts::FRAC_PI_2)),
//     );
//     table.insert(
//         (3, 1),
//         table::Operation::Sine(table::OperationValue::Cell((1, 4))),
//     );
//     println!("-------------------------");
//     table.insert(
//         (1, 1),
//         table::Operation::None(table::OperationValue::Value(-1.0)),
//     );
//     table.insert(
//         (1, 4),
//         table::Operation::None(table::OperationValue::Value(std::f64::consts::FRAC_PI_4)),
//     );

//     for (coord, val) in table.iter() {
//         println!("({}, {}): {}", coord.0, coord.1, val.get_value());
//     }
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
