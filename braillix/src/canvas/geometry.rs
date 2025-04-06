use super::{coords::ToCoords, Canvas, Rasterize, Style};

pub struct Line {
    from: (i32, i32),
    to: (i32, i32),
}

impl Line {
    #[inline]
    pub fn new(from: impl ToCoords, to: impl ToCoords) -> Self {
        Self {
            from: from.to_coords_i32(),
            to: to.to_coords_i32(),
        }
    }
}

impl Rasterize for Line {
    fn rasterize_onto(&self, canvas: &mut Canvas, style: Style) {
        let brightness = match style.outline {
            Some(b) => b,
            None => return,
        };

        let (x0, y0) = self.from;
        let (x1, y1) = self.to;

        match (x0 == x1, y0 == y1) {
            (true, true) => canvas.set_with_brightness((x0, y0), brightness),
            (false, true) => canvas.draw_hor_line(self.from, self.to, brightness),
            (true, false) => canvas.draw_ver_line(self.from, self.to, brightness),
            (false, false) => {
                // Generalized Bresenham algorithm sourced from:
                // https://en.wikipedia.org/wiki/Bresenham%27s_line_algorithm#All_cases

                let mut x0 = x0;
                let mut y0 = y0;

                let dx = (x1 - x0).abs();
                let sx = (x1 - x0).signum();
                let dy = -(y1 - y0).abs();
                let sy = (y1 - y0).signum();
                let mut error = dx + dy;

                loop {
                    canvas.set_with_brightness((x0, y0), brightness);
                    let e2 = 2 * error;

                    if e2 >= dy {
                        if x0 == x1 {
                            break;
                        }
                        error += dy;
                        x0 += sx;
                    }

                    if e2 <= dx {
                        if y0 == y1 {
                            break;
                        }
                        error += dx;
                        y0 += sy;
                    }
                }
            }
        }
    }
}

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

        let (min_x, max_x) = super::min_and_max(x, p1.0);
        let (min_y, max_y) = super::min_and_max(y, p1.1);

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

pub struct Circle {
    center: (i32, i32),
    radius: i32,
}

impl Circle {
    #[inline]
    pub fn new(center: impl ToCoords, radius: i32) -> Self {
        Self {
            center: center.to_coords_i32(),
            radius,
        }
    }
}

impl Rasterize for Circle {
    fn rasterize_onto(&self, canvas: &mut Canvas, style: Style) {
        // Implementation from
        // https://en.wikipedia.org/wiki/Midpoint_circle_algorithm#Jesko%27s_Method

        let (cx, cy) = self.center;
        let mut x = self.radius;
        let mut y = 0;
        let mut t1 = x / 16;

        while x >= y {
            for qx in [-1, 1] {
                for qy in [-1, 1] {
                    let o1 = (cx + qx * x, cy + qy * y);
                    let o2 = (cx + qx * y, cy + qy * x);

                    if let Some(brightness) = style.fill {
                        // Draw a line from the diagonal to the point on each octant.
                        let d = (o2.0, o1.1);
                        canvas.draw(
                            Line::new(d, o1),
                            Style::outlined_with_brightness(brightness),
                        );
                        canvas.draw(
                            Line::new(d, o2),
                            Style::outlined_with_brightness(brightness),
                        );
                    }

                    if let Some(brightness) = style.distinguishable_outline() {
                        canvas.set_with_brightness(o1, brightness);
                        canvas.set_with_brightness(o2, brightness);
                    }
                }
            }

            y += 1;
            t1 += y;
            let t2 = t1 - x;
            if t2 >= 0 {
                t1 = t2;
                x -= 1;
            }
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
        // TODO: filled triangles
        if let Some(brightness) = style.outline {
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
