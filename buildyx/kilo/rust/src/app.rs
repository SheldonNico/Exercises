use std::{io::{self, Read, Write}, thread};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use termion::{
    raw::IntoRawMode,
    input::TermRead,
    event::{Event, Key, MouseEvent},
};
use tui::{
    backend::{Backend, TermionBackend},
    terminal::Frame,
    Terminal,
    widgets::*,
    style::*,
    layout::{Layout, Constraint, Direction},
};
use crossbeam::channel::{self, select};
use log::*;

pub struct Kilo {
    pub status: Status,

    pub cx: u16,
    pub cy: u16,
    pub wait_key: String,
    pub wait_last: u64,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Status {
    Inited,
    Wait,
    Quit
}

impl Kilo {
    pub fn new() -> Self {
        Self {
            status: Status::Inited,

            cx: 0, cy: 0,
            wait_key: "".into(),
            wait_last: 0,
        }
    }

    pub fn is_quit(&self) -> bool {
        self.status == Status::Quit
    }

    pub fn refresh<B: Backend>(&mut self, f: &mut Frame<B>) {
        let tui::layout::Rect { width, height, .. } = f.size();
        if height <= 2 { return; }

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(0)
            .constraints([
                Constraint::Length(height-2),
                Constraint::Length(1),
                Constraint::Length(1),
            ].as_ref())
            .split(f.size());


        let mut items = vec![];
        let middleline = {
            let mut line = "~".to_string();
            let msg = "Welcome to kilo 1.0.0";
            let pad = (width as usize - msg.len() - 1) / 2;
            for _ in 0..pad { line.push_str(" "); }
            line.push_str(msg);
            line
        };
        for idx in 0..chunks[0].height {
            if idx == chunks[0].height / 3 {
                items.push(ListItem::new(middleline.as_ref()));
            } else {
                items.push(ListItem::new("~"));
            }
        }
        f.render_widget(List::new(items), chunks[0]);

        f.render_widget(
            Paragraph::new("").style(Style::default().bg(Color::White)),
            chunks[1]
        );

        self.cx = self.cx.min(width-1);
        self.cy = self.cy.min(height-1-2);
        f.set_cursor(self.cx, self.cy);

        if self.status == Status::Wait {
            let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
            if now - self.wait_last > 5 {
                info!("time out key.");
                self.status = Status::Inited;
            }
        }
    }

    pub fn on_wait(&mut self, key: String) -> Option<()> {
        match key.as_ref() {
            "gg" => {
                self.cy = 0;
                Some(())
            },
            _ => None,
        }
    }

    pub fn on_event(&mut self, event: Event) {
        match event {
            Event::Key(Key::Ctrl('q')) => self.status = Status::Quit,
            Event::Key(Key::Char('G')) => self.cy = 9999,
            Event::Key(Key::Char('j')) | Event::Key(Key::Down) => {
                self.cy += 1;
            },
            Event::Key(Key::Char('k')) | Event::Key(Key::Up)   => {
                if self.cy > 0 {
                    self.cy -= 1;
                }
            },
            Event::Key(Key::Char('l')) | Event::Key(Key::Right) => {
                self.cx += 1;
            }
            Event::Key(Key::Char('h')) | Event::Key(Key::Left) => {
                if self.cx > 0 {
                    self.cx -= 1;
                }
            }
            Event::Key(Key::Char(c)) => {
                match self.status {
                    Status::Inited => {
                        self.status = Status::Wait;
                        self.wait_key.clear();
                        self.wait_key.push(c);
                        self.wait_last = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
                    },
                    Status::Wait   => {
                        self.wait_key.push(c);
                        self.wait_last = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
                        if let Some(()) = self.on_wait(self.wait_key.to_string()) {
                            self.status = Status::Inited;
                        }
                    }
                    _ => {}
                }
            }
            _ => { }
        }
    }

    pub fn run(&mut self) -> Result<(), io::Error> {
        let mut stdout = std::io::stdout().into_raw_mode()?;
        write!(stdout, "{}", termion::clear::All)?;
        let backend = TermionBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        let (tx, rx_term) = channel::bounded(1024);
        let _r = thread::spawn(move || {
            let stdin = io::stdin();
            for evt in stdin.events() {
                if let Ok(event) = evt {
                    if let Err(e) = tx.send(event) {
                        warn!("fail to send term event: {}", e);
                    }
                }
            }
        });

        let rx_tick = channel::tick(std::time::Duration::from_millis(250));

        loop {
            if let Err(e) = terminal.draw(|f| self.refresh(f)) {
                warn!("fail to draw: {}", e);
            }
            if self.is_quit() { break; }
            select! {
                recv(rx_tick) -> _tick => { },
                recv(rx_term) -> event => {
                    if let Ok(event) = event {
                        self.on_event(event);
                    }
                }
            }
        }

        write!(terminal.backend_mut(), "{}", termion::cursor::Goto(1, 1))?;
        // write!(terminal.backend_mut(), "{}{}", termion::clear::All, termion::cursor::Goto(0, 0))?;

        Ok(())
    }
}
