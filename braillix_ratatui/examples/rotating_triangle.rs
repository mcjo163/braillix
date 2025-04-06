use std::{
    f64::consts::{FRAC_PI_6, PI},
    io,
    time::Duration,
};

use braillix::canvas::{
    geometry::{Line, Tri},
    Canvas, Style,
};
use braillix_ratatui::animation::{Animation, AnimationState};

#[derive(Default)]
struct State {
    theta: f64,
}

impl AnimationState for State {
    fn update(&mut self, delta: Duration) {
        let rotation_amount = 1.4 * delta.as_secs_f64();
        self.theta = (self.theta + rotation_amount) % (2.0 * PI);
    }

    fn paint(&self, canvas: &mut Canvas) {
        canvas.clear();

        // These are in (y, x) form so that f64::sin_cos can be used verbatim.
        let verts_around_origin = [
            (-1.0, 0.0),
            (FRAC_PI_6).sin_cos(),
            (5.0 * FRAC_PI_6).sin_cos(),
        ];

        let (dw, dh) = canvas.dot_size();
        let center = ((dw / 2) as f64, (dh / 2) as f64);
        let tri_size = dw.min(dh) as f64 * 0.4;

        let (sin_theta, cos_theta) = self.theta.sin_cos();
        let transformed_points: Vec<_> = verts_around_origin
            .iter()
            .map(|(y, x)| {
                let tx = tri_size * (x * cos_theta - y * sin_theta);
                let ty = tri_size * (y * cos_theta + x * sin_theta);

                (center.0 + tx, center.1 + ty)
            })
            .collect();

        for &p in transformed_points.iter() {
            canvas.draw(Line::new(center, p), Style::outlined());
        }

        canvas.draw(
            Tri::new(
                transformed_points[0],
                transformed_points[1],
                transformed_points[2],
            ),
            Style::outlined(),
        );
    }
}

fn main() -> io::Result<()> {
    let mut term = ratatui::init();
    let res = Animation::new(&mut term, State::default())?.run(60.0);
    ratatui::restore();
    res
}
