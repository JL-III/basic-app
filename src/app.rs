use ratatui::{buffer::Buffer, layout::Rect, widgets::{block::Title, Widget}, Frame};
use database::{get_database_data, initialize_database, seed_database};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};

use color_eyre::{
  eyre::{bail, Ok, WrapErr},
  Result,
};

use ratatui::{
  prelude::*,
  symbols::border,
  widgets::{block::*, *},
};

use crate::{database, tui};


#[derive(Debug, Default)]
pub struct App {
    counter: u8,
    exit: bool,
}

impl App {
  /// runs the application's main loop until the user quits
  pub fn run(&mut self, terminal: &mut tui::Tui) -> Result<()> {
      while !self.exit {
          terminal.draw(|frame| self.render_frame(frame))?;
          self.handle_events().wrap_err("handle events failed")?;
      }
      Ok(())
  }

  fn render_frame(&self, frame: &mut Frame) {
      frame.render_widget(self, frame.size());
  }

  fn handle_events(&mut self) -> Result<()> {
      match event::read()? {
          Event::Key(key_event) if key_event.kind == KeyEventKind::Press => self
              .handle_key_event(key_event)
              .wrap_err_with(|| format!("handling key event failed\n{key_event:#?}")),
          _ => Ok(()),
      }
  }

  fn handle_key_event(&mut self, key_event: KeyEvent) -> Result<()> {
      match key_event.code {
          KeyCode::Char('q') => {
              println!("pressed 'q'");
              self.exit()
          }
          KeyCode::Char('i') => {
              println!("pressed 'i'");
              initialize_database()?;
          }
          KeyCode::Char('g') => {
              print!("pressed 'g'");
              get_database_data()?;
          }
          KeyCode::Char('s') => {
              println!("pressed 's'");
              seed_database()?;
          }
          KeyCode::Left => self.decrement_counter()?,
          KeyCode::Right => self.increment_counter()?,
          _ => {}
      }
      Ok(())
  }

  fn exit(&mut self) {
      self.exit = true;
  }

  fn increment_counter(&mut self) -> Result<()> {
      self.counter += 1;
      if self.counter > 2 {
          bail!("counter overflow")
      }
      Ok(())
  }

  fn decrement_counter(&mut self) -> Result<()> {
      self.counter -= 1;
      Ok(())
  }
}

impl Widget for &App {
  fn render(self, area: Rect, buf: &mut Buffer) {
    let title = Title::from(" Counter App Tutorial ".bold());
    let instructions = Title::from(Line::from(vec![
      " Decrement ".into(),
      "<Left>".blue().bold(),
      " Increment ".into(),
      "<Right>".blue().bold(),
      " Quit ".into(),
      "<Q> ".blue().bold(),
    ]));
    let block = Block::default()
        .title(title.alignment(Alignment::Center))
        .title(
            instructions
                .alignment(Alignment::Center)
                .position(block::Position::Bottom),
        )
        .borders(Borders::ALL)
        .border_set(border::THICK);

    let counter_text = Text::from(vec![Line::from(vec![
        "Value: ".into(),
        self.counter.to_string().yellow(),
    ])]);

    Paragraph::new(counter_text)
        .centered()
        .block(block)
        .render(area, buf)
  }
}