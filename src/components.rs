use ggez_goodies::{Point2, Vector2};

use specs::*;
use specs_derive::*;

// This file contains every component. Components are simply storage for some data,
// they only become useful when systems interact with them. However they are a useful
// abstraction for organizing our game, and therefor we use them.

/// A position in the game world.
#[derive(Clone, Debug, Component)]
#[storage(VecStorage)]
pub struct Position(pub Point2);

/// Motion in the game world.
#[derive(Clone, Debug, Component)]
#[storage(VecStorage)]
pub struct Motion {
    pub velocity: Vector2,
    pub acceleration: Vector2,
}

// We add every component to our specs world
pub fn register_components(specs_world: &mut World) {
    specs_world.register::<Position>();
    specs_world.register::<Motion>();
}
