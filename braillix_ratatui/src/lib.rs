use braillix::{canvas::Canvas, display::Display};
use ratatui::{buffer::Buffer, layout::Rect, text::Line, widgets::Widget};

pub trait ToWidget {
    type Output<'a>: Widget
    where
        Self: 'a;

    /// Returns an object implementing `ratatui::widgets::Widget` for rendering.
    ///
    /// # Example
    ///
    /// ```edition2021
    /// use braillix::canvas::{Canvas, ShapeStyle};
    /// use braillix_ratatui::ToWidget;
    /// use ratatui::prelude::*;
    ///
    /// let mut canvas = Canvas::with_dot_size(4, 4);
    /// let mut buf = Buffer::empty(Rect::new(0, 0, 2, 1));
    ///
    /// canvas.draw_rect((0, 0), 4, 4, ShapeStyle::Outlined);
    /// canvas.widget().render(buf.area, &mut buf);
    ///
    /// let expected = Buffer::with_lines(vec!["⣏⣹"]);
    /// assert_eq!(buf, expected);
    /// ```
    fn widget(&self) -> Self::Output<'_>;
}

/// Newtype for rendering a `braillix::Display` as a ratatui widget.
pub struct DisplayWidget<'a>(&'a Display);
impl Widget for DisplayWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let d_width = self.0.output_width() as u16;
        let d_height = self.0.output_height() as u16;

        let render_area = area.intersection(Rect::new(area.left(), area.top(), d_width, d_height));
        let (rw, rh) = (render_area.width as usize, render_area.height as usize);
        for (y, s) in self.0.lines().take(rh).enumerate() {
            Line::from(s.chars().take(rw).collect::<String>()).render(
                Rect::new(
                    render_area.left(),
                    render_area.top() + y as u16,
                    render_area.width,
                    1,
                ),
                buf,
            );
        }
    }
}
impl ToWidget for Display {
    type Output<'a> = DisplayWidget<'a>;

    fn widget(&self) -> Self::Output<'_> {
        DisplayWidget(self)
    }
}

/// Newtype for rendering a `braillix::Canvas` as a ratatui widget.
pub struct CanvasWidget<'a>(&'a Canvas);
impl Widget for CanvasWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        DisplayWidget(&self.0.display).render(area, buf);
    }
}
impl ToWidget for Canvas {
    type Output<'a> = CanvasWidget<'a>;

    fn widget(&self) -> Self::Output<'_> {
        CanvasWidget(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use braillix::canvas::ShapeStyle;

    #[test]
    fn render() {
        // standard usage
        let mut canvas = Canvas::with_dot_size(4, 4);
        canvas.draw_rect((0, 0), 4, 4, ShapeStyle::Outlined);

        let mut buf = Buffer::empty(Rect::new(0, 0, 2, 1));
        canvas.widget().render(buf.area, &mut buf);

        let expected = Buffer::with_lines(vec!["⣏⣹"]);
        assert_eq!(buf, expected);

        // cropping
        let mut small_buf = Buffer::empty(Rect::new(0, 0, 1, 1));
        canvas.widget().render(small_buf.area, &mut small_buf);

        let expected = Buffer::with_lines(vec!["⣏"]);
        assert_eq!(small_buf, expected);
    }
}