use std::fs::read_to_string;
use std::io::{stdout, Read, Stdout, Write};
use std::{thread::sleep, time::Duration};
use termion::async_stdin;
use termion::cursor::{Goto, Hide, Show};
use termion::event::Key::Ctrl;
use termion::input::TermRead;
use termion::screen::IntoAlternateScreen;
use termion::{clear, raw::IntoRawMode};
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
use serde::Deserialize;
#[derive(Deserialize, Debug)]
struct Animation {
    frames: Vec<String>,
    multiplier: Option<usize>,
    x: i16,
    y: i16,
    repeat: Option<bool>,
    #[serde(skip)]
    frame: usize,
}
impl Animation {
    fn render(&mut self, mut stdout: &Stdout) -> Result<()> {
        match self.frames.get(self.frame / self.multiplier.unwrap_or(1)) {
            Some(frame) => {
                for (i, line) in frame.split("\n").enumerate() {
                    write!(stdout, "{}", Goto(self.x as u16, self.y as u16 + i as u16))?;
                    if let Some(true) = self.repeat {
                        println!(
                            "{}",
                            &(0..(termion::terminal_size()?.0 - self.x as u16) / line.len() as u16)
                                .map(|_| line)
                                .collect::<String>()
                        );
                    } else {
                        println!("{line}");
                    }
                }
                self.frame += 1;
            }
            None => self.frame = 0,
        };
        Ok(())
    }
}
fn main() -> Result<()> {
    let mut animations: Vec<Animation> = serde_json::from_str(&read_to_string("animations.json")?)?;
    let mut stdout = stdout().into_raw_mode()?.into_alternate_screen()?;
    let mut stdin = async_stdin().keys();
    write!(stdout, "{}{}", clear::All, Hide)?;
    loop {
        if let Some(Ok(k)) = stdin.next() {
            match k {
                Ctrl('p') => {
                    write!(stdout, "{Show}")?;
                    return Ok(());
                }
                _ => {}
            }
        }
        sleep(Duration::from_millis(17));
        write!(stdout, "{}{}", clear::All, Goto(1, 1))?;
        for n in animations.iter_mut() {
            n.render(&stdout)?;
        }
    }
}
