use crate::Result;
use crossterm::event::{
    self, Event as CrosstermEvent, KeyEvent, MouseButton, MouseEvent, MouseEventKind,
};
use std::{
    sync::mpsc,
    thread,
    time::{Duration, Instant},
};

/// Terminal events.
#[derive(Clone, Copy, Debug)]
pub enum Event {
    /// Key press.
    Key(KeyEvent),
    /// Mouse click/scroll.
    Mouse(MouseEvent),
    /// Left double click.
    MouseDoubleClick,
    /// Terminal resize.
    Resize(u16, u16),
}

/// Terminal event handler.
#[derive(Debug)]
pub struct EventHandler {
    /// Event sender channel.
    #[allow(dead_code)]
    sender: mpsc::Sender<Event>,
    /// Event receiver channel.
    receiver: mpsc::Receiver<Event>,

    /// Event handler thread.
    #[allow(dead_code)]
    handler: thread::JoinHandle<()>,
}

impl EventHandler {
    /// Constructs a new instance of [`EventHandler`].
    pub fn new(timeout: u64) -> Self {
        let timeout = Duration::from_millis(timeout);
        let (sender, receiver) = mpsc::channel();
        let handler = {
            let sender = sender.clone();
            thread::spawn(move || {
                let mut last_click = Instant::now();
                loop {
                    if event::poll(timeout).expect("unable to poll for event") {
                        match event::read().expect("unable to read event") {
                            CrosstermEvent::Key(e) => {
                                if e.kind == event::KeyEventKind::Press {
                                    sender.send(Event::Key(e))
                                } else {
                                    Ok(()) // ignore KeyEventKind::Release on windows
                                }
                            }
                            CrosstermEvent::Mouse(e) => {
                                // Record the time of left clicking, and emit the MouseDoubleClick event
                                // when the gap between two clicks is shorter than a duration.
                                if let MouseEventKind::Down(MouseButton::Left) = &e.kind {
                                    let now = Instant::now();
                                    let old = std::mem::replace(&mut last_click, now);
                                    if let Some(diff) = now.checked_duration_since(old) {
                                        if diff < Duration::from_millis(450) {
                                            sender
                                                .send(Event::MouseDoubleClick)
                                                .expect("failed to send MouseDoubleClick event");
                                            continue; // no need to emit Mouse click event
                                        }
                                    }
                                }
                                sender.send(Event::Mouse(e))
                            }
                            CrosstermEvent::Resize(w, h) => sender.send(Event::Resize(w, h)),
                            _ => Ok(()),
                        }
                        .expect("failed to send terminal event")
                    }
                }
            })
        };
        Self {
            sender,
            receiver,
            handler,
        }
    }

    /// Receive the next event from the handler thread.
    ///
    /// This function will always block the current thread if
    /// there is no data available and it's possible for more data to be sent.
    pub fn next(&self) -> Result<Event> {
        Ok(self.receiver.recv()?)
    }
}
