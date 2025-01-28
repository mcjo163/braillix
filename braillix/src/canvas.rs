use std::fmt;

use crate::display::Display;

mod dither;

mod style;
pub use style::Style;

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
    /// Create a new `Canvas` with the given dot size.
    ///
    /// For now, the width must be a multiple of 2 and the height
    /// must be a multiple of 4 so that it maps nicely to braille
    /// characters.
    ///
    /// # Panics
    /// This function panics if the width and height do not meet
    /// the above constraints.
    pub fn with_dot_size(width: usize, height: usize) -> Self {
        let display = Display::with_dot_size(width, height);
        Self { display }
    }

    /// Creates a new `Canvas` with the given output (character) dimensions.
    pub fn with_output_size(width: usize, height: usize) -> Self {
        let display = Display::with_output_size(width, height);
        Self { display }
    }

    // TODO: re-export some other display methods that might be useful, like size getters
    pub fn clear(&mut self) {
        self.display.clear();
    }

    pub fn draw_line(&mut self, p0: Point, p1: Point, style: Style) {
        let brightness = match style.outline {
            Some(b) => b,
            None => return,
        };

        let (x0, y0) = p0;
        let (x1, y1) = p1;

        match (x0 == x1, y0 == y1) {
            (true, true) => self.set_dithered(x0, y0, brightness),
            (false, true) => self.draw_hor_line(y0, x0, x1, brightness),
            (true, false) => self.draw_ver_line(x0, y0, y1, brightness),
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
                    self.set_dithered(x0 as usize, y0 as usize, brightness);
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

    pub fn draw_rect(&mut self, p: Point, w: usize, h: usize, style: Style) {
        if let Some(brightness) = style.fill {
            for y in (p.1)..(p.1 + h) {
                self.draw_line(
                    (p.0, y),
                    (p.0 + w - 1, y),
                    Style::outlined_with_brightness(brightness),
                );
            }
        }

        if let Some(brightness) = style.distinguishable_outline() {
            // draw top and bottom edges
            if w == 0 || h == 0 {
                return;
            }

            if w == 1 && h == 1 {
                self.draw_line(p, p, Style::outlined_with_brightness(brightness));
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
                    Style::outlined_with_brightness(brightness),
                );
            }

            self.draw_line(
                p,
                (p.0 + w - 1, p.1),
                Style::outlined_with_brightness(brightness),
            );
            self.draw_line(
                (p.0, p.1 + h - 1),
                (p.0 + w - 1, p.1 + h - 1),
                Style::outlined_with_brightness(brightness),
            );

            if h > 2 {
                // draw left and right edges
                self.draw_line(
                    (p.0, p.1 + 1),
                    (p.0, p.1 + h - 2),
                    Style::outlined_with_brightness(brightness),
                );
                self.draw_line(
                    (p.0 + h - 1, p.1 + 1),
                    (p.0 + h - 1, p.1 + h - 2),
                    Style::outlined_with_brightness(brightness),
                );
            }
        }
    }

    pub fn draw_tri(&mut self, p0: Point, p1: Point, p2: Point, style: Style) {
        // TODO: filled triangles
        if let Some(brightness) = style.outline {
            self.draw_line(p0, p1, Style::outlined_with_brightness(brightness));
            self.draw_line(p1, p2, Style::outlined_with_brightness(brightness));
            self.draw_line(p2, p0, Style::outlined_with_brightness(brightness));
        }
    }

    pub fn draw_circle(&mut self, p: Point, r: usize, style: Style) {
        // Implementation from
        // https://en.wikipedia.org/wiki/Midpoint_circle_algorithm#Jesko%27s_Method
        let (cx, cy) = (p.0 as isize, p.1 as isize);
        let mut t1 = (r / 16) as isize;
        let mut x = r as isize;
        let mut y = 0;

        while x >= y {
            for m0 in [-1, 1] {
                for m1 in [-1, 1] {
                    let x_o1 = (cx + m0 * x) as usize;
                    let y_o1 = (cy + m1 * y) as usize;
                    let x_o2 = (cx + m0 * y) as usize;
                    let y_o2 = (cy + m1 * x) as usize;

                    if let Some(brightness) = style.fill {
                        // Draw a line from the diagonal to the point on each octant.
                        let p = ((cx + m0 * y) as usize, (cy + m1 * y) as usize);
                        self.draw_line(
                            p,
                            (x_o1, y_o1),
                            Style::outlined_with_brightness(brightness),
                        );
                        self.draw_line(
                            p,
                            (x_o2, y_o2),
                            Style::outlined_with_brightness(brightness),
                        );
                    }

                    if let Some(brightness) = style.distinguishable_outline() {
                        self.set_dithered(x_o1, y_o1, brightness);
                        self.set_dithered(x_o2, y_o2, brightness);
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

// Private implemetation helpers.
impl Canvas {
    fn is_in_bounds(&self, x: usize, y: usize) -> bool {
        let (w, h) = self.display.dot_size();
        x < w && y < h
    }

    fn set_dithered(&mut self, x: usize, y: usize, brightness: usize) {
        if self.is_in_bounds(x, y) {
            const MAX_B: usize = dither::max_brightness();
            match brightness {
                // Anything with 0 brightness will end up unset, and
                // anything above the max threshold will be set.
                0 => self.display.unset(x, y),
                MAX_B.. => self.display.set(x, y),

                b => {
                    if b > dither::threshold(x, y) {
                        self.display.set(x, y);
                    } else {
                        self.display.unset(x, y);
                    }
                }
            }
        }
    }

    fn draw_ver_line(&mut self, x: usize, y0: usize, y1: usize, brightness: usize) {
        let (y0, y1) = min_and_max(y0, y1);
        for y in y0..=y1 {
            self.set_dithered(x, y, brightness);
        }
    }

    fn draw_hor_line(&mut self, y: usize, x0: usize, x1: usize, brightness: usize) {
        let (x0, x1) = min_and_max(x0, x1);
        for x in x0..=x1 {
            self.set_dithered(x, y, brightness);
        }
    }
}

impl From<&Canvas> for String {
    fn from(value: &Canvas) -> Self {
        String::from(&value.display)
    }
}

impl From<Canvas> for String {
    fn from(value: Canvas) -> Self {
        String::from(&value)
    }
}

impl fmt::Display for Canvas {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", String::from(self))
    }
}

fn min_and_max(a: usize, b: usize) -> (usize, usize) {
    if a < b {
        (a, b)
    } else {
        (b, a)
    }
}
