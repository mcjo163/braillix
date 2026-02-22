use crate::canvas::{coords::ToCoords, geometry::Line, min_and_max, Canvas, Rasterize, Style};

pub struct Rect {
    top_left: (i32, i32),
    dim: (i32, i32),
}

impl Rect {
    #[inline]
    pub fn new(top_left: impl ToCoords, dim: impl ToCoords) -> Self {
        Self {
            top_left: top_left.to_coords_i32(),
            dim: dim.to_coords_i32(),
        }
    }
}

impl Rasterize for Rect {
    fn rasterize_onto(&self, canvas: &mut Canvas, style: Style) {
        let (x, y) = self.top_left;
        let (w, h) = self.dim;
        let p1 = (x + w, y + h);

        let (min_x, max_x) = min_and_max(x, p1.0);
        let (min_y, max_y) = min_and_max(y, p1.1);

        // p0: top-left, p1: bottom-right
        let p0 = (min_x, min_y);
        let p1 = (max_x, max_y);

        if let Some(brightness) = style.fill {
            for y in p0.1..p1.1 {
                canvas.draw(
                    Line::new((p0.0, y), (p1.0 - 1, y)),
                    Style::outlined_with_brightness(brightness),
                );
            }
        }

        if let Some(brightness) = style.distinguishable_outline() {
            let w = p1.0 - p0.0;
            let h = p1.1 - p0.1;

            if w == 0 || h == 0 {
                return;
            }

            if w == 1 && h == 1 {
                canvas.draw(
                    Line::new(p0, p0),
                    Style::outlined_with_brightness(brightness),
                );
                return;
            }

            if w == 1 || h == 1 {
                canvas.draw(
                    Line::new(p0, p1),
                    Style::outlined_with_brightness(brightness),
                );
            }

            // draw top and bottom edges
            canvas.draw(
                Line::new(p0, (p1.0 - 1, p0.1)),
                Style::outlined_with_brightness(brightness),
            );
            canvas.draw(
                Line::new((p0.0, p1.1 - 1), (p1.0 - 1, p1.1 - 1)),
                Style::outlined_with_brightness(brightness),
            );

            if h > 2 {
                // draw left and right edges
                canvas.draw(
                    Line::new((p0.0, p0.1 + 1), (p0.0, p1.1 - 2)),
                    Style::outlined_with_brightness(brightness),
                );
                canvas.draw(
                    Line::new((p1.0 - 1, p0.1 + 1), (p1.0 - 1, p1.1 - 2)),
                    Style::outlined_with_brightness(brightness),
                );
            }
        }
    }
}
