use crate::{app::App, event::EventHandler, ui::Page, Result};
use color_eyre::eyre;
use crossterm::{
    cursor,
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{self, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, layout::Rect, Terminal};
use std::{io, panic};

pub struct Tui {
    /// Interface to the Terminal.
    terminal: CrosstermTerminal,
    /// Terminal event handler.
    pub events: EventHandler,
}

impl Tui {
    pub fn new(timeout: u64) -> Result<Tui> {
        enter_terminal()?;
        let terminal = CrosstermTerminal::new(CrosstermBackend::new(pipeline()))?;
        let events = EventHandler::new(timeout);
        Ok(Tui { terminal, events })
    }

    pub fn draw(&mut self, app: &mut App, widgets: &mut Page) -> Result<()> {
        self.terminal
            .draw(|frame| crate::ui::render(app, widgets, frame))?;
        Ok(())
    }

    pub fn full_area(&self) -> Result<Rect> {
        Ok(self.terminal.size()?)
    }
}

impl Drop for Tui {
    fn drop(&mut self) {
        restore_terminal().expect("Failed to restore terminal when dropping App");
    }
}

pub type CrosstermTerminal = Terminal<CrosstermBackend<io::Stdout>>;

fn pipeline() -> io::Stdout {
    io::stdout()
}

/// Resets the terminal interface when the program exits.
///
/// This function is also used for the panic hook to revert
/// the terminal properties if unexpected errors occur.
fn restore_terminal() -> io::Result<()> {
    terminal::disable_raw_mode()?;
    execute!(
        pipeline(),
        LeaveAlternateScreen,
        DisableMouseCapture,
        cursor::Show
    )?;
    Ok(())
}

/// Set alternate screen and mouse capturing etc when the program starts.
fn enter_terminal() -> io::Result<()> {
    terminal::enable_raw_mode()?;
    execute!(
        pipeline(),
        EnterAlternateScreen,
        EnableMouseCapture,
        cursor::Hide,
        Clear(ClearType::All)
    )?;
    Ok(())
}

/// This replaces the standard color_eyre panic and error hooks with hooks that
/// restore the terminal before printing the panic or error.
pub fn install_hooks() -> crate::Result<()> {
    // add any extra configuration you need to the hook builder
    let hook_builder = color_eyre::config::HookBuilder::default();
    let (panic_hook, eyre_hook) = hook_builder.into_hooks();

    // convert from a color_eyre PanicHook to a standard panic hook
    let panic_hook = panic_hook.into_panic_hook();
    panic::set_hook(Box::new(move |panic_info| {
        restore_terminal().unwrap();
        panic_hook(panic_info);
    }));

    // convert from a color_eyre EyreHook to a eyre ErrorHook
    let eyre_hook = eyre_hook.into_eyre_hook();
    eyre::set_hook(Box::new(move |error| {
        restore_terminal().unwrap();
        eyre_hook(error)
    }))?;

    Ok(())
}
