use hecs::{Entity, World};
use raylib::{consts::MouseButton, math::Vector2, RaylibHandle};

use crate::{
    pin::{Pin, PIN_RADIUS},
    Parent, Position,
};

pub fn collide_pin(app: &World, rl: &RaylibHandle, mouse: Entity, pin: Entity) -> bool {
    let pos = get_global_position(app, pin).unwrap();
    let mouse_pos = app.get::<Position>(mouse).unwrap().0;

    if !raylib::check_collision_point_circle(mouse_pos, pos, PIN_RADIUS) {
        return false;
    }

    rl.is_mouse_button_pressed(MouseButton::MOUSE_LEFT_BUTTON)
}

pub fn get_global_position(app: &World, entity: Entity) -> Option<Vector2> {
    let p = app.get::<Position>(entity).ok()?.0;

    match app.get::<Parent>(entity) {
        Err(_) => Some(p),
        Ok(x) => Some(p + get_global_position(app, x.0).unwrap_or(Vector2::zero())),
    }
}

pub fn toggle_pin(app: &mut World, pin: Entity) {
    let mut from = app.get_mut::<Pin>(pin).unwrap();
    from.1 = !from.1;
}
