use hecs::{Entity, World};
use raylib::prelude::*;

use crate::{chip::*, graphics::*, pin::*, Position};

use super::utils::*;

pub fn pins(app: &World, renderer: &mut Renderer) {
    for (e, (pin, _)) in app.query::<(&Pin, &Position)>().iter() {
        let pos = get_global_position(app, e).unwrap();

        renderer.draw_pin(pos, pin);
    }
}

pub fn connections(app: &World, renderer: &mut Renderer) {
    for (_, connection) in app.query::<&PinConnection>().iter() {
        let p1 = get_global_position(app, connection.0).unwrap();
        let p2 = get_global_position(app, connection.1).unwrap();

        let state = app.get::<Pin>(connection.0).unwrap().1;

        let color = if state { Color::RED } else { Color::BLACK };

        renderer.draw_connection(p1, p2, 2.0, color);
    }
}

pub fn connection_builders(app: &World, renderer: &mut Renderer, mouse: Entity) {
    let mouse_pos = app.get::<Position>(mouse).unwrap().0;

    for (_, builder) in app.query::<&PinConnectionBuilder>().iter() {
        if let Some(from) = builder.from {
            let p1 = get_global_position(app, from).unwrap();
            let p2 = match builder.to {
                None => mouse_pos,
                Some(e) => get_global_position(app, e).unwrap(),
            };

            renderer.draw_connection(p1, p2, 1.0, Color::BLUE);
        }
    }
}

pub fn chips(app: &World, renderer: &mut Renderer) {
    for (_, (pos, chip)) in app.query::<(&Position, &Chip)>().iter() {
        let pos = pos.0;

        renderer.draw_chip(pos, &chip);
    }
}
