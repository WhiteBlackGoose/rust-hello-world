use crossterm::{
    ExecutableCommand, QueueableCommand,
    terminal, cursor, style::{self, Stylize}, Result
};
use std::io::{stdout, Write};
use console::Term;
use std::cmp;

fn clamp<T: Ord>(input: T, min: T, max: T) -> T {
    cmp::min(cmp::max(input, min), max)
}

fn main() -> Result<()> {
  let mut stdout = stdout();

  stdout.execute(terminal::Clear(terminal::ClearType::All))?;
  let (width, height) = {
      let (w, h) = term_size::dimensions().unwrap();
      (w as i16, h as i16)
  };
  let (mut x, mut y) = (0i16, 0i16);
  loop {
    stdout
        .queue(cursor::MoveTo(x as u16, y as u16))?
        .queue(style::PrintStyledContent("█".magenta()))?;
    stdout.flush()?;
    match Term::stdout().read_char() {
        Ok('a') => x = clamp(x - 1, 0, width - 1),
        Ok('f') => x = clamp(x + 1, 0, width - 1),
        Ok('d') => y = clamp(y + 1, 0, height - 1),
        Ok('s') => y = clamp(y - 1, 0, height - 1),
        _ => {}
    }
  }
}
