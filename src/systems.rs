//! specs systems.
use crate::components::*;
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

// Create specs dispatcher with systems
pub fn register_systems() -> specs::Dispatcher<'static, 'static> {
    specs::DispatcherBuilder::new()
        .with(MovementSystem, "sys_movement", &[])
        .build()
}