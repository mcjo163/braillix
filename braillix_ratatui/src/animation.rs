use std::{
    io,
    time::{Duration, Instant},
};

use braillix::canvas::Canvas;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{layout::Size, DefaultTerminal};

use crate::ToWidget;

/// Implement this trait for your custom state type to use it for an
/// `Animation`. `update` is called once per tick and `paint` is called
/// to draw each frame.
pub trait AnimState {
    /// `update` is called once per tick to update the state.
    fn update(&mut self, delta: Duration);

    /// `paint` is called before drawing to the terminal to
    /// get the canvas ready for display.
    fn paint(&self, canvas: &mut Canvas);
}

/// A simple ratatui app that can be used for programs that just need
/// to draw fullscreen based on some state that gets updated.
pub struct Animation<'a, S: AnimState> {
    terminal: &'a mut DefaultTerminal,
    canvas: Canvas,
    state: S,
    quit_requested: bool,
}

impl<'a, S: AnimState> Animation<'a, S> {
    /// Create a new `Animation` with a terminal reference and an
    /// initial value for the state.
    pub fn new(terminal: &'a mut DefaultTerminal, initial_state: S) -> io::Result<Self> {
        let Size { width, height } = terminal.size()?;
        Ok(Self {
            terminal,
            canvas: Canvas::with_output_size(width as usize, height as usize),
            state: initial_state,
            quit_requested: false,
        })
    }

    /// Run the animation at the desired FPS. Listens for `Q`, `Esc`,
    /// and `ctrl-c` to quit.
    pub fn run(&mut self, fps: f64) -> io::Result<()> {
        let tick_rate = Duration::from_secs_f64(1.0 / fps);
        let mut last_tick = Instant::now();

        loop {
            self.state.paint(&mut self.canvas);

            self.terminal
                .draw(|f| f.render_widget(self.canvas.widget(), f.area()))?;

            let timeout = tick_rate.saturating_sub(last_tick.elapsed());
            if event::poll(timeout)? {
                if let Event::Key(evt) = event::read()? {
                    if evt.kind == KeyEventKind::Press {
                        self.handle_keypress(evt);
                    }
                }
            }

            if last_tick.elapsed() >= tick_rate {
                self.state.update(last_tick.elapsed());
                last_tick = Instant::now();
            }

            if self.quit_requested {
                return Ok(());
            }
        }
    }

    fn handle_keypress(&mut self, event: KeyEvent) {
        match event.code {
            KeyCode::Char('q') | KeyCode::Esc => {
                self.quit_requested = true;
            }
            KeyCode::Char('c') if event.modifiers.contains(KeyModifiers::CONTROL) => {
                self.quit_requested = true;
            }
            _ => {}
        }
    }
}
