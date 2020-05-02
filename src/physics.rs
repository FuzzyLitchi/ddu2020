use specs::{self, World, Entity};
use collider::{*, geom::{Vec2, v2}};

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

const SMALL: f64 = 0.01;

pub fn handle_wall_collision(entity: Profile, wall: Profile, collider: &mut ColliderRes) -> Vec<Profile> {
    // Get hitboxes
    let mut entity_hb = collider.get_hitbox(entity.id());
    let wall_hb = collider.get_hitbox(wall.id());

    // Get new resolved hitbox
    entity_hb = resolve_wall_collision(entity_hb, wall_hb);

    // Apply new hitbox
    let _ = collider.remove_hitbox(entity.id());
    collider.add_hitbox(entity, entity_hb);

    // Get new collisions
    collider.get_overlaps(entity.id())
}

pub fn resolve_wall_collision(entity_hb: Hitbox, wall_hb: Hitbox) -> Hitbox {
    // Get collision normal
    let normal = entity_hb.value.normal_from(&wall_hb.value);

    // Apply collision normal to entity pos, thereby placing it outside of the collider.
    let new_pos = entity_hb.value.pos + normal.dir()*SMALL;

    // Project velocity vector onto the orthogonal of the normal as to reset movement in the direction of the collider.
    let new_vec = {
        let b = v2(normal.dir().y, -normal.dir().x);

        dot_product(entity_hb.vel.value, b) * b
    };

    // Return new hitbox with updated data
    return entity_hb.value.shape.place(new_pos).moving(new_vec);
}

pub fn resolve_entity_collision(hb_1: Hitbox, hb_2: Hitbox) -> (Hitbox, Hitbox) {
    // Get collision normal
    let normal = hb_1.value.normal_from(&hb_2.value);

    // Apply collision normal to entity pos, thereby placing it outside of the collider.
    let new_pos_1 = hb_1.value.pos + normal.dir()*SMALL;
    let new_pos_2 = hb_2.value.pos - normal.dir()*SMALL;

    // Project velocity vector onto the orthogonal of the normal as to reset movement in the direction of the collider.
    let b = v2(normal.dir().y, -normal.dir().x);

    let new_vec_1 = dot_product(hb_1.vel.value, b) * b;
    let new_vec_2 = dot_product(hb_2.vel.value, b) * b;

    // Return new hitbox with updated data
    (
        hb_1.value.shape.place(new_pos_1).moving(new_vec_1),
        hb_2.value.shape.place(new_pos_2).moving(new_vec_2),
    )
}

pub fn dot_product(a: Vec2, b: Vec2) -> f64 {
    a.x * b.x + a.y * b.y
}