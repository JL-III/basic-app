use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use database::{initialize_database, seed_database};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    widgets::{block::Title, Widget},
    Frame,
};

use anyhow::Result;

use ratatui::{
    prelude::*,
    symbols::border,
    widgets::{block::*, *},
};

use crate::{
    bill::Bill,
    database::{self, get_database_entry},
    tui,
};
use core::result::Result::Ok;

#[derive(Debug, Default)]
pub struct App {
    bill: Option<Bill>,
    exit: bool,
}

impl App {
    /// runs the application's main loop until the user quits
    pub fn run(&mut self, terminal: &mut tui::Tui) -> Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.render_frame(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn render_frame(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.size());
    }

    fn handle_events(&mut self) -> Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
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
                self.get_bill()?;
            }
            KeyCode::Char('s') => {
                println!("pressed 's'");
                seed_database()?;
            }
            _ => {}
        }
        Ok(())
    }

    fn exit(&mut self) {
        self.exit = true;
    }

    fn get_bill(&mut self) -> Result<()> {
        match get_database_entry(1) {
            Ok(bill) => {
                self.bill = Some(bill);
            }
            Err(e) => println!("Error: {:?}", e),
        }
        Ok(())
    }

    fn bill(&self) -> Option<&Bill> {
      self.bill.as_ref()
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
        let bill_text = if let Some(bill) = self.bill() {
          vec![
            Line::from(format!("ID: {}", bill.id)),
            Line::from(format!("Date: {}", bill.date)),
            Line::from(format!("Description: {}", bill.description)),
            Line::from(format!("Notes: {}", bill.notes)),
            Line::from(format!("Amount: {}", bill.amount)),
            Line::from(format!("Timestamp: {}", bill.timestamp)),
            Line::from(format!("Allocated: {}", bill.allocated)),
            Line::from(format!("Sent: {}", bill.sent)),
            Line::from(format!("Paid: {}", bill.paid)),
          ]
        } else {
          vec![Line::from("No bill loaded")]
        };

        Paragraph::new(bill_text)
            .centered()
            .block(block)
            .render(area, buf)
    }
}
