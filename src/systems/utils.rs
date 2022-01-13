use hecs::{Entity, World};
use raylib::{consts::MouseButton, math::Vector2, RaylibHandle};

use crate::{
    chip::{Chip, CHIP_WIDTH},
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

pub fn collide_chip(app: &World, point: Vector2, chip: Entity) -> bool {
    let pos = get_global_position(app, chip).unwrap();
    if point.x < pos.x || point.x > pos.x + CHIP_WIDTH {
        return false;
    }
    if point.y < pos.y || point.y > pos.y + compute_chip_height(&app.get::<Chip>(chip).unwrap()) {
        return false;
    }
    true
}

pub fn is_chip_released(app: &World, rl: &RaylibHandle, mouse: Entity, chip: Entity) -> bool {
    let mouse_pos = app.get::<Position>(mouse).unwrap().0;

    if !collide_chip(app, mouse_pos, chip) {
        return false;
    }

    rl.is_mouse_button_released(MouseButton::MOUSE_LEFT_BUTTON)
}

pub fn is_chip_pressed(app: &World, rl: &RaylibHandle, mouse: Entity, chip: Entity) -> bool {
    let mouse_pos = app.get::<Position>(mouse).unwrap().0;

    if !collide_chip(app, mouse_pos, chip) {
        return false;
    }

    rl.is_mouse_button_pressed(MouseButton::MOUSE_LEFT_BUTTON)
}

pub fn compute_chip_height(chip: &Chip) -> f32 {
    chip.inputs.len().max(chip.outputs.len()) as f32 * PIN_RADIUS * 3.0
}
