//! Projectile logic and creation.

use crate::basic::{
    motion::{ChargeDisable, ChargeReceiver, MaxVelocity, PhysicsMotion},
    render::Sprite,
    DamageDealer, HitEvent, HurtBox, Position, Team,
};
use hecs::{CommandBuffer, World};
use macroquad::prelude::*;

/// Marker of projectile entities.
#[derive(Clone, Copy, Debug)]
pub struct Projectile;

/// Defines the type of projectile to spawn.
#[derive(Clone, Debug)]
pub enum ProjectileType {
    Small {
        /// Sets the polarity of the projectile.
        /// x > 0 => positively charged
        /// x = 0 => neutral
        /// x < 0 => negatively charged
        charge: i8,
    },
    Medium {
        /// Sets the polarity of the projectile.
        /// x > 0 => positively charged
        /// x = 0 => neutral
        /// x < 0 => negatively charged
        charge: i8,
    },
}

/// Texture ID of positively charged small projectile.
pub const PROJ_SMALL_TEX_POS: &str = "proj_small_plus";
/// Texture ID of negatively charged small projectile.
pub const PROJ_SMALL_TEX_NEG: &str = "proj_small_minus";

/// Small projectiles's mass.
const PROJ_SMALL_MASS: f32 = 1.0;
/// Small projectiles's size.
/// Also influences Hurt/HitBox's size.
const PROJ_SMALL_SIZE: f32 = 2.0;
/// Small projectiles's charge force.
const PROJ_SMALL_CHARGE: f32 = 20.0;
/// Small projectiles's influence by charge multiplier.
const PROJ_SMALL_CHARGE_MULT: f32 = 0.4;
/// Small projectiles's charge full force radius.
const PROJ_SMALL_F_RADIUS: f32 = 100.0;
/// Small projectiles's charge zero force radius.
const PROJ_SMALL_RADIUS: f32 = 200.0;

/// Texture ID of positively charged medium projectile.
pub const PROJ_MED_TEX_POS: &str = "proj_medium_plus";
/// Texture ID of negatively charged medium projectile.
pub const PROJ_MED_TEX_NEG: &str = "proj_medium_minus";
/// Texture ID of non-charged medium projectile.
pub const PROJ_MED_TEX_NEUTRAL: &str = "proj_medium_neutral";

/// Medium projectiles's mass.
const PROJ_MED_MASS: f32 = 1.0;
/// Medium projectiles's size.
/// Also influences Hurt/HitBox's size.
const PROJ_MED_SIZE: f32 = 8.0;
/// Medium projectiles's charge force.
const PROJ_MED_CHARGE: f32 = 40.0;
/// Medium projectiles's influence by charge multiplier.
const PROJ_MED_CHARGE_MULT: f32 = 0.7;
/// Medium projectiles's charge full force radius.
const PROJ_MED_F_RADIUS: f32 = 120.0;
/// Medium projectiles's charge zero force radius.
const PROJ_MED_RADIUS: f32 = 250.0;

//-----------------------------------------------------------------------------
//CONSTRUCT ENTITY
//-----------------------------------------------------------------------------

/// Creates fully featured projetile.
/// # Arguments
/// - `pos` - position of the projectile
/// - `vel` - velocity of the projectile
/// - `dmg` - damage the projectile deals
/// - `team` - team the projectile belongs to
/// - `proj_type` - type of the projectile to spawn
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
    //ChargeSender,
    ChargeReceiver,
    ChargeDisable,
    PhysicsMotion,
    MaxVelocity,
) {
    //get properties from type
    let size = match proj_type {
        ProjectileType::Small { .. } => PROJ_SMALL_SIZE,
        ProjectileType::Medium { .. } => PROJ_MED_SIZE,
    };

    let mass = match proj_type {
        ProjectileType::Small { .. } => PROJ_SMALL_MASS,
        ProjectileType::Medium { .. } => PROJ_MED_MASS,
    };

    let texture = match proj_type {
        ProjectileType::Small { charge } => {
            if charge > 0 {
                PROJ_SMALL_TEX_POS
            } else {
                PROJ_SMALL_TEX_NEG
            }
        }
        ProjectileType::Medium { charge } => match charge {
            1 => PROJ_MED_TEX_POS,
            -1 => PROJ_MED_TEX_NEG,
            0 => PROJ_MED_TEX_NEUTRAL,
            _ => panic!("Charge can only be 0,1,-1"),
        },
    };

    let (charge, charge_mult, _f_radius, _n_radius) = match proj_type {
        ProjectileType::Small { charge } => (
            charge as f32 * PROJ_SMALL_CHARGE,
            PROJ_SMALL_CHARGE_MULT,
            PROJ_SMALL_F_RADIUS,
            PROJ_SMALL_RADIUS,
        ),
        ProjectileType::Medium { charge } => (
            charge as f32 * PROJ_MED_CHARGE,
            PROJ_MED_CHARGE_MULT,
            PROJ_MED_F_RADIUS,
            PROJ_MED_RADIUS,
        ),
    };

    // return the entire projectile entity
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
        //ChargeSender {
        //    force: charge,
        //    full_radius: f_radius,
        //    no_radius: n_radius,
        //},
        ChargeReceiver {
            multiplier: charge_mult
                * match charge {
                    x if x.abs() <= 0.01 => 0.0,
                    x => x.signum(),
                },
        },
        ChargeDisable { timer: 0.2 },
        PhysicsMotion { vel, mass },
        MaxVelocity {
            max_velocity: vel.length() * 2.0,
        },
    )
}

//-----------------------------------------------------------------------------
//SYSTEM PART
//-----------------------------------------------------------------------------
// Handles deletion of projectiles on collision with something they can hurt.
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
