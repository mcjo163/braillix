//! # canvas
//!
//! This module provides `Canvas` and related types. It uses a `Display`
//! internally and provides an abstraction for drawing shapes and lines.

use std::fmt;

use crate::display::Display;

mod coords;
use coords::{ToCoords, ToDisplay};

mod dither;

pub mod geometry;

mod style;
pub use style::Style;

/// Types implementing `Rasterize` can be drawn onto a `Canvas`.
pub trait Rasterize {
    /// Draw `self` onto the `Canvas` with the specified `Style`.
    fn rasterize_onto(&self, canvas: &mut Canvas, style: Style);
}

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

    /// Draw to the `Canvas` using the object's `Rasterize` implementation.
    pub fn draw(&mut self, object: impl Rasterize, style: Style) {
        object.rasterize_onto(self, style);
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
