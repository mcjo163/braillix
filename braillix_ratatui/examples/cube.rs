use nalgebra::{point, zero, Isometry3, Perspective3, Point3, Vector3};
use std::{f64::consts::FRAC_PI_6, io, time::Duration};

use braillix::canvas::{
    geometry::{Line, Tri},
    Canvas, Style,
};
use braillix_ratatui::animation::{Animation, AnimationState};

#[rustfmt::skip]
const CUBE_VERTICES: [Point3<f64>; 8] = [
    point!( 0.5,  0.5,  0.5), //    top, front, right
    point!(-0.5,  0.5,  0.5), //    top, front,  left
    point!(-0.5,  0.5, -0.5), //    top,  back,  left
    point!( 0.5,  0.5, -0.5), //    top,  back, right
    point!( 0.5, -0.5,  0.5), // bottom, front, right
    point!(-0.5, -0.5,  0.5), // bottom, front,  left
    point!(-0.5, -0.5, -0.5), // bottom,  back,  left
    point!( 0.5, -0.5, -0.5), // bottom,  back, right
];

/// ```
///   2-----3
///  /|    /|
/// 1-----0 |
/// | |   | |
/// | 6 --|-7
/// |/    |/
/// 5-----4
/// ```
const TRI_INDICES: [[usize; 3]; 12] = [
    [0, 1, 2], // top
    [2, 3, 0],
    [4, 5, 1], // front
    [1, 0, 4],
    [7, 4, 0], // right
    [0, 3, 7],
    [6, 7, 3], // back
    [3, 2, 6],
    [5, 6, 2], // left
    [2, 1, 5],
    [7, 6, 5], // bottom
    [5, 4, 7],
];

fn map_ndc_to_canvas_coords(p: Point3<f64>, w: f64, h: f64) -> (f64, f64) {
    let (x, y) = (p.x, p.y);
    (
        (x + 1.0) * (w / 2.0),
        (-y + 1.0) * (h / 2.0), // Flip y axis
    )
}

#[derive(Default)]
struct State {
    t: f64,
}

impl AnimationState for State {
    fn update(&mut self, delta: Duration) {
        self.t += delta.as_secs_f64();
    }

    fn paint(&self, canvas: &mut Canvas) {
        canvas.clear();

        let (w, h) = canvas.dot_size();
        let (w, h) = (w as f64, h as f64);

        let model = Isometry3::new(Vector3::z() * -5.0, zero());

        let view = {
            let eye = point!(0.0, self.t.sin(), 0.0);
            let target = point!(0.0, 0.0, -5.0);
            Isometry3::look_at_rh(&eye, &target, &Vector3::y())
        };

        let perspective = Perspective3::new(w / h, FRAC_PI_6, 1.0, 10.0);

        let ndc_vertices: Vec<_> = CUBE_VERTICES
            .iter()
            .map(|p| {
                let translated = view * model * p;
                perspective.project_point(&translated)
            })
            .collect();

        for tri in TRI_INDICES.iter() {
            canvas.draw(
                Tri::new(
                    map_ndc_to_canvas_coords(ndc_vertices[tri[0]], w, h),
                    map_ndc_to_canvas_coords(ndc_vertices[tri[1]], w, h),
                    map_ndc_to_canvas_coords(ndc_vertices[tri[2]], w, h),
                ),
                Style::outlined(),
            );
        }
    }
}

fn main() -> io::Result<()> {
    ratatui::run(|term| Animation::new(term, State::default())?.run(30.0))
}
