use anyhow::Result;
use crossterm::{
    terminal::{Clear, ClearType},
    ExecutableCommand,
};
use std::io::stdout;
use tomatrix2::*;

pub fn main() -> Result<()> {
    stdout().execute(Clear(ClearType::All))?;

    let mut w = Window::default();
    loop {
        w.draw_loop()?;
        std::thread::sleep(std::time::Duration::new(0, 100000000));
    }
}
