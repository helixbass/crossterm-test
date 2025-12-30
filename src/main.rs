use std::io::{stdout, Write};
use std::time::Duration;

use crossterm::{
    cursor, execute,
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

    let row_str = "0".repeat(width.into());

    stdout.queue(cursor::MoveTo(0, 0)).unwrap();

    stdout.queue(cursor::SavePosition).unwrap();
    stdout.queue(cursor::Hide).unwrap();
    stdout.queue(cursor::MoveTo(0, 0)).unwrap();

    for row_num in 0..height {
        stdout.queue(Print(&row_str)).unwrap();
    }

    stdout.queue(cursor::RestorePosition).unwrap();
    stdout.queue(cursor::Show).unwrap();

    stdout.flush().unwrap();

    std::thread::sleep(Duration::from_secs(2));

    execute!(stdout, LeaveAlternateScreen).unwrap();
    disable_raw_mode().unwrap();
}
