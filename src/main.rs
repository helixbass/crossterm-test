use std::io::{stdout, Write};
use std::time::Duration;

use crossterm::{
    cursor,
    event::{self, Event, KeyCode},
    execute,
    style::Print,
    terminal::{
        disable_raw_mode, enable_raw_mode, size, EnterAlternateScreen, LeaveAlternateScreen,
    },
    QueueableCommand,
};

fn main() {
    let mut stdout = stdout();
    enable_raw_mode().unwrap();
    execute!(stdout, EnterAlternateScreen).unwrap();

    let (width, height) = size().unwrap();

    let mut cursor_position = Position { row: 4, column: 4 };

    let mut grid: Vec<String> = (0..height)
        .into_iter()
        .map(|_| "0".repeat(width.into()))
        .collect();

    stdout
        .queue(cursor::MoveTo(cursor_position.column, cursor_position.row))
        .unwrap();

    stdout.queue(cursor::SavePosition).unwrap();
    stdout.queue(cursor::Hide).unwrap();
    stdout.queue(cursor::MoveTo(0, 0)).unwrap();

    for row in &grid {
        stdout.queue(Print(&row)).unwrap();
    }

    stdout.queue(cursor::RestorePosition).unwrap();
    stdout.queue(cursor::Show).unwrap();

    stdout.flush().unwrap();

    loop {
        match event::read().unwrap() {
            Event::Key(key_event) if key_event.code == KeyCode::Char('q') => break,
            Event::Key(key_event) if key_event.code == KeyCode::Char('l') => {
                cursor_position.column += 1;
                stdout
                    .queue(cursor::MoveTo(cursor_position.column, cursor_position.row))
                    .unwrap();
                stdout.flush().unwrap();
            }
            _ => {}
        }
    }

    execute!(stdout, LeaveAlternateScreen).unwrap();
    disable_raw_mode().unwrap();
}

struct Position {
    pub row: u16,
    pub column: u16,
}
