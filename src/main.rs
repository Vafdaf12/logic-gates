use graphics::Renderer;
use hecs::{Entity, World};
use raylib::{color::Color, math::Vector2};

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
pub struct Dragging(Vector2);

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
        systems::update::drag_chips(&mut world, &rl, mouse);
        systems::update::connection_builder(&mut world, &rl, mouse, builder);
        systems::update::toggle_pins(&mut world, &rl, mouse);
        systems::update::connection_state(&mut world);
        systems::update::evaluate_chips(&mut world);


        let mut renderer = Renderer::begin(&mut rl, &thread);
        renderer.clear(Color::WHITE);

        systems::graphics::chips(&world, &mut renderer);
        systems::graphics::pins(&world, &mut renderer);
        systems::graphics::connections(&world, &mut renderer);
        systems::graphics::connection_builders(&world, &mut renderer, mouse);
    }
}
