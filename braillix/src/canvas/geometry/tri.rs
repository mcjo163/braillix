use crate::canvas::{coords::ToCoords, geometry::Line, Canvas, Rasterize, Style};

#[derive(Debug)]
struct Edge {
    p0: (i32, i32),
    p1: (i32, i32),
}

impl Edge {
    fn new(p0: impl ToCoords, p1: impl ToCoords) -> Self {
        let p0 = p0.to_coords_i32();
        let p1 = p1.to_coords_i32();

        // Ensure that the lower-y point is first.
        if p0.1 <= p1.1 {
            Self { p0, p1 }
        } else {
            Self { p0: p1, p1: p0 }
        }
    }
}

pub struct Tri {
    p0: (f64, f64),
    p1: (f64, f64),
    p2: (f64, f64),
}

impl Tri {
    #[inline]
    pub fn new(p0: impl ToCoords, p1: impl ToCoords, p2: impl ToCoords) -> Self {
        Self {
            p0: p0.to_coords_f64(),
            p1: p1.to_coords_f64(),
            p2: p2.to_coords_f64(),
        }
    }
}

impl Rasterize for Tri {
    fn rasterize_onto(&self, canvas: &mut Canvas, style: Style) {
        if let Some(brightness) = style.fill {
            // Adapted from https://joshbeam.com/articles/triangle_rasterization/.

            let edges = [
                Edge::new(self.p0, self.p1),
                Edge::new(self.p1, self.p2),
                Edge::new(self.p2, self.p0),
            ];

            let mut max_length = 0;
            let mut long_edge = 0;

            for (i, edge) in edges.iter().enumerate() {
                let y_diff = edge.p1.1 - edge.p0.1;
                if y_diff > max_length {
                    max_length = y_diff;
                    long_edge = i;
                }
            }

            let short_edge_1 = (long_edge + 1) % 3;
            let short_edge_2 = (long_edge + 2) % 3;

            let mut fill_between_edges = |long: &Edge, short: &Edge| {
                let long_dy = (long.p1.1 - long.p0.1) as f64;
                let short_dy = (short.p1.1 - short.p0.1) as f64;
                if long_dy == 0. || short_dy == 0. {
                    return;
                }

                let long_dx = (long.p1.0 - long.p0.0) as f64;
                let short_dx = (short.p1.0 - short.p0.0) as f64;

                let mut long_interp_factor = (short.p0.1 - long.p0.1) as f64 / long_dy;
                let mut short_interp_factor: f64 = 0.;

                for y in short.p0.1..=short.p1.1 {
                    let x0 = long.p0.0 + (long_dx * long_interp_factor) as i32;
                    let x1 = short.p0.0 + (short_dx * short_interp_factor) as i32;

                    canvas.draw_hor_line((x0, y), (x1, y), brightness);

                    long_interp_factor += 1. / long_dy;
                    short_interp_factor += 1. / short_dy;
                }
            };

            fill_between_edges(&edges[long_edge], &edges[short_edge_1]);
            fill_between_edges(&edges[long_edge], &edges[short_edge_2]);
        }

        if let Some(brightness) = style.distinguishable_outline() {
            canvas.draw(
                Line::new(self.p0, self.p1),
                Style::outlined_with_brightness(brightness),
            );
            canvas.draw(
                Line::new(self.p1, self.p2),
                Style::outlined_with_brightness(brightness),
            );
            canvas.draw(
                Line::new(self.p2, self.p0),
                Style::outlined_with_brightness(brightness),
            );
        }
    }
}
