use hecs::{Entity, World};
use raylib::{color::Color, math::Vector2, prelude::RaylibDraw};

pub mod chip;
pub mod graphics;
pub mod pin;
pub mod systems;

use chip::*;
use pin::*;

// GENERAL
// -------------------------------------
pub struct Name(String);
pub struct Position(Vector2);
pub struct Parent(Entity);
pub struct Mouse;

fn main() {
    let mut world = World::new();

    let mouse = world.spawn((Mouse, Position(Vector2::zero())));

    let _e1 = spawn_pin(&mut world, Vector2::new(100.0, 200.0), PinKind::Constant);
    let _e2 = spawn_pin(&mut world, Vector2::new(200.0, 300.0), PinKind::Input);

    let builder = world.spawn((PinConnectionBuilder {
        from: None,
        to: None,
    },));

    let _chip = spawn_chip(&mut world, None, Vector2::new(400.0, 100.0), 2, 1, |pins| {
        let result = pins.into_iter().reduce(|a, b| a && b).unwrap_or(false);

        vec![result]
    });

    let (mut rl, thread) = raylib::init()
        .size(800, 600)
        .title("Logic Gates")
        .msaa_4x()
        .build();
    rl.set_exit_key(None);

    while !rl.window_should_close() {
        systems::update::mouse(&mut world, &rl, mouse);
        systems::update::connection_builder(&mut world, &rl, mouse, builder);
        systems::update::toggle_pins(&mut world, &rl, mouse);
        systems::update::connection_state(&mut world);
        systems::update::evaluate_chips(&mut world);

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::WHITE);

        systems::graphics::chips(&world, &mut d);
        systems::graphics::pins(&world, &mut d);
        systems::graphics::connections(&world, &mut d);
        systems::graphics::connection_builders(&world, &mut d, mouse);
    }
}
