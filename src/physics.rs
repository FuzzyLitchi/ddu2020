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
    fn cell_width() -> f64 { 35.0 }
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

pub fn handle_collision(entity_hb: Hitbox, wall_hb: Hitbox) -> Hitbox {
    // Get collision normal
    let normal = entity_hb.value.normal_from(&wall_hb.value);

    println!("{:?}", normal);

    // Apply collision normal to entity pos, thereby placing it outside of the collider.
    let new_pos = entity_hb.value.pos + normal.dir()*0.01;

    // Project velocity vector onto the orthogonal of the normal as to reset movement in the direction of the collider.
    let new_vec = {
        let a = entity_hb.vel.value;
        let b = v2(normal.dir().y, -normal.dir().x);

        let dot_product = a.x * b.x + a.y * b.y;
        
        dot_product * b
    };

    // Return new hitbox with updated data
    return entity_hb.value.shape.place(new_pos).moving(new_vec);
}