use hecs::{Entity, World};
use raylib::math::Vector2;

use crate::{pin::*, Name, Parent, Position};
pub const CHIP_WIDTH: f32 = 50.0;

pub type ChipEvaluator = fn(Vec<PinState>) -> Vec<PinState>;
pub struct Chip {
    pub inputs: Vec<Entity>,
    pub outputs: Vec<Entity>,
    pub evaluator: ChipEvaluator,
}
pub fn spawn_chip(
    app: &mut World,
    name: Option<String>,
    pos: Vector2,
    n_inputs: u8,
    n_outputs: u8,
    evaluator: ChipEvaluator,
) -> Entity {
    let chip = app.spawn((Position(pos),));

    let mut inputs: Vec<Entity> = Vec::new();
    let mut outputs: Vec<Entity> = Vec::new();

    for i in 0..n_inputs {
        let pin = spawn_pin(
            app,
            Vector2::new(0.0, PIN_RADIUS * 3.0 * i as f32),
            PinKind::Input,
        );

        app.insert(pin, (Parent(chip),)).unwrap();

        inputs.push(pin);
    }

    for i in 0..n_outputs {
        let pin = spawn_pin(
            app,
            Vector2::new(CHIP_WIDTH, PIN_RADIUS * 3.0 * i as f32),
            PinKind::Output,
        );
        app.insert(pin, (Parent(chip),)).unwrap();

        outputs.push(pin);
    }

    app.insert(
        chip,
        (Chip {
            inputs,
            outputs,
            evaluator,
        },),
    )
    .unwrap();
    if let Some(name) = name {
        app.insert(chip, (Name(name),)).unwrap();
    }
    chip
}
