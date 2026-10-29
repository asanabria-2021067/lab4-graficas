use crate::framebuffer::Framebuffer;
use crate::line::{line, Vec2};

/// Dibuja el contorno de un triángulo con tres llamadas a `line`.
pub fn triangle(fb: &mut Framebuffer, a: Vec2, b: Vec2, c: Vec2) {
    line(fb, a, b);
    line(fb, b, c);
    line(fb, c, a);
}
