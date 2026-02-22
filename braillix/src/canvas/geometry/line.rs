use crate::canvas::{coords::ToCoords, Canvas, Rasterize, Style};

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
