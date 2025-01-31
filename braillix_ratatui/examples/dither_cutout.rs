use std::{io, time::Duration};

use braillix::canvas::{Canvas, Style};
use braillix_ratatui::animation::{Animation, AnimationState};

#[derive(Default)]
struct State {
    t: f64,
}

impl AnimationState for State {
    fn update(&mut self, delta: Duration) {
        self.t += delta.as_secs_f64();
    }

    fn paint(&self, canvas: &mut Canvas) {
        let b = ((self.t * 2.0).cos() + 1.0) / 2.0;
        canvas.draw_rect((10, 10), (80, 80), Style::filled());
        canvas.draw_circle((50, 50), 30, Style::filled_with_brightness_f64(b));
    }
}

fn main() -> io::Result<()> {
    let mut term = ratatui::init();
    let res = Animation::new(&mut term, State::default())?.run(60.0);
    ratatui::restore();
    res
}
