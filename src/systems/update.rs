use hecs::{Entity, World};
use raylib::{consts::MouseButton, RaylibHandle};

use crate::{chip::*, pin::*, Dragging, Position};

use super::utils::{
    can_connect, get_global_position, is_chip_pressed, is_chip_released, is_pin_pressed,
    is_pin_released, toggle_pin,
};

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

    if let Some((entity, _)) = app
        .query::<(&Position, &Pin)>()
        .iter()
        .find(|(pin, _)| is_pin_pressed(app, rl, mouse, *pin))
    {
        connection_builder.from = Some(entity);
    }

    if let Some((entity, _)) = app
        .query::<(&Position, &Pin)>()
        .iter()
        .find(|(pin, _)| is_pin_released(app, rl, mouse, *pin))
    {
        connection_builder.to = Some(entity);
    } else if rl.is_mouse_button_released(MouseButton::MOUSE_LEFT_BUTTON) {
        connection_builder.reset();
    }
    if let Some(connection) = connection_builder.build() {
        let from_kind = app.get::<Pin>(connection.0).unwrap().0;
        let to_kind = app.get::<Pin>(connection.1).unwrap().0;

        drop(connection_builder);
        if can_connect(from_kind, to_kind) {
            app.spawn((connection,));
        }
    }
}

pub fn toggle_pins(app: &mut World, rl: &RaylibHandle, mouse: Entity) {
    let clicked: Vec<Entity> = app
        .query::<&Pin>()
        .into_iter()
        .filter(|(_, pin)| pin.0 == PinKind::Constant)
        .filter(|(e, _)| is_pin_released(app, rl, mouse, *e))
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

pub fn drag_chips(app: &mut World, rl: &RaylibHandle, mouse: Entity) {
    let mouse_pos = get_global_position(app, mouse).unwrap();

    let dragging = app
        .query::<&Chip>()
        .into_iter()
        .map(|(e, _)| e)
        .find(|e| is_chip_pressed(app, rl, mouse, *e));

    if let Some(e) = dragging {
        let pos = get_global_position(app, e).unwrap();
        let delta = pos - mouse_pos;
        app.insert(e, (Dragging(delta),)).unwrap();
    }

    let not_dragging: Vec<Entity> = app
        .query::<&Chip>()
        .with::<Dragging>()
        .into_iter()
        .map(|(e, _)| e)
        .filter(|e| is_chip_released(app, rl, mouse, *e))
        .collect();

    for &elem in not_dragging.iter() {
        app.remove_one::<Dragging>(elem).unwrap();
    }

    for (_, (pos, delta)) in app
        .query_mut::<(&mut Position, &Dragging)>()
        .with::<Chip>()
        .into_iter()
    {
        *pos = Position(mouse_pos + delta.0);
    }
}
