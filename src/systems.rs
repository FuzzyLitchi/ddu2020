//! specs systems.
use crate::components::*;
use crate::input;
use specs::{self, Join};

pub struct MovementSystem;

impl<'a> specs::System<'a> for MovementSystem {
    type SystemData = (
        specs::WriteStorage<'a, Position>,
        specs::ReadStorage<'a, Motion>,
    );

    fn run(&mut self, (mut pos, motion): Self::SystemData) {
        // The `.join()` combines multiple components,
        // so we only access those entities which have
        // both of them.
        for (pos, motion) in (&mut pos, &motion).join() {
            pos.0 += motion.velocity;
        }
    }
}

pub struct MouseSystem;

impl<'a> specs::System<'a> for MouseSystem {
    type SystemData = (
        specs::WriteStorage<'a, Position>,
        specs::ReadStorage<'a, MouseTeleport>,
        specs::Read<'a, input::State>,
    );

    fn run(&mut self, (mut pos, mouse_tele, input): Self::SystemData) {
        if !input.get_button_down(input::Button::Left) {
            return;
        }

        for (pos, _) in (&mut pos, &mouse_tele).join() {
            pos.0 = input.mouse_position();
        }
    }
}

// Create specs dispatcher with systems
pub fn register_systems() -> specs::Dispatcher<'static, 'static> {
    specs::DispatcherBuilder::new()
        .with(MovementSystem, "sys_movement", &[])
        .with(MouseSystem, "mouse_tele_movement", &[])
        .build()
}