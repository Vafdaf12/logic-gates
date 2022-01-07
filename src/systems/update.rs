use hecs::{Entity, World};
use raylib::{
    consts::{KeyboardKey, MouseButton},
    RaylibHandle,
};

use crate::{chip::*, pin::*, Position};

use super::utils::{collide_pin, toggle_pin};

pub fn connection_state(app: &mut World) {
    for (_, connection) in app.query::<&PinConnection>().iter() {
        let from = app.get::<Pin>(connection.0).unwrap().1;
        app.get_mut::<Pin>(connection.1).unwrap().1 = from;
    }
}
pub fn mouse(app: &mut World, rl: &RaylibHandle, mouse: Entity) {
    app.get_mut::<Position>(mouse).unwrap().0 = rl.get_mouse_position();
}

pub fn connection_builder(app: &mut World, rl: &RaylibHandle, mouse: Entity, builder: Entity) {
    let mut connection_builder = app.get_mut::<PinConnectionBuilder>(builder).unwrap();

    if !rl.is_key_down(KeyboardKey::KEY_LEFT_SHIFT) {
        return;
    }

    if let Some((entity, _)) = app
        .query::<(&Position, &Pin)>()
        .iter()
        .find(|(pin, _)| collide_pin(app, rl, mouse, *pin))
    {
        if connection_builder.from.is_none() {
            connection_builder.from = Some(entity);
        } else if connection_builder.to.is_none() && connection_builder.from.unwrap() != entity {
            connection_builder.to = Some(entity);
        }
    } else if rl.is_mouse_button_pressed(MouseButton::MOUSE_LEFT_BUTTON)
        && connection_builder.from.is_some()
    {
        connection_builder.from = None;
    }
    if let Some(connection) = connection_builder.build() {
        drop(connection_builder);
        app.spawn((connection,));
    }
}

pub fn toggle_pins(app: &mut World, rl: &RaylibHandle, mouse: Entity) {
    if rl.is_key_down(KeyboardKey::KEY_LEFT_SHIFT) {
        return;
    }
    let clicked: Vec<Entity> = app
        .query::<&Pin>()
        .into_iter()
        .filter(|(_, pin)| pin.0 == PinKind::Constant)
        .filter(|(e, _)| collide_pin(app, rl, mouse, *e))
        .map(|(e, ..)| e)
        .collect();

    for e in clicked.iter() {
        toggle_pin(app, *e);
    }
}

pub fn evaluate_chips(app: &mut World) {
    for (_, chip) in app.query::<&Chip>().iter() {
        let x: Vec<PinState> = chip
            .inputs
            .clone()
            .into_iter()
            .map(|e| app.get::<Pin>(e).unwrap())
            .map(|p| p.1)
            .collect();

        let mut y = (chip.evaluator)(x).into_iter();

        for mut pin in chip.outputs.iter().map(|e| app.get_mut::<Pin>(*e).unwrap()) {
            match y.next() {
                None => break,
                Some(s) => pin.1 = s,
            }
        }
    }
}
