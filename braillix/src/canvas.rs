#![allow(dead_code)]

use crate::display::Display;

#[derive(PartialEq, Eq)]
pub enum ShapeStyle {
    Outlined,
    Filled,
}

// TODO: replace this with nalgebra or at least offer signed
// and float alternatives.
type Point = (usize, usize);

/// A canvas that offers a higher-level API on top of `Display`
/// with drawing primitives.
pub struct Canvas {
    pub display: Display,
}

// Public API
impl Canvas {
    pub fn with_dot_size(width: usize, height: usize) -> Self {
        let display = Display::with_dot_size(width, height);
        Self { display }
    }

    pub fn with_output_size(width: usize, height: usize) -> Self {
        let display = Display::with_output_size(width, height);
        Self { display }
    }

    // TODO: is it worth re-exposing some/all `Display` methods or just making it public?
    pub fn clear(&mut self) {
        self.display.clear();
    }

    pub fn draw_line(&mut self, p0: Point, p1: Point) {
        let (x0, y0) = p0;
        let (x1, y1) = p1;

        match (x0 == x1, y0 == y1) {
            (true, true) => self.set_if_in_bounds(x0, y0),
            (false, true) => self.draw_hor_line(y0, x0, x1),
            (true, false) => self.draw_ver_line(x0, y0, y1),
            (false, false) => {
                // Generalized Bresenham algorithm sourced from:
                // https://en.wikipedia.org/wiki/Bresenham%27s_line_algorithm#All_cases

                let (mut x0, x1) = (x0 as isize, x1 as isize);
                let (mut y0, y1) = (y0 as isize, y1 as isize);

                let dx = (x1 - x0).abs();
                let sx = (x1 - x0).signum();
                let dy = -(y1 - y0).abs();
                let sy = (y1 - y0).signum();
                let mut error = dx + dy;

                loop {
                    self.set_if_in_bounds(x0 as usize, y0 as usize);
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

    pub fn draw_rect(&mut self, p: Point, w: usize, h: usize, style: ShapeStyle) {
        match style {
            ShapeStyle::Outlined => {
                // draw top and bottom edges
                if w == 0 || h == 0 {
                    return;
                }

                if w == 1 && h == 1 {
                    self.draw_line(p, p);
                    return;
                }

                if w == 1 || h == 1 {
                    self.draw_line(
                        p,
                        if w == 1 {
                            (p.0, p.1 + h - 1)
                        } else {
                            (p.0 + w - 1, p.1)
                        },
                    );
                }

                self.draw_line(p, (p.0 + w - 1, p.1));
                self.draw_line((p.0, p.1 + h - 1), (p.0 + w - 1, p.1 + h - 1));

                if h > 2 {
                    // draw left and right edges
                    self.draw_line((p.0, p.1 + 1), (p.0, p.1 + h - 2));
                    self.draw_line((p.0 + h - 1, p.1 + 1), (p.0 + h - 1, p.1 + h - 2));
                }
            }
            ShapeStyle::Filled => {
                for y in (p.1)..(p.1 + h) {
                    self.draw_line((p.0, y), (p.0 + w - 1, y));
                }
            }
        }
    }

    pub fn draw_tri(&mut self, p0: Point, p1: Point, p2: Point) {
        self.draw_line(p0, p1);
        self.draw_line(p1, p2);
        self.draw_line(p2, p0);
    }
}

// Private implemetation helpers.
impl Canvas {
    fn set_if_in_bounds(&mut self, x: usize, y: usize) {
        if x < self.display.dot_width() && y < self.display.dot_height() {
            self.display.set(x, y);
        }
    }

    fn draw_ver_line(&mut self, x: usize, y0: usize, y1: usize) {
        let (y0, y1) = min_and_max(y0, y1);
        for y in y0..=y1 {
            self.set_if_in_bounds(x, y);
        }
    }

    fn draw_hor_line(&mut self, y: usize, x0: usize, x1: usize) {
        let (x0, x1) = min_and_max(x0, x1);
        for x in x0..=x1 {
            self.set_if_in_bounds(x, y);
        }
    }
}

fn min_and_max(a: usize, b: usize) -> (usize, usize) {
    if a < b {
        (a, b)
    } else {
        (b, a)
    }
}
