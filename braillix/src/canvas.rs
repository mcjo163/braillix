//! # canvas
//!
//! This module provides `Canvas` and related types. It uses a `Display`
//! internally and provides an abstraction for drawing shapes and lines.

use std::fmt;

use crate::display::Display;

mod coords;
use coords::{ToCoords, ToDisplay};

mod dither;

mod style;
pub use style::Style;

/// A canvas that offers a higher-level API on top of `Display`
/// with drawing primitives.
pub struct Canvas {
    display: Display,
}

// Public API
impl Canvas {
    /// Creates a new `Canvas` with the given dot size.
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

    /// Returns a reference to the underlying `Display`.
    pub fn display(&self) -> &Display {
        &self.display
    }

    /// Returns a mutable reference to the underlying `Display`.
    pub fn display_mut(&mut self) -> &mut Display {
        &mut self.display
    }

    /// Clears the canvas.
    pub fn clear(&mut self) {
        self.display.clear();
    }

    /// Gets the width of the canvas in dots.
    pub fn dot_width(&self) -> usize {
        self.display.dot_width()
    }

    /// Gets the height of the canvas in dots.
    pub fn dot_height(&self) -> usize {
        self.display.dot_height()
    }

    /// Gets the size (width, height) of the canvas in dots.
    pub fn dot_size(&self) -> (usize, usize) {
        self.display.dot_size()
    }

    /// Gets the width of the canvas in characters.
    pub fn output_width(&self) -> usize {
        self.display.output_width()
    }

    /// Gets the height of the canvas in characters.
    pub fn output_height(&self) -> usize {
        self.display.output_height()
    }

    /// Gets the size (width, height) of the canvas in characters.
    pub fn output_size(&self) -> (usize, usize) {
        self.display.output_size()
    }

    pub fn draw_line(&mut self, p0: impl ToCoords, p1: impl ToCoords, style: Style) {
        let brightness = match style.outline {
            Some(b) => b,
            None => return,
        };

        let (x0, y0) = p0.to_coords_i32();
        let (x1, y1) = p1.to_coords_i32();

        match (x0 == x1, y0 == y1) {
            (true, true) => self.set_with_brightness((x0, y0), brightness),
            (false, true) => self.draw_hor_line(p0, p1, brightness),
            (true, false) => self.draw_ver_line(p0, p1, brightness),
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
                    self.set_with_brightness((x0, y0), brightness);
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

    pub fn draw_rect(&mut self, p: impl ToCoords, dim: impl ToCoords, style: Style) {
        let p0 = p.to_coords_i32();
        let (w, h) = dim.to_coords_i32();
        let p1 = (p0.0 + w, p0.1 + h);

        let (min_x, max_x) = min_and_max(p0.0, p1.0);
        let (min_y, max_y) = min_and_max(p0.1, p1.1);

        // p0: top-left, p1: bottom-right
        let p0 = (min_x, min_y);
        let p1 = (max_x, max_y);

        if let Some(brightness) = style.fill {
            for y in p0.1..p1.1 {
                self.draw_line(
                    (p0.0, y),
                    (p1.0 - 1, y),
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
                self.draw_line(p, p, Style::outlined_with_brightness(brightness));
                return;
            }

            if w == 1 || h == 1 {
                self.draw_line(p0, p1, Style::outlined_with_brightness(brightness));
            }

            // draw top and bottom edges
            self.draw_line(
                p0,
                (p1.0 - 1, p0.1),
                Style::outlined_with_brightness(brightness),
            );
            self.draw_line(
                (p0.0, p1.1 - 1),
                (p1.0 - 1, p1.1 - 1),
                Style::outlined_with_brightness(brightness),
            );

            if h > 2 {
                // draw left and right edges
                self.draw_line(
                    (p0.0, p0.1 + 1),
                    (p0.0, p1.1 - 2),
                    Style::outlined_with_brightness(brightness),
                );
                self.draw_line(
                    (p1.0 - 1, p0.1 + 1),
                    (p1.0 - 1, p1.1 - 2),
                    Style::outlined_with_brightness(brightness),
                );
            }
        }
    }

    pub fn draw_tri(
        &mut self,
        p0: impl ToCoords,
        p1: impl ToCoords,
        p2: impl ToCoords,
        style: Style,
    ) {
        // TODO: filled triangles
        if let Some(brightness) = style.outline {
            self.draw_line(p0, p1, Style::outlined_with_brightness(brightness));
            self.draw_line(p1, p2, Style::outlined_with_brightness(brightness));
            self.draw_line(p2, p0, Style::outlined_with_brightness(brightness));
        }
    }

    pub fn draw_circle(&mut self, p: impl ToCoords, r: usize, style: Style) {
        // Implementation from
        // https://en.wikipedia.org/wiki/Midpoint_circle_algorithm#Jesko%27s_Method

        let (cx, cy) = p.to_coords_i32();
        let mut x = r as i32;
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
                        self.draw_line(d, o1, Style::outlined_with_brightness(brightness));
                        self.draw_line(d, o2, Style::outlined_with_brightness(brightness));
                    }

                    if let Some(brightness) = style.distinguishable_outline() {
                        self.set_with_brightness(o1, brightness);
                        self.set_with_brightness(o2, brightness);
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
    fn set_with_brightness(&mut self, p: impl ToDisplay, brightness: usize) {
        if let Some((x, y)) = p.to_display(self.dot_size()) {
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

    /// If `p1` has a different x-coordinate from `p0`, it is ignored.
    fn draw_ver_line(&mut self, p0: impl ToCoords, p1: impl ToCoords, brightness: usize) {
        let (x, y0) = p0.to_coords_i32();
        let (_, y1) = p1.to_coords_i32();

        let (y0, y1) = min_and_max(y0, y1);
        for y in y0..=y1 {
            self.set_with_brightness((x, y), brightness);
        }
    }

    /// If `p1` has a different y-coordinate from `p0`, it is ignored.
    fn draw_hor_line(&mut self, p0: impl ToCoords, p1: impl ToCoords, brightness: usize) {
        let (x0, y) = p0.to_coords_i32();
        let (x1, _) = p1.to_coords_i32();

        let (x0, x1) = min_and_max(x0, x1);
        for x in x0..=x1 {
            self.set_with_brightness((x, y), brightness);
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

fn min_and_max(a: i32, b: i32) -> (i32, i32) {
    if a < b {
        (a, b)
    } else {
        (b, a)
    }
}
