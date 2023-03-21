use std::{io::stdout, thread::sleep, time::Duration, env};
use crossterm::{Result, cursor, terminal, ExecutableCommand};

mod board;

fn main() -> Result<()> {
    let args = env::args().collect::<Vec<String>>();
    let size = if args.len() < 1 {
        args[1].parse::<u32>().unwrap()
    } else {
        50
    };
    let mut board = board::Board::new(size);

    let mut stdout = stdout();
    println!("{board}");

    stdout.execute(cursor::Show)?;

    loop {
        sleep(Duration::from_millis(100));
        stdout.execute(cursor::MoveUp(size as u16))?;
        stdout.execute(terminal::Clear(terminal::ClearType::FromCursorDown))?;
        board = board.calculate_board();
        println!("{board}");
    }


    Ok(())
}
