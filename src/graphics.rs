use raylib::{
    color::Color,
    math::Vector2,
    prelude::{RaylibDraw, RaylibDrawHandle},
};

use crate::pin::*;

pub fn draw_connection(
    handle: &mut RaylibDrawHandle,
    from: Vector2,
    to: Vector2,
    thick: f32,
    color: Color,
) {
    handle.draw_line_bezier(from, to, thick, color);
}

pub fn draw_pin2(handle: &mut RaylibDrawHandle, org: Vector2, pin: &Pin) {
    let Pin(kind, state) = pin;

    let color = if *state { Color::RED } else { Color::BLACK };

    handle.draw_circle_v(org, PIN_RADIUS, color);

    match kind {
        PinKind::Constant => {
            handle.draw_circle_lines(org.x as i32, org.y as i32, PIN_RADIUS + 3.0, color)
        }
        _ => {}
    }
}
