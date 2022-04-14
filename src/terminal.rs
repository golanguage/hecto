//use crate::Position;

use std::io::{self, stdout, Write};
use crossterm::{
    cursor,
    event::{self, 
        //read, 
        Event
    },
    //style::{Color, Colors, ResetColor, SetColors, SetForegroundColor},
    terminal, ExecutableCommand, QueueableCommand,
};
use crate::Position;

pub struct Size {
    pub width: u16,
    pub height: u16,
}

pub struct Terminal {
    size: Size,
}

impl Terminal {
    pub fn default() -> Result<Self, std::io::Error> {
        let size = crossterm::terminal::size().unwrap();
        crossterm::terminal::enable_raw_mode().ok();

        Ok(Self {
            size: Size {
                width: size.0,
                height: size.1.saturating_sub(2),
            },
        })
        
    }

    pub fn size(&self) -> &Size {
        &self.size
    }
    pub fn clear_screen()  {
        stdout()
            .execute(terminal::Clear(terminal::ClearType::All))
            .ok();
    }
    pub fn flush() -> Result<(), std::io::Error> {
        io::stdout().flush()
    }

    pub fn cursor_position(position: &Position) {
        let Position{mut x, mut y} = position;
        x = x.saturating_add(0);
        y = y.saturating_add(0);
        let x = x as u16;
        let y = y as u16;
        //eprintln!("x:{}y:{}", x, y);
        stdout().queue(cursor::MoveTo(x, y)).ok();
    }

    pub fn cursor_hide() {
        stdout().execute(cursor::DisableBlinking).ok();
    }

    pub fn cursor_show() {
        stdout().execute(cursor::DisableBlinking).ok();
    }

    pub fn clear_current_line(){
        stdout().execute(terminal::Clear(terminal::ClearType::CurrentLine)).ok();
    }

    pub fn read_key()-> Result<Event, std::io::Error> {
        loop {
            let keyevent = event::read();
            return keyevent
        }

    }
}
