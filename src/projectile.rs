use crate::basic::{
    motion::{ChargeReceiver, ChargeSender, PhysicsMotion},
    render::Sprite,
    DamageDealer, HitEvent, HurtBox, Position, Team,
};
use hecs::{CommandBuffer, World};
use macroquad::prelude::*;

#[derive(Clone, Copy, Debug)]
pub struct Projectile;

#[derive(Clone, Debug)]
pub enum ProjectileType {
    Small { charge: i8 },
}

pub const PROJ_SMALL_TEX_POS: &str = "proj_small_plus";
pub const PROJ_SMALL_TEX_NEG: &str = "proj_small_minus";

const PROJ_SMALL_MASS: f32 = 1.0;
const PROJ_SMALL_SIZE: f32 = 2.0;
const PROJ_SMALL_CHARGE: f32 = 20.0;
const PROJ_SMALL_CHARGE_MULT: f32 = 0.4;
const PROJ_SMALL_F_RADIUS: f32 = 100.0;
const PROJ_SMALL_RADIUS: f32 = 200.0;

//-----------------------------------------------------------------------------
//CONSTRUCT ENTITY
//-----------------------------------------------------------------------------

pub fn create_projectile(
    pos: Vec2,
    vel: Vec2,
    dmg: f32,
    team: Team,
    proj_type: ProjectileType,
) -> (
    Projectile,
    Position,
    Team,
    HurtBox,
    DamageDealer,
    Sprite,
    ChargeSender,
    ChargeReceiver,
    PhysicsMotion,
) {
    //get properties from type
    let size = match proj_type {
        ProjectileType::Small { .. } => PROJ_SMALL_SIZE,
    };

    let mass = match proj_type {
        ProjectileType::Small { .. } => PROJ_SMALL_MASS,
    };

    let texture = match proj_type {
        ProjectileType::Small { charge } => {
            if charge > 0 {
                PROJ_SMALL_TEX_POS
            } else {
                PROJ_SMALL_TEX_NEG
            }
        }
    };

    let (charge, charge_mult, f_radius, n_radius) = match proj_type {
        ProjectileType::Small { charge } => (
            charge as f32 * PROJ_SMALL_CHARGE,
            PROJ_SMALL_CHARGE_MULT,
            PROJ_SMALL_F_RADIUS,
            PROJ_SMALL_RADIUS,
        ),
    };

    (
        Projectile,
        Position { x: pos.x, y: pos.y },
        team,
        HurtBox { radius: size },
        DamageDealer { dmg },
        Sprite {
            texture,
            scale: 1.0,
            color: WHITE,
            z_index: -1,
        },
        ChargeSender {
            force: charge,
            full_radius: f_radius,
            no_radius: n_radius,
        },
        ChargeReceiver {
            multiplier: charge_mult * charge.signum(),
        },
        PhysicsMotion { vel, mass },
    )
}

//-----------------------------------------------------------------------------
//SYSTEM PART
//-----------------------------------------------------------------------------
pub fn on_hurt(world: &mut World, events: &mut World, cmd: &mut CommandBuffer) {
    for (proj_id, _) in world.query_mut::<&Projectile>() {
        for (_, event) in events.query_mut::<&HitEvent>() {
            //did it hurt?
            if !event.can_hurt {
                continue;
            }
            //despawn myself
            if event.by == proj_id {
                cmd.despawn(proj_id);
                //don't read other events
                break;
            }
        }
    }
}
