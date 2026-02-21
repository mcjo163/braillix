use crate::canvas::{coords::ToCoords, geometry::Line, Canvas, Rasterize, Style};

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
