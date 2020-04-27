use ggez_goodies::{Point2, Vector2};

use specs::*;
use specs_derive::*;

use crate::sprites::SpriteId;

// This file contains every component. Components are simply storage for some data,
// they only become useful when systems interact with them. However they are a useful
// abstraction for organizing our game, and therefor we use them.

// A position in the game world.
#[derive(Clone, Debug, Component)]
#[storage(VecStorage)]
pub struct Position(pub Point2);

// Motion in the game world.
#[derive(Clone, Debug, Component)]
#[storage(VecStorage)]
pub struct Motion {
    pub velocity: Vector2,
}

// A tag to enable redering for the entity
#[derive(Clone, Debug, Component)]
#[storage(VecStorage)]
pub enum Renderable {
    Rectangle {w: f32, h: f32},
    SpriteId(SpriteId),
}

// Friendly character
#[derive(Clone, Debug, Default, Component)]
#[storage(VecStorage)]
pub struct Friendly {
    pub selected: bool,
    pub action: Action
}

pub const WALK_SPEED: f32 = 3.0;

#[derive(Clone, Debug)]
pub enum Action {
    Standby,
    Goto(Point2),
    //Attack(EntityId)??
}

impl Default for Action {
    fn default() -> Self {
        return Action::Standby;
    }
}

// We add every component to our specs world
pub fn register_components(specs_world: &mut World) {
    specs_world.register::<Position>();
    specs_world.register::<Motion>();
    specs_world.register::<Renderable>();
    specs_world.register::<Friendly>();
}
