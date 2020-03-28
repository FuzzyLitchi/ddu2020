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

pub struct FriendlySystem;

impl<'a> specs::System<'a> for FriendlySystem {
    type SystemData = (
        specs::ReadStorage<'a, Position>,
        specs::WriteStorage<'a, Motion>,
        specs::WriteStorage<'a, Friendly>,
        specs::Read<'a, input::State>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            pos,
            mut motion,
            mut friendly,
            input
        ) = data;
        
        if input.get_button_pressed(input::Button::Right) {
            // TODO limit this to selected enetities
            for friendly in (&mut friendly).join() {
                friendly.action = Action::Goto(input.mouse_position());
            }
        }

        // TODO: I could optimize this code to only run when Action::Goto changes or collisions and stuff
        for (pos, motion, friendly) in (&pos, &mut motion, &friendly).join() {
            match friendly.action {
                Action::Goto(target_pos) => {
                    let direction = (target_pos - pos.0).normalize();

                    motion.velocity = direction * WALK_SPEED;
                }
                _ => (),
            }
        }
    }
}

// Create specs dispatcher with systems
pub fn register_systems() -> specs::Dispatcher<'static, 'static> {
    specs::DispatcherBuilder::new()
        .with(MovementSystem, "movement", &[])
        .with(FriendlySystem, "friendly", &[])
        .build()
}