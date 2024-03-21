use crate::basic::{
    motion::{Charge, LinearMotion, PhysicsMotion},
    render::Circle,
    DamageDealer, HitEvent, HurtBox, Position, Team,
};
use hecs::{CommandBuffer, World};
use macroquad::prelude::*;

#[derive(Clone, Copy, Debug)]
pub struct Projectile;

//-----------------------------------------------------------------------------
//CONSTRUCT ENTITY
//-----------------------------------------------------------------------------

pub fn create_projectile(
    pos: Vec2,
    vel: Vec2,
    size: f32,
    dmg: f32,
    team: Team,
) -> (
    Projectile,
    LinearMotion,
    Position,
    Team,
    HurtBox,
    DamageDealer,
    Circle,
    Charge,
    PhysicsMotion,
) {
    (
        Projectile,
        LinearMotion { vel },
        Position { x: pos.x, y: pos.y },
        team,
        HurtBox { radius: size },
        DamageDealer { dmg },
        Circle {
            radius: size,
            color: GREEN,
            z_index: -1,
        },
        Charge { charge: 1.0 },
        PhysicsMotion {
            vel: Vec2::ZERO,
            mass: 4.0,
        },
    )
}

//-----------------------------------------------------------------------------
//SYSTEM PART
//-----------------------------------------------------------------------------
pub fn on_hurt(world: &mut World, events: &mut World, cmd: &mut CommandBuffer) {
    for (proj_id, _) in world.query_mut::<&Projectile>() {
        for (_, event) in events.query_mut::<&HitEvent>() {
            if event.by == proj_id {
                cmd.despawn(proj_id);
            }
        }
    }
}
