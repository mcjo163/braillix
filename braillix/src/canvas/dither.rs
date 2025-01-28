// TODO: figure out if/how to allow user to choose the matrix size
const M: usize = 3;

/// Get the (2^M) x (2^M) Bayer threshold matrix value for (x, y).
pub fn threshold(x: usize, y: usize) -> usize {
    // TODO: is this any faster than just saving the matrix and doing a lookup?

    let n = 1 << M;
    let x = x % n;
    let y = y % n;

    // Implementation adapted from the black magic at:
    // https://bisqwit.iki.fi/story/howto/dither/jy/
    let mut v = 0;
    let mut mask = M.saturating_sub(1);
    let xc = x;
    let yc = x ^ y;
    let mut bit = 0;

    // This loop does an "interleave in reverse order" of x and (x ^ y).
    while bit < 2 * M {
        v |= ((xc >> mask) & 1) << bit;
        bit += 1;

        v |= ((yc >> mask) & 1) << bit;
        bit += 1;

        mask = mask.saturating_sub(1);
    }

    v
}

/// Get the highest threshold value for M.
pub const fn max_brightness() -> usize {
    (1 << M) * (1 << M)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rustfmt::skip]
    const BAYER_3: [[usize; 1 << 3]; 1 << 3] = [
        [ 0, 48, 12, 60,  3, 51, 15, 63],
        [32, 16, 44, 28, 35, 19, 47, 31],
        [ 8, 56,  4, 52, 11, 59,  7, 55],
        [40, 24, 36, 20, 43, 27, 39, 23],
        [ 2, 50, 14, 62,  1, 49, 13, 61],
        [34, 18, 46, 30, 33, 17, 45, 29],
        [10, 58,  6, 54,  9, 57,  5, 53],
        [42, 26, 38, 22, 41, 25, 37, 21],
    ];

    #[test]
    fn interleave_algorithm() {
        #![allow(clippy::needless_range_loop)]

        let dim3 = 1 << 3;
        for y in 0..dim3 {
            for x in 0..dim3 {
                assert_eq!(threshold(x, y), BAYER_3[y][x]);
            }
        }
    }
}
