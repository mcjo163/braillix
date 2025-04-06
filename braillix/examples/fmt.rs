use braillix::canvas::{geometry::Circle, Canvas, Style};

fn main() {
    let mut canvas = Canvas::with_dot_size(60, 60);

    canvas.draw(Circle::new((30, 30), 24), Style::outlined());

    println!("{canvas}");
}
