extern crate failure;
extern crate termion;
extern crate tui;

#[allow(dead_code)]
mod util;

use std::io;
use termion::event::Key;
use termion::input::MouseTerminal;
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;
use tui::backend::TermionBackend;
use tui::layout::{Constraint, Direction, Layout};
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Block, Borders, Widget, SelectableList};
use tui::Terminal;

use util::event::{Event, Events};

struct App<'a> {
    menu_items:   Vec<&'a str>,
    selected_menu: Option<usize>
}

impl <'a> App<'a> {
    fn new() -> App<'a> {
        App {
            menu_items: vec![
                "Install",
                "Update",
                "Authors",
                "Exit"
            ],
            selected_menu: Some(0)
        }
    }
}

fn main() -> Result<(), failure::Error> {
    // Terminal initialization
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;

    // Setup event handlers
    let events = Events::new();

    // Application init
    let mut app = App::new();

    loop {
        let size = terminal.size()?;

        terminal.draw(|mut f| {

            Block::default().borders(Borders::ALL).render(&mut f, size);
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(4)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                .split(size);
            {
                let chunks = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                    .split(chunks[0]);

                SelectableList::default()
                    .block(Block::default().title("Menu"))
                    .select(app.selected_menu)
                    .items(&app.menu_items)
                    .highlight_style(Style::default().fg(Color::Red))
                    .render(&mut f, chunks[0]);

                Block::default()
                    .title("Options")
                    .title_style(
                        Style::default()
                            .fg(Color::White)
                            .bg(Color::Red)
                            .modifier(Modifier::Bold),
                    ).render(&mut f, chunks[1]);
            }
        })?;

        match events.next()? {
            Event::Input(key) => {
                match key {
                    Key::Char('q') => break,

                    Key::Down => {
                        app.selected_menu = if let Some(selected) = app.selected_menu {
                            if selected >= app.menu_items.len() - 1 {
                                Some(0)
                            } else {
                                Some(selected + 1)
                            }
                        } else {
                            Some(0)
                        }
                    },

                    Key::Up => {
                        app.selected_menu = if let Some(selected) = app.selected_menu {
                            if selected > 0 {
                                Some(selected - 1)
                            } else {
                                Some(app.menu_items.len() - 1)
                            }
                        } else {
                            Some(0)
                        }
                    },

                    _ => ()
                }
            },
            _ => ()
        }
    }
    Ok(())
}