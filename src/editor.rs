//use crossterm::terminal;
use std::env;
use crossterm::{
    //cursor,
    event::{
        //self, 
        Event, KeyCode, KeyModifiers},
};
//use std::io::{self, stdout, Write};

use crate::Document;
use crate::Row;
use crate::Terminal;



const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Default)]
pub struct Position {            
    pub x: usize,            
    pub y: usize,            
}
pub struct Editor {
    should_quit: bool,
    terminal: Terminal,
    offset: Position,
    document: Document,
    cursor_position : Position,
}

impl Editor {
    pub fn default() -> Self {
        let mut initial_status =
            String::from("HEWP: Ctrl-F = find | Ctrl-S = save | Esc = qwit");
        let args: Vec<String> = env::args().collect();
        let document = if let Some(file_name) = args.get(1) {
            let doc = Document::open(&file_name);
            if let Ok(doc) = doc {
                doc
            } else {
                initial_status = format!("EWWOR!!! Could not open file??! ＼＼(๑`^´๑)۶/怒／／ {}", file_name);
                Document::default()
            }
        } else {
            
            Document::default()
        };
        Self { 
            should_quit: false,
            terminal: Terminal::default().expect("Fail to create a terminal"),
            document,
            cursor_position: Position::default(),
            offset: Position::default(),
        }
    }
    pub fn run(&mut self) {
        
        loop {
            if let Err(error) = self.refresh_screen() {
                die(&error);
            }
            if let Err(error) = self.process_keypress() {
                die(&error);
            }
            if self.should_quit {
                break;
            }
            // Err(e) => die(&e),
        }
    }
    pub fn process_keypress(&mut self) -> Result<(), std::io::Error> {
        let keyevent = Terminal::read_key();
        
        if let Ok(Event::Key(event)) = keyevent {
            match (event.modifiers, event.code) {
                (KeyModifiers::CONTROL, KeyCode::Char('q')) => {
                    self.should_quit = true;
                },
                (_, KeyCode::Char(_c)) => {
                    //println!(" ({})", &c);
                },
                (_, KeyCode::Up)
                | (_, KeyCode::Down)
                | (_, KeyCode::Left)
                | (_, KeyCode::Right)
                | (_, KeyCode::PageUp)
                | (_, KeyCode::PageDown)
                | (_, KeyCode::End)
                | (_, KeyCode::Home) => self.move_cursor(event.code),
                _ => {
                    //println!("{:?}  ", &event);
                }
            }
        }
        self.scroll();
        Ok(())
    }
    fn scroll(&mut self) {            
        let Position { x, y } = self.cursor_position;            
        let width = self.terminal.size().width as usize;            
        let height = self.terminal.size().height as usize;            
        let mut offset = &mut self.offset;            
        if y < offset.y {            
            offset.y = y;            
        } else if y >= offset.y.saturating_add(height) {            
            offset.y = y.saturating_sub(height).saturating_add(1);            
        }            
        if x < offset.x {            
            offset.x = x;            
        } else if x >= offset.x.saturating_add(width) {            
            offset.x = x.saturating_sub(width).saturating_add(1);            
        }            
    }


    fn move_cursor(&mut self, key: KeyCode) {
        let terminal_height = self.terminal.size().height as usize;
        let Position { mut y, mut x } = self.cursor_position;
        // let height = self.document.len();
        // let mut width = if let Some(row) = self.document.row(y) {
        //     row.len()
        // } else {
        //     0
        // };
        let size = self.terminal.size();            
        let height = self.document.len();          
        let width = size.width.saturating_sub(1) as usize;
        match key {
            KeyCode::Up => y = y.saturating_sub(1),
            KeyCode::Down => {
                if y < height {
                    y = y.saturating_add(1);
                }
            }
            KeyCode::Left => {
                if x > 0 {
                    x -= 1;
                } else if y > 0 {
                    y -= 1;
                    // if let Some(row) = self.document.row(y) {
                    //     x = row.len();
                    // } else {
                    //     x = 0;
                    // }
                }
            }
            KeyCode::Right => {
                if x < width {
                    x += 1;
                } else if y < height {
                    y += 1;
                    x = 0;
                }
            }
            KeyCode::PageUp => {
                y = if y > terminal_height {
                    y.saturating_sub(terminal_height)
                } else {
                    0
                }
            }
            KeyCode::PageDown => {
                y = if y.saturating_add(terminal_height) < height {
                    y.saturating_add(terminal_height)
                } else {
                    height
                }
            }
            KeyCode::Home => x = 0,
            KeyCode::End => x = width,
            _ => ()
        }
        self.cursor_position = Position { x, y }
    }

    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        //print!("\x1b[2J");
        Terminal::cursor_hide();
        Terminal::cursor_position(&Position::default());
        //stdout().queue(cursor::MoveTo(1, 1)).ok();
        if self.should_quit {
            Terminal::clear_screen();
            println!("Goodbye.\r");
        } else {
            self.draw_rows();
            Terminal::cursor_position(&self.cursor_position);
        }
        Terminal::cursor_show();
        Terminal::flush()
        //io::stdout().flush()
    }
    fn draw_welcome_message(&self) {            
        let mut welcome_message = format!("Hecto editor -- version {}", VERSION);            
        let width = self.terminal.size().width as usize;            
        let len = welcome_message.len();            
        let padding = width.saturating_sub(len) / 2;            
        let spaces = " ".repeat(padding.saturating_sub(1));            
        welcome_message = format!("~{}{}", spaces, welcome_message);            
        welcome_message.truncate(width);            
        println!("{}\r", welcome_message);            
    }


    pub fn draw_row(&self, row: &Row) {
        let width = self.terminal.size().width as usize;
        let start = self.offset.x;
        let end = start + width;
        let row = row.render(start, end);
        println!("{}\r", row);
    }

    fn draw_rows(&self) {
        let height = self.terminal.size().height;

        for term_row in 0..height-1 {
            Terminal::clear_current_line();
            if let Some(row) = self.document.row(term_row as usize + self.offset.y) {
                self.draw_row(row);
            } else if self.document.is_empty() && term_row == height/3 {
                self.draw_welcome_message();
            }else {
                println!("~\r");
            }
        }
    }
}

fn die(e: &std::io::Error) {
    //stdout().execute(terminal::Clear(terminal::ClearType::All)).ok();
    Terminal::clear_screen();
    panic!("{}", *e);
}
