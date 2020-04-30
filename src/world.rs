use rand::prelude::*;
use crate::{
    input,
    systems,
    components,
    resources,
    sprites,
    level,
    physics
};

use ggez::graphics::*;
use ggez_goodies::{Point2, Vector2};
use specs::{self, world::Builder, WorldExt};
use collider::geom::*;

// The game world. Every entity lives in here.
pub struct World {
    // ECS
    specs_world: specs::World, // Contains components and entities
    dispatcher: specs::Dispatcher<'static, 'static>, // Contains systems

    // Meshes for rendering
    sprites: Vec<Image>, // Sprites are loaded upen world initialization and
                         // aren't supposed to change after that
    square: Mesh, // Mesh for rendering rectangles
}

impl World {
    pub fn new(ctx: &mut ggez::Context) -> Self {
        // Create empty specs world
        let mut specs_world = specs::WorldExt::new();
        components::register_components(&mut specs_world);

        // Add input state
        specs_world.insert(input::State::new());

        let mut dispatcher = systems::register_systems();
        dispatcher.setup(&mut specs_world);

        // Add mesh for debug square rendering
        let square = Mesh::new_rectangle(
            ctx,
            DrawMode::fill(),
            Rect::new(0.0, 0.0, 1.0, 1.0),
            Color::new(1.0, 1.0, 1.0, 1.0)
        ).unwrap();

        // Add images for sprite rendering
        let sprites = sprites::load_sprites(ctx);

        let mut the_world = Self {
            // resources: store,
            specs_world,
            dispatcher,
            square,
            sprites,
        };

        // Make a test entity.
        the_world
            .specs_world
            .create_entity()
            .with(components::Position(Point2::new(0.0, 0.0)))
            .with(components::Motion {
                velocity: Vector2::new(1.0, 1.0),
            })
            .with(components::Renderable::SpriteId(sprites::SMILEY))
            .build();

        let mut rng = thread_rng();
        for _ in 0..100 {
            let x: f64 = rng.gen_range(0.0, 800.0);
            let y: f64 = rng.gen_range(0.0, 600.0);

            let entity = the_world
                .specs_world
                .create_entity()
                .with(components::Position(Point2::new(x as f32, y as f32)))
                .with(components::Motion {
                    velocity: Vector2::new(0.0, 0.0),
                })
                .with(components::Renderable::Rectangle {
                    w: 30.0,
                    h: 20.0,
                    color: ggez::graphics::Color::new(0.0, 0.0, 1.0, 1.0),
                })
                .with(components::Friendly::default())
                .build();
            
            let hitbox = Shape::rect(v2(30.0, 20.0)).place(v2(x, y)).still();
            physics::add_box_collider(entity, hitbox, false, &mut the_world.specs_world);
        }

        level::load_level(&"test.png", &mut the_world.specs_world);

        the_world
    }

    pub fn update(&mut self, _ctx: &mut ggez::Context) {
        // Run systems
        self.dispatcher.dispatch(&mut self.specs_world);
        
        // Update input state
        // This has to be last. Order is important for get_button_pressed and _released.
        self.specs_world.fetch_mut::<input::State>().update();
    }

    pub fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult<()> {
        use components::{Renderable, Position};
        use specs::{Join, ReadStorage};

        // We can draw every entity that has both a position and a renderable component
        let (renderable, position): (ReadStorage<Renderable>, ReadStorage<Position>) = self.specs_world.system_data();
        
        // .join() to make sure we only get entities that have both
        for (renderable, position) in (&renderable, &position).join() {
            match renderable {
                // Draw rectangle
                Renderable::Rectangle {w, h, color} => draw(
                    ctx,
                    &self.square,
                    DrawParam::default()
                        .dest(position.0)
                        .scale(Vector2::new(*w, *h))
                        .color(*color)
                )?,
                // Draw sprite
                Renderable::SpriteId(id) => draw(
                    ctx,
                    &self.sprites[*id],
                    DrawParam::default()
                        .dest(position.0)
                )?,
            }
        }

        // Render selection box
        let sel_box = self.specs_world.fetch::<Option<resources::SelectionBox>>();
        if let Some(sel_box) = &*sel_box {
            draw(
                ctx,
                &self.square,
                DrawParam::default()
                    .dest(sel_box.start)
                    .scale(Vector2::new(
                        sel_box.stop.x-sel_box.start.x,
                        sel_box.stop.y-sel_box.start.y,
                    ))
                    .color(ggez::graphics::Color::new(1.0, 1.0, 1.0, 0.1))
            )?
        }

        Ok(())
    }

    pub fn handle_input(&mut self, ev: input::Event, started: bool) {
        self.specs_world.fetch_mut::<input::State>()
            .update_effect(ev, started);
    }

    pub fn handle_mouse_motion(&mut self, x: f32, y: f32) {
        self.specs_world.fetch_mut::<input::State>()
            .update_mouse_position(x, y);
    }
}
