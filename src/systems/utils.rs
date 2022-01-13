use hecs::{Entity, World};
use raylib::{consts::MouseButton, math::Vector2, RaylibHandle};

use crate::{
    pin::{Pin, PinKind, PIN_RADIUS},
    Parent, Position,
};

pub fn is_pin_pressed(app: &World, rl: &RaylibHandle, mouse: Entity, pin: Entity) -> bool {
    let mouse_pos = app.get::<Position>(mouse).unwrap().0;

    if !collide_pin(app, mouse_pos, pin) {
        return false;
    }

    rl.is_mouse_button_pressed(MouseButton::MOUSE_LEFT_BUTTON)
}

pub fn collide_pin(app: &World, point: Vector2, pin: Entity) -> bool {
    let pos = get_global_position(app, pin).unwrap();

    raylib::check_collision_point_circle(point, pos, PIN_RADIUS)
}

pub fn is_pin_released(app: &World, rl: &RaylibHandle, mouse: Entity, pin: Entity) -> bool {
    let mouse_pos = app.get::<Position>(mouse).unwrap().0;

    if !collide_pin(app, mouse_pos, pin) {
        return false;
    }

    rl.is_mouse_button_released(MouseButton::MOUSE_LEFT_BUTTON)
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

pub fn can_connect(kind1: PinKind, kind2: PinKind) -> bool {
    match kind1 {
        PinKind::Input => kind2 != PinKind::Input,
        PinKind::Output | PinKind::Constant => kind2 == PinKind::Input,
    }
}
