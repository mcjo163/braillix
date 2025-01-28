use braillix::canvas::{Canvas, Style};

fn main() {
    let segment_width = 3;
    let segment_height = 24;
    let mut c = Canvas::with_dot_size(65 * segment_width + 1, segment_height);

    for b in 0..=64 {
        c.draw_rect(
            (b * segment_width, 0),
            segment_width,
            segment_height,
            Style::filled_with_brightness(b),
        );
    }

    println!("{c}");
}
