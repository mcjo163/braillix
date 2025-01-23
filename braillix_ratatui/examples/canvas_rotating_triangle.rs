use std::{
    f64::consts::{FRAC_PI_6, PI},
    io,
    time::{Duration, Instant},
};

use braillix::canvas::Canvas;
use braillix_ratatui::ToWidget;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{layout::Size, DefaultTerminal, Frame};

struct App {
    quit: bool,
    canvas: Canvas,
    theta: f64,
}

impl App {
    fn with_term_size(size: Size) -> Self {
        Self {
            quit: false,
            canvas: Canvas::with_output_size(size.width as usize, size.height as usize),
            theta: 0.0,
        }
    }

    fn run(&mut self, term: &mut DefaultTerminal) -> io::Result<()> {
        let tick_rate = Duration::from_secs_f64(1.0 / 60.0);
        let mut last_tick = Instant::now();

        loop {
            term.draw(|f| {
                self.paint();
                self.draw_to_frame(f);
            })?;

            let timeout = tick_rate.saturating_sub(last_tick.elapsed());
            if event::poll(timeout)? {
                if let Event::Key(evt) = event::read()? {
                    if evt.kind == KeyEventKind::Press {
                        self.handle_keypress(evt);
                    }
                }
            }

            if last_tick.elapsed() >= tick_rate {
                self.update();
                last_tick = Instant::now();
            }

            if self.quit {
                return Ok(());
            }
        }
    }

    fn handle_keypress(&mut self, event: KeyEvent) {
        match event.code {
            KeyCode::Char('q') => {
                self.quit = true;
            }
            _ => {}
        }
    }

    fn update(&mut self) {
        self.theta = (self.theta + 0.02) % (2.0 * PI);
    }

    fn paint(&mut self) {
        // Needs ``&mut self` since the canvas is mutated when drawn to.
        self.canvas.clear();

        // These are in (y, x) form so that f64::sin_cos can be used verbatim.
        let verts_around_origin = [
            (-1.0, 0.0),
            (FRAC_PI_6).sin_cos(),
            (5.0 * FRAC_PI_6).sin_cos(),
        ];

        let center = {
            let (w, h) = self.canvas.display.dot_size();
            (w / 2, h / 2)
        };

        let tri_size =
            (self.canvas.display.dot_width()).min(self.canvas.display.dot_height()) as f64 * 0.4;

        let (sin_theta, cos_theta) = self.theta.sin_cos();

        // TODO: consider using a linear algebra library and making the canvas API support signed ints and/or floats

        let transformed_points: Vec<_> = verts_around_origin
            .iter()
            .map(|(y, x)| {
                let tx = tri_size * (x * cos_theta - y * sin_theta);
                let ty = tri_size * (y * cos_theta + x * sin_theta);
                (
                    if tx > 0.0 {
                        center.0 + tx.round() as usize
                    } else {
                        center.0 - (-tx).round() as usize
                    },
                    if ty > 0.0 {
                        center.1 + ty as usize
                    } else {
                        center.1 - (-ty) as usize
                    },
                )
            })
            .collect();

        for &p in transformed_points.iter() {
            self.canvas.draw_line(center, p);
        }

        self.canvas.draw_tri(
            transformed_points[0],
            transformed_points[1],
            transformed_points[2],
        );
    }

    fn draw_to_frame(&self, frame: &mut Frame) {
        frame.render_widget(self.canvas.widget(), frame.area());
    }
}

fn main() -> io::Result<()> {
    let mut term = ratatui::init();
    let res = App::with_term_size(term.size()?).run(&mut term);
    ratatui::restore();
    res
}
