use nalgebra::{
    point, vector, Isometry3, Perspective3, Point3, Translation3, UnitQuaternion, Vector3,
};
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

const LIGHT_DIRECTION: Vector3<f64> = vector!(-4.0, 10.0, 6.0);
const AMBIENT_LIGHT: f64 = 0.1;

/// Converts a `nalgebra::Point3<f64>` in the canonical view volume into
/// screen-space coordinates. Drops the z value, relying on backface culling
/// and the simplicity of the cube model to avoid layering issues.
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

        // Model transform centers the cube at (0, 0, -5) and applies a rotation
        // based on the time value.
        let model = Isometry3::from_parts(
            Translation3::from(Vector3::z() * -5.0),
            UnitQuaternion::from_euler_angles(-self.t * 0.8, self.t * 0.9, self.t * 0.2),
        );

        // View transform lines up the camera to look at the cube's origin.
        let view = {
            let eye = point!(0.0, 1.5, 0.0);
            let target = point!(0.0, 0.0, -5.0);
            Isometry3::look_at_rh(&eye, &target, &Vector3::y())
        };

        // Perspective projection adjusted based on the canvas aspect ratio.
        let perspective = Perspective3::new(w / h, FRAC_PI_6, 1.0, 10.0);

        // Calculates the view-space and NDC coordinates for each vertex of
        // the cube. The view-space values are used for lighting calculation
        // and the NDC for culling and drawing.
        let transformed_vertices: Vec<_> = CUBE_VERTICES
            .iter()
            .map(|p| {
                let translated = view * model * p;
                (translated, perspective.project_point(&translated))
            })
            .collect();

        for tri in TRI_INDICES.iter() {
            // Fetch the vertex data for this triangle.
            let (view, ndc): (Vec<_>, Vec<_>) =
                tri.iter().map(|&i| transformed_vertices[i]).unzip();

            // Calculate the normal in NDC to see if we can ignore the
            // triangle since it faces away from the camera.
            let ndc_normal = {
                let edge1 = ndc[0] - ndc[1];
                let edge2 = ndc[2] - ndc[1];
                edge1.cross(&edge2).normalize()
            };
            if ndc_normal.z <= 0.0 {
                continue;
            }

            // Calculate the normal in view space for the brightness calculation.
            let view_normal = {
                let edge1 = view[0] - view[1];
                let edge2 = view[2] - view[1];
                edge1.cross(&edge2).normalize()
            };
            let brightness_from_light = LIGHT_DIRECTION.normalize().dot(&view_normal).max(0.0);

            let face_brightness = AMBIENT_LIGHT + brightness_from_light;
            let edge_brightness = face_brightness + 0.3;

            // Draw face triangle.
            canvas.draw(
                Tri::new(
                    map_ndc_to_canvas_coords(ndc[0], w, h),
                    map_ndc_to_canvas_coords(ndc[1], w, h),
                    map_ndc_to_canvas_coords(ndc[2], w, h),
                ),
                Style::filled_with_brightness_f64(face_brightness),
            );

            // Draw face outlines.
            //
            // Relies on the fact that the triangles are defined consistently;
            // these are always the edges (not the diagonals).
            canvas.draw(
                Line::new(
                    map_ndc_to_canvas_coords(ndc[0], w, h),
                    map_ndc_to_canvas_coords(ndc[1], w, h),
                ),
                Style::outlined_with_brightness_f64(edge_brightness),
            );
            canvas.draw(
                Line::new(
                    map_ndc_to_canvas_coords(ndc[1], w, h),
                    map_ndc_to_canvas_coords(ndc[2], w, h),
                ),
                Style::outlined_with_brightness_f64(edge_brightness),
            );
        }
    }
}

fn main() -> io::Result<()> {
    ratatui::run(|term| Animation::new(term, State::default())?.run(60.0))
}
