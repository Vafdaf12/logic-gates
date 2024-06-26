use hecs::{Entity, World};
use raylib::math::Vector2;

use crate::Position;

pub type PinState = bool;
pub struct Pin(pub PinKind, pub PinState);
pub struct PinConnection(pub Entity, pub Entity);

pub const PIN_RADIUS: f32 = 5.0;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum PinKind {
    Input,
    Output,
    Constant,
}

pub struct PinConnectionBuilder {
    pub from: Option<Entity>,
    pub to: Option<Entity>,
}

impl PinConnectionBuilder {
    pub fn build(&mut self) -> Option<PinConnection> {
        let from = self.from?;
        let to = self.to?;

        self.reset();

        if from == to {
            return None;
        }
        println!("Connections");

        Some(PinConnection(from, to))
    }

    pub fn reset(&mut self) {
        self.from = None;
        self.to = None;
    }

    pub fn is_pending(&self) -> bool {
        self.from.is_some()
    }
}

pub fn spawn_pin(app: &mut World, pos: Vector2, kind: PinKind) -> Entity {
    app.spawn((Pin(kind, false), Position(pos)))
}
