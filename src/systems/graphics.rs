use hecs::{Entity, World};
use raylib::prelude::*;

use crate::{chip::*, graphics::*, pin::*, Name, Position};

use super::utils::*;

pub fn pins(app: &World, draw_handle: &mut RaylibDrawHandle) {
    for (e, (pin, _)) in app.query::<(&Pin, &Position)>().iter() {
        let pos = get_global_position(app, e).unwrap();

        draw_pin2(draw_handle, pos, pin);
    }
}

pub fn connections(app: &World, draw_handle: &mut RaylibDrawHandle) {
    for (_, connection) in app.query::<&PinConnection>().iter() {
        let p1 = get_global_position(app, connection.0).unwrap();
        let p2 = get_global_position(app, connection.1).unwrap();

        let state = app.get::<Pin>(connection.0).unwrap().1;

        let color = if state { Color::RED } else { Color::BLACK };

        draw_connection(draw_handle, p1, p2, 2.0, color);
    }
}

pub fn connection_builders(app: &World, draw_handle: &mut RaylibDrawHandle, mouse: Entity) {
    let mouse_pos = app.get::<Position>(mouse).unwrap().0;

    for (_, builder) in app.query::<&PinConnectionBuilder>().iter() {
        if let Some(from) = builder.from {
            let p1 = get_global_position(app, from).unwrap();
            let p2 = match builder.to {
                None => mouse_pos,
                Some(e) => get_global_position(app, e).unwrap(),
            };

            draw_connection(draw_handle, p1, p2, 1.0, Color::BLUE);
        }
    }
}

pub fn chips(app: &World, draw_handle: &mut RaylibDrawHandle) {
    for (e, (pos, chip)) in app.query::<(&Position, &Chip)>().iter() {
        let pos = pos.0;
        let height = compute_chip_height(&chip);

        draw_handle.draw_rectangle_v(pos, Vector2::new(CHIP_WIDTH, height), Color::ORANGE);
        if let Ok(name) = app.get::<Name>(e) {
            draw_handle.draw_text(
                &name.0,
                pos.x as i32 + 10,
                pos.y as i32 + 5,
                20,
                Color::BLACK,
            );
        }
    }
}
