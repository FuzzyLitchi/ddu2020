use crate::{input, components, systems};

use ggez::graphics::*;
use ggez_goodies::{Point2, Vector2};
use specs::{self, world::Builder, WorldExt};

// The game world. Every entity lives in here.
pub struct World {
    input_state: input::State,
    specs_world: specs::World, // Contains components and entities
    dispatcher: specs::Dispatcher<'static, 'static>, // Contains systems

    // tmp
    mesh: Mesh,
}

impl World {
    pub fn new(ctx: &mut ggez::Context) -> Self {
        let mut specs_world = specs::WorldExt::new();
        components::register_components(&mut specs_world);

        let mut dispatcher = systems::register_systems();
        dispatcher.setup(&mut specs_world);

        let mut the_world = Self {
            // resources: store,
            input_state: input::State::new(),
            specs_world,
            dispatcher,
            mesh: Mesh::new_circle(ctx, DrawMode::fill(), Point2::new(0.0, 0.0), 32.0, 1.0, Color::new(0.0, 0.0, 1.0, 1.0)).unwrap()
        };

        // Make a test entity.
        the_world
            .specs_world
            .create_entity()
            .with(components::Position(Point2::new(0.0, 0.0)))
            .with(components::Motion {
                velocity: Vector2::new(1.0, 1.0),
            })
            .with(components::Renderable)
            .build();

        the_world
    }

    pub fn update(&mut self, _ctx: &mut ggez::Context) {
        self.dispatcher.dispatch(&mut self.specs_world);
    }

    pub fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult<()> {
        use components::{Renderable, Position};
        use specs::{Join, ReadStorage};

        let (renderable, position): (ReadStorage<Renderable>, ReadStorage<Position>) = self.specs_world.system_data();
        
        for (_renderable, position) in (&renderable, &position).join() {
            draw(ctx, &self.mesh, DrawParam::default().dest(position.0))?;
        }

        Ok(())
    }
}
