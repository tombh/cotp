use std::io;

use crossterm::event::{DisableMouseCapture, EnableMouseCapture};
use crossterm::terminal::{self, EnterAlternateScreen, LeaveAlternateScreen};
use tui::backend::Backend;
use tui::Terminal;

use crate::interface::app::{App, AppResult};
use crate::interface::event::EventHandler;

/// Representation of a terminal user interface.
///
/// It is responsible for setting up the terminal,
/// initializing the interface and handling the draw events.
#[derive(Debug)]
pub struct Tui<B: Backend> {
    /// Interface to the Terminal.
    terminal: Terminal<B>,
    /// Terminal event handler.
    pub events: EventHandler,
}

impl<B: Backend> Tui<B> {
    /// Constructs a new instance of [`Tui`].
    pub fn new(terminal: Terminal<B>, events: EventHandler) -> Self {
        Self { terminal, events }
    }

    /// Initializes the terminal interface.
    ///
    /// It enables the raw mode and sets terminal properties.
    pub fn init(&mut self) -> AppResult<()> {
        terminal::enable_raw_mode()?;
        crossterm::execute!(io::stderr(), EnterAlternateScreen, EnableMouseCapture)?;
        self.terminal.hide_cursor()?;
        self.terminal.clear()?;
        Ok(())
    }

    /// [`Draw`] the terminal interface by [`rendering`] the widgets.
    ///
    /// [`Draw`]: tui::Terminal::draw
    /// [`rendering`]: crate::app::App::render
    pub fn draw(&mut self, app: &mut App) -> AppResult<()> {
        self.terminal.draw(|frame| app.render(frame))?;
        Ok(())
    }

    /// Exits the terminal interface.
    ///
    /// It disables the raw mode and reverts back the terminal properties.
    pub fn exit(&mut self) -> AppResult<()> {
        terminal::disable_raw_mode()?;
        crossterm::execute!(io::stderr(), LeaveAlternateScreen, DisableMouseCapture)?;
        self.terminal.show_cursor()?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::otp::otp_element::OTPElement;

    use super::*;
    use tui::backend::TestBackend;
    #[test]
    fn test_term_tui() -> AppResult<()> {
        let elements: Vec<OTPElement> = vec![
            OTPElement::new(
                "ORSXG5A=".to_string(),
                "Test".to_string(),
                "Test".to_string(),
                6,
                "TOTP".to_string(),
                "SHA1".to_string(),
                String::new(),
                0,
                0,
                30,
                0,
                vec![],
            ),
            OTPElement::new(
                "OZ2HE3TP".to_string(),
                "Test".to_string(),
                "Test".to_string(),
                6,
                "TOTP".to_string(),
                "SHA1".to_string(),
                String::new(),
                0,
                0,
                30,
                0,
                vec![],
            ),
        ];
        let mut app: App = App::new(elements);

        let backend = TestBackend::new(500, 500);
        let terminal = Terminal::new(backend)?;
        let mut tui = Tui::new(terminal, EventHandler::new(10));
        tui.init()?;
        tui.draw(&mut app)?;
        tui.exit()?;
        Ok(())
    }
}
