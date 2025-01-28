use super::dither;

/// Drawing style. A shape's `outline` and `fill` modes can be independently
/// configured. Each field, if `Some`, indicates the dither brightness.
#[derive(Default)]
pub struct Style {
    pub outline: Option<usize>,
    pub fill: Option<usize>,
}

impl Style {
    /// Get a new `Style` object with nothing configured. Using this verbatim
    /// will result in nothing being drawn.
    pub fn none() -> Self {
        Self::default()
    }

    /// Get a new `Style` object with the fill set to full brightness.
    pub fn filled() -> Self {
        Self::none().fill_on()
    }

    /// Get a new `Style` object with the specified fill brightness.
    pub fn filled_with_brightness(brightness: usize) -> Self {
        Self::none().fill_brightness(brightness)
    }

    /// Get a new `Style` object with the specified fill brightness.
    pub fn filled_with_brightness_f64(brightness: f64) -> Self {
        Self::none().fill_brightness_f64(brightness)
    }

    /// Get a new `Style` object with the outline set to full brightness.
    pub fn outlined() -> Self {
        Self::none().outline_on()
    }

    /// Get a new `Style` object with the specified outline brightness.
    pub fn outlined_with_brightness(brightness: usize) -> Self {
        Self::none().outline_brightness(brightness)
    }

    /// Get a new `Style` object with the specified outline brightness.
    pub fn outlined_with_brightness_f64(brightness: f64) -> Self {
        Self::none().outline_brightness_f64(brightness)
    }

    /// Sets the fill (background) to full brightness (every dot enabled).
    pub fn fill_on(self) -> Self {
        Self {
            fill: Some(dither::max_brightness()),
            ..self
        }
    }

    /// Sets the raw brightness value (dither threshold) for the fill (background).
    pub fn fill_brightness(self, brightness: usize) -> Self {
        Self {
            fill: Some(brightness.min(dither::max_brightness())),
            ..self
        }
    }

    /// Sets the dither threshold for the fill based on a float.
    /// The input is clamped to the range `0.0..=1.0`.
    pub fn fill_brightness_f64(self, brightness: f64) -> Self {
        let brightness = brightness.clamp(0.0, 1.0);
        let b = dither::max_brightness() as f64 * brightness;
        Self {
            fill: Some(b.round() as usize),
            ..self
        }
    }

    /// Sets the fill (background) to lowest brightness. Every dot drawn will be cleared.
    pub fn fill_off(self) -> Self {
        Self {
            fill: Some(0),
            ..self
        }
    }

    /// Disables the fill. Dots inside the shape drawn will be left unchanged.
    pub fn no_fill(self) -> Self {
        Self { fill: None, ..self }
    }

    /// Sets the outline to full brightness (every dot enabled).
    pub fn outline_on(self) -> Self {
        Self {
            outline: Some(dither::max_brightness()),
            ..self
        }
    }

    /// Sets the raw brightness value (dither threshold) for the outline.
    pub fn outline_brightness(self, brightness: usize) -> Self {
        Self {
            outline: Some(brightness.min(dither::max_brightness())),
            ..self
        }
    }

    /// Sets the dither threshold for the outline based on a float.
    /// The input is clamped to the range `0.0..=1.0`.
    pub fn outline_brightness_f64(self, brightness: f64) -> Self {
        let brightness = brightness.clamp(0.0, 1.0);
        let b = dither::max_brightness() as f64 * brightness;
        Self {
            outline: Some(b.round() as usize),
            ..self
        }
    }

    /// Sets the outline to lowest brightness. Every dot drawn will be cleared.
    pub fn outline_off(self) -> Self {
        Self {
            outline: Some(0),
            ..self
        }
    }

    /// Disables the outline.
    pub fn no_outline(self) -> Self {
        Self {
            outline: None,
            ..self
        }
    }

    /// Return the "distinguishable outline" for the style. If the fill
    /// and the outline are both set to the same brightness, there is no
    /// difference between how they are rendered and it is a waste to
    /// draw the outline.
    pub(super) fn distinguishable_outline(&self) -> Option<usize> {
        self.outline.filter(|&o| self.fill.is_none_or(|f| f != o))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn distinguishable_outline() {
        let no_distinguishable_outline = Style {
            outline: Some(14),
            fill: Some(14),
        };
        assert_eq!(no_distinguishable_outline.distinguishable_outline(), None);

        let no_outline = Style {
            outline: None,
            fill: Some(16),
        };
        assert_eq!(no_outline.distinguishable_outline(), None);

        let no_fill = Style {
            outline: Some(5),
            fill: None,
        };
        assert_eq!(no_fill.distinguishable_outline(), Some(5));

        let distinguishable_outline = Style {
            outline: Some(16),
            fill: Some(5),
        };
        assert_eq!(distinguishable_outline.distinguishable_outline(), Some(16));
    }
}
