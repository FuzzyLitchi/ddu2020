use specs::{self, World, Entity};
use collider::{*, geom::{Shape, v2}};

use crate::components::BoxCollider;

pub type ColliderRes = Collider<Profile>;

#[derive(Copy, Clone, Debug)]
pub struct Profile {
    pub id: specs::world::Index,
    pub wall: bool,
}

impl HbProfile for Profile {
    fn id(&self) -> HbId {
        self.id as HbId
    }

    fn can_interact(&self, _other: &Profile) -> bool { true }
    fn cell_width() -> f64 { 4.0 }
    fn padding() -> f64 { 0.01 }
}

pub fn add_box_collider(entity: Entity, hitbox: Hitbox, wall: bool, world: &mut World) {
    let mut collider = world.fetch_mut::<Option<ColliderRes>>();
    let collider = collider.as_mut().unwrap();

    let profile = Profile {
        id:  entity.id(),
        wall,
    };

    collider.add_hitbox(profile, hitbox);

    let mut box_collider: specs::WriteStorage<BoxCollider> = world.system_data();
    box_collider.insert(entity, BoxCollider(entity.id() as HbId)).unwrap();
}