use std::{
    f64::consts::{FRAC_PI_6, PI},
    io,
    time::Duration,
};

use braillix::canvas::{Canvas, Style};
use braillix_ratatui::animation::{AnimState, Animation};

#[derive(Default)]
struct State {
    theta: f64,
}

impl AnimState for State {
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

        let center = {
            let (w, h) = canvas.display.dot_size();
            (w / 2, h / 2)
        };

        let tri_size = (canvas.display.dot_width()).min(canvas.display.dot_height()) as f64 * 0.4;

        let (sin_theta, cos_theta) = self.theta.sin_cos();
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
            canvas.draw_line(center, p, Style::outlined());
        }

        canvas.draw_tri(
            transformed_points[0],
            transformed_points[1],
            transformed_points[2],
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
