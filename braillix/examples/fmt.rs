use braillix::canvas::{Canvas, Style};

fn main() {
    let mut canvas = Canvas::with_dot_size(60, 60);

    canvas.draw_circle((30, 30), 24, Style::outlined());

    println!("{canvas}");
}
