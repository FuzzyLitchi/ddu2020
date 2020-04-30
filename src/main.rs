use ggez::{self, *};

use std::{env, path};

mod input;
mod world;
mod systems;
mod components;
mod resources;
mod sprites;
mod level;
mod physics;

fn main() {
    // ?
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        path::PathBuf::from("./resources")
    };
    println!("Resource dir: {:?}", resource_dir);

    // Create new context with some options
    let cb = ContextBuilder::new("ddu2020 eksamensprojekt", "Polly and Lukas")
        // .window_setup(conf::WindowSetup::default().title("game template"))
        .window_mode(conf::WindowMode::default().dimensions(800.0, 600.0))
        .add_resource_path(&resource_dir);

    // build context and event_loop
    let (ctx, event_loop) = &mut cb.build().unwrap();

    let state = &mut MainState::new(ctx, &resource_dir);
    match event::run(ctx, event_loop, state) {
        Ok(_) => println!("Exited cleanly."),
        Err(e) => println!("Error occured: {}", e)
    }
}

// This struct contains all the state relevant to our game.
struct MainState {
    world: world::World,
    input_binding: input::Binding,
}

impl MainState {
    fn new(ctx: &mut Context, _resource_path: &path::Path) -> Self {
        // let world = world::World::new(resource_path);
        // let mut scenestack = scenes::Stack::new(ctx, world);
        // let initial_scene = Box::new(scenes::level::LevelScene::new(ctx, &mut scenestack.world));
        // scenestack.push(initial_scene);

        // Self {
        //     input_binding: input::create_input_binding(),
        //     scenes: scenestack,
        // }
        Self {
            world: world::World::new(ctx),
            input_binding: input::create_input_binding(),
        }
    }
}

impl event::EventHandler for MainState {
    // This function is run as fast as possible on our system.
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        // This code should be under mouse event
        // {
        //     let pos = ggez::input::mouse::position(&ctx);
        //     let mut mouse_input = &mut *self.scenes.world.specs_world.fetch_mut::<input::MouseInput>();
        //     mouse_input.pos = pos.into();
        // }

        const DESIRED_FPS: u32 = 60;
        while timer::check_update_time(ctx, DESIRED_FPS) {
            self.world.update(ctx);
        }
        // self.scenes.world.resources.sync(ctx);

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::Color::from((0.0, 0.0, 0.4, 0.0)));
        self.world.draw(ctx)?;
        graphics::present(ctx)
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        keycode: event::KeyCode,
        _keymod: event::KeyMods,
        _repeat: bool,
    ) {
        if let Some(ev) = self.input_binding.resolve(keycode) {
            self.world.handle_input(ev, true);
        }
    }

    fn key_up_event(
        &mut self,
        _ctx: &mut Context,
        keycode: event::KeyCode,
        _keymod: event::KeyMods,
    ) {
        if let Some(ev) = self.input_binding.resolve(keycode) {
            self.world.handle_input(ev, false);
        }
    }

    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        button: event::MouseButton,
        _x: f32,
        _y: f32
    ) {
        if let Some(ev) = self.input_binding.resolve(button) {
            self.world.handle_input(ev, true);
        }
    }

    fn mouse_button_up_event(
        &mut self,
        _ctx: &mut Context,
        button: event::MouseButton,
        _x: f32,
        _y: f32
    ) {
        if let Some(ev) = self.input_binding.resolve(button) {
            self.world.handle_input(ev, false);
        }
    }

    fn mouse_motion_event(
        &mut self,
        _ctx: &mut Context,
        x: f32,
        y: f32,
        _dx: f32,
        _dy: f32
    ) {
        self.world.handle_mouse_motion(x, y);
    }
}