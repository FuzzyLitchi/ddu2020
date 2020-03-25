use crate::{input, components};

use ggez_goodies::{Point2, Vector2};
use specs::{self, world::Builder, WorldExt};

// The game world. Every entity lives in here.
pub struct World {
    input_state: input::State,
    specs_world: specs::World, // Contains components and entities
    dispatcher: specs::Dispatcher<'static, 'static>, // Contains systems
}

impl World {
    pub fn new() -> Self {
        let mut specs_world = specs::WorldExt::new();
        components::register_components(&mut specs_world);

        let mut dispatcher = register_systems();
        dispatcher.setup(&mut specs_world);

        let mut the_world = Self {
            // resources: store,
            input_state: input::State::new(),
            specs_world,
            dispatcher
        };

        // Make a test entity.
        the_world
            .specs_world
            .create_entity()
            .with(components::Position(Point2::new(0.0, 0.0)))
            .with(components::Motion {
                velocity: Vector2::new(1.0, 1.0),
                acceleration: Vector2::new(0.0, 0.0),
            })
            .build();

        the_world
    }

    pub fn update(&mut self, _ctx: &mut ggez::Context) {
        self.dispatcher.dispatch(&mut self.specs_world);
    }
}

// Create systems TODO: move into systems
fn register_systems() -> specs::Dispatcher<'static, 'static> {
    specs::DispatcherBuilder::new()
        // .with(MovementSystem, "sys_movement", &[])
        // .with(PlayerCharacterSystem::default(), "sys_player_character", &[])
        .build()
}