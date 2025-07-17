use std::{
    collections::VecDeque,
    io::{stdout,Write},
    thread,
    time::Duration,
};
use crossterm::{
    cursor,
    event::{self, Event, KeyCode},
    execute,
    terminal::{self, ClearType}
};
use rand::Rng;
mod game;
mod snake;
mod input;
mod ui;


const WIDTH : u16 = 20;
const HEIGHT: u16 = 20;

#[derive(Clone, Copy, PartialEq)]
enum Direction{
    Up,
    Down,
    Left,
    Right,
}
fn main() {
    let mut stdout = stdout();
    terminal::enable_raw_mode().unwrap();
    execute!(stdout, terminal::Clear(ClearType::All), cursor::Hide).unwrap();
    
}