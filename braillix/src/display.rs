//! # display
//!
//! This module provides `Display`, a low-level buffer for braille drawing. It is
//! a stateful object holding an array of indices into a compile-time generated
//! unicode lookup table.
//!
//! For more advanced drawing, see `Canvas`.

use std::fmt;

/// A low-level buffer for braille drawing.
#[derive(Debug, PartialEq, Eq)]
pub struct Display {
    width: usize,
    height: usize,
    cells: Vec<u8>,
}

impl Display {
    /// Creates a new `Display` with the given dot size.
    ///
    /// For now, the width must be a multiple of 2 and the height
    /// must be a multiple of 4 so that it maps nicely to braille
    /// characters.
    ///
    /// # Panics
    /// This function panics if the width and height do not meet
    /// the above constraints.
    pub fn with_dot_size(width: usize, height: usize) -> Self {
        // TODO: implement non-snapped grid sizes by using partial cells
        assert!(width % 2 == 0, "width must be a multiple of 2");
        assert!(height % 4 == 0, "height must be a multiple of 4");
        Self::with_output_size(width / 2, height / 4)
    }

    /// Creates a new `Display` with the given output (character) dimensions.
    pub fn with_output_size(width: usize, height: usize) -> Self {
        let cells = vec![0; width * height];
        Self {
            width,
            height,
            cells,
        }
    }

    /// Gets the width of the display in dots.
    pub fn dot_width(&self) -> usize {
        self.width * 2
    }

    /// Gets the height of the display in dots.
    pub fn dot_height(&self) -> usize {
        self.height * 4
    }

    /// Gets the size (width, height) of the display in dots.
    pub fn dot_size(&self) -> (usize, usize) {
        (self.dot_width(), self.dot_height())
    }

    /// Gets the width of the display in characters.
    pub fn output_width(&self) -> usize {
        self.width
    }

    /// Gets the height of the display in characters.
    pub fn output_height(&self) -> usize {
        self.height
    }

    /// Gets the size (width, height) of the display in characters.
    pub fn output_size(&self) -> (usize, usize) {
        (self.width, self.height)
    }

    /// Returns an iterator over the lines of the display as `String`s.
    pub fn lines(&self) -> DisplayLines<'_> {
        DisplayLines {
            display: self,
            index: 0,
        }
    }

    /// Checks if the given dot is set in the display.
    pub fn is_set(&self, x: usize, y: usize) -> bool {
        let ((cell_x, subcell_x), (cell_y, subcell_y)) = Self::parse_coord(x, y);

        let i = self.coord_to_index(cell_x, cell_y);
        braille_util::is_set(self.cells[i], subcell_x, subcell_y)
    }

    /// Sets the given dot in the display.
    pub fn set(&mut self, x: usize, y: usize) {
        let ((cell_x, subcell_x), (cell_y, subcell_y)) = Self::parse_coord(x, y);

        let i = self.coord_to_index(cell_x, cell_y);
        braille_util::set_coord(&mut self.cells[i], subcell_x, subcell_y);
    }

    /// Unsets the given dot in the display.
    pub fn unset(&mut self, x: usize, y: usize) {
        let ((cell_x, subcell_x), (cell_y, subcell_y)) = Self::parse_coord(x, y);

        let i = self.coord_to_index(cell_x, cell_y);
        braille_util::unset_coord(&mut self.cells[i], subcell_x, subcell_y);
    }

    /// Clears the display by unsetting all of its dots.
    pub fn clear(&mut self) {
        self.cells.fill(0);
    }

    /// Splits dot coordinates into the cell coordinates and sub-cell coordinates.
    fn parse_coord(x: usize, y: usize) -> ((usize, usize), (usize, usize)) {
        ((x / 2, x % 2), (y / 4, y % 4))
    }

    /// Turns cell coordinates into their corresponding internal array index.
    fn coord_to_index(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }
}

/// Iterator over the lines of braille text in the display.
pub struct DisplayLines<'a> {
    display: &'a Display,
    index: usize,
}

impl Iterator for DisplayLines<'_> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.display.height {
            return None;
        }

        let start = self.display.coord_to_index(0, self.index);
        let end = start + self.display.width;
        let line = self.display.cells[start..end]
            .iter()
            .map(|&i| braille_util::get_char(i))
            .collect();

        self.index += 1;
        Some(line)
    }
}

impl From<&Display> for String {
    fn from(value: &Display) -> Self {
        value.lines().collect::<Vec<_>>().join("\n")
    }
}

impl From<Display> for String {
    fn from(value: Display) -> Self {
        String::from(&value)
    }
}

impl fmt::Display for Display {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", String::from(self))
    }
}

/// Helper functions for working with the braille characters and their `u8` lookup table indices.
mod braille_util {
    // Lookup table generation code based on:
    // https://github.com/766F6964/dotdotdot/blob/master/src/renderer.c
    const BRAILLE_UNICODE_OFFSET: u32 = 0x2800;
    const TRANSFORMATION_MATRIX: [u32; 8] = [0x01, 0x02, 0x04, 0x40, 0x08, 0x10, 0x20, 0x80];

    const fn gen_braille_table() -> [char; 256] {
        let mut table = ['\0'; 256];

        let mut i = 0;
        while i < table.len() {
            let mut u = BRAILLE_UNICODE_OFFSET;

            let mut j = 0;
            while j < 8 {
                if i & (1 << j) != 0 {
                    u += TRANSFORMATION_MATRIX[j];
                }
                j += 1;
            }

            table[i] = char::from_u32(u).expect("logic error in lookup table generation");
            i += 1;
        }

        table
    }
    const LOOKUP_TABLE: [char; 256] = gen_braille_table();

    /// Indexes the compile-time generated lookup table for braille characters.
    ///
    /// 0b7654_3210 maps to:
    ///
    /// 0 4
    /// 1 5
    /// 2 6
    /// 3 7
    pub const fn get_char(i: u8) -> char {
        LOOKUP_TABLE[i as usize]
    }

    /// Checks if a given dot is set in `i`.
    pub fn is_set(i: u8, x: usize, y: usize) -> bool {
        let mask = 1 << (4 * x + y);
        i & mask != 0
    }

    /// Sets dots on an index ref according to a mask.
    pub fn set_mask(i: &mut u8, mask: u8) {
        *i |= mask;
    }

    /// Sets a braille dot on an index ref.
    ///
    /// # Bounds:
    /// - 0 <= x < 2
    /// - 0 <= y < 4
    pub fn set_coord(i: &mut u8, x: usize, y: usize) {
        let mask = 1 << (4 * x + y);
        set_mask(i, mask);
    }

    /// Unsets dots on an index ref according to a mask.
    pub fn unset_mask(i: &mut u8, mask: u8) {
        *i &= mask;
    }

    /// Unsets a braille dot on an index ref.
    ///
    /// # Bounds:
    /// - 0 <= x < 2
    /// - 0 <= y < 4
    pub fn unset_coord(i: &mut u8, x: usize, y: usize) {
        let mask = !(1 << (4 * x + y));
        unset_mask(i, mask);
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn braille_lookup() {
            assert_eq!(get_char(0b1110_1111), '⣷');
        }

        #[test]
        fn set() {
            let mut i: u8 = 0;
            set_mask(&mut i, 0b1110_1111);
            assert_eq!(get_char(i), '⣷');

            i = 15;
            assert_eq!(get_char(i), '⡇');

            set_coord(&mut i, 1, 2);
            assert_eq!(get_char(i), '⡧');
        }

        #[test]
        fn unset() {
            let mut i: u8 = 79;
            assert_eq!(get_char(i), '⡧');

            unset_coord(&mut i, 0, 1);
            unset_coord(&mut i, 0, 2);
            assert_eq!(get_char(i), '⡡');
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let display = Display::with_dot_size(4, 4);
        assert_eq!(
            display,
            Display {
                width: 2,
                height: 1,
                cells: vec![0, 0],
            }
        );
    }

    #[test]
    fn display_lines() {
        let mut display = Display::with_dot_size(4, 4);
        display.cells[0] = 0b1001_1111;
        display.cells[1] = 0b1111_1001;

        let lines: Vec<_> = display.lines().collect();
        assert_eq!(lines, vec!["⣏⣹"]);
    }

    #[test]
    fn set() {
        let mut display = Display::with_dot_size(8, 8);
        for i in 0..8 {
            display.set(i, i);
        }

        let lines: Vec<_> = display.lines().collect();
        assert_eq!(
            lines,
            vec![
                "⠑⢄⠀⠀", //
                "⠀⠀⠑⢄", //
            ]
        )
    }
}
