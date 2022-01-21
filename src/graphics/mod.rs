use raylib::{
    color::Color,
    math::Vector2,
    prelude::{RaylibDraw, RaylibDrawHandle}, RaylibHandle, RaylibThread,
};

use crate::{pin::*, chip::{Chip, CHIP_WIDTH}, systems::utils::compute_chip_height};

pub struct Renderer<'a> {
    handle: RaylibDrawHandle<'a>
}

impl<'a> Renderer<'a> {
    pub fn begin(rl: &'a mut RaylibHandle, thread: &RaylibThread) -> Renderer<'a> {
        Self {
            handle: rl.begin_drawing(thread)
        }
        
    }
}

impl Renderer<'_> {
    pub fn clear(&mut self, color: Color) {
        self.handle.clear_background(color);
    }
    pub fn draw_connection(
        &mut self,
        from: Vector2,
        to: Vector2,
        thick: f32,
        color: Color,
    ) {
        self.handle.draw_line_bezier(from, to, thick, color);
    }
    pub fn draw_pin(&mut self, org: Vector2, pin: &Pin) {
        let Pin(kind, state) = pin;
    
        let color = if *state { Color::RED } else { Color::BLACK };
    
        self.handle.draw_circle_v(org, PIN_RADIUS, color);
    
        match kind {
            PinKind::Constant => {
                self.handle.draw_circle_lines(org.x as i32, org.y as i32, PIN_RADIUS + 3.0, color)
            }
            _ => {}
        }
    }

    pub fn draw_chip(&mut self, pos: Vector2, chip: &Chip) {
        let height = compute_chip_height(&chip);

        self.handle.draw_rectangle_v(pos, Vector2::new(CHIP_WIDTH, height), Color::ORANGE);
    }
}
