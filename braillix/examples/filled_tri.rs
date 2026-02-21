use braillix::canvas::{geometry::Tri, Canvas, Style};

fn main() {
    let mut canvas = Canvas::with_dot_size(20, 20);

    canvas.draw(
        Tri::new((1, 1), (18, 11), (1, 18)),
        Style::filled_with_brightness_f64(0.5),
    );

    println!("{canvas}");
}
