use crate::basic::{
    motion::{ChargeReceiver, ChargeSender, PhysicsMotion},
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
    charge: f32,
    mass: f32,
) -> (
    Projectile,
    Position,
    Team,
    HurtBox,
    DamageDealer,
    Circle,
    ChargeSender,
    ChargeReceiver,
    PhysicsMotion,
) {
    (
        Projectile,
        Position { x: pos.x, y: pos.y },
        team,
        HurtBox { radius: size },
        DamageDealer { dmg },
        Circle {
            radius: size,
            color: GREEN,
            z_index: -1,
        },
        ChargeSender { charge },
        ChargeReceiver { charge },
        PhysicsMotion { vel, mass },
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
