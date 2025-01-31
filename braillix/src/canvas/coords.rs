/// Types implementing `ToCoords` can be used as parameters for defining
/// shapes to draw onto a `Canvas`.
pub trait ToCoords: Copy {
    fn to_coords_i32(&self) -> (i32, i32);
    fn to_coords_f64(&self) -> (f64, f64);
}

impl ToCoords for (usize, usize) {
    #[inline]
    fn to_coords_i32(&self) -> (i32, i32) {
        (self.0 as i32, self.1 as i32)
    }

    #[inline]
    fn to_coords_f64(&self) -> (f64, f64) {
        (self.0 as f64, self.1 as f64)
    }
}

impl ToCoords for (i32, i32) {
    #[inline]
    fn to_coords_i32(&self) -> (i32, i32) {
        (self.0, self.1)
    }

    #[inline]
    fn to_coords_f64(&self) -> (f64, f64) {
        (self.0 as f64, self.1 as f64)
    }
}

impl ToCoords for (f64, f64) {
    #[inline]
    fn to_coords_i32(&self) -> (i32, i32) {
        (self.0.round() as i32, self.1.round() as i32)
    }

    #[inline]
    fn to_coords_f64(&self) -> (f64, f64) {
        (self.0, self.1)
    }
}

pub(super) trait ToDisplay {
    fn to_display(&self, dim: (usize, usize)) -> Option<(usize, usize)>;
}

impl ToDisplay for (usize, usize) {
    #[inline]
    fn to_display(&self, dim: (usize, usize)) -> Option<(usize, usize)> {
        (self.0 < dim.0 && self.1 < dim.1).then_some(*self)
    }
}

impl ToDisplay for (i32, i32) {
    #[inline]
    fn to_display(&self, dim: (usize, usize)) -> Option<(usize, usize)> {
        ((
            self.0.clamp(0, dim.0 as i32),
            (self.1.clamp(0, dim.1 as i32)),
        ) == *self)
            .then_some((self.0 as usize, self.1 as usize))
    }
}

impl ToDisplay for (f64, f64) {
    #[inline]
    fn to_display(&self, dim: (usize, usize)) -> Option<(usize, usize)> {
        ((
            self.0.clamp(0.0, dim.0 as f64),
            (self.1.clamp(0.0, dim.1 as f64)),
        ) == *self)
            .then_some((self.0.round() as usize, self.1.round() as usize))
    }
}
