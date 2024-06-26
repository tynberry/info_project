//! Sawblade logic.
use std::f32::consts::PI;

use hecs::{EntityBuilder, World};
use macroquad::prelude::*;

use crate::{
    basic::{
        fx::{FxManager, Particle},
        motion::{ChargeReceiver, KnockbackDealer, LinearTorgue, MaxVelocity, PhysicsMotion},
        render::Sprite,
        DamageDealer, Health, HitBox, HurtBox, Position, Rotation, Team,
    },
    player::Player,
    xp::BurstXpOnDeath,
};

use super::Enemy;

/// Health of a sawblade.
const FOLLOWER_HEALTH: f32 = 0.8;
/// Speed of a sawblade.
const FOLLOWER_SPEED: f32 = 240.0;
/// Acceleration towards player of a sawblade.
const FOLLOWER_SPEED_CHANGE: f32 = 400.0;
/// Mass of a sawblade.
const FOLLOWER_MASS: f32 = 4.0;

/// Size of a sawblade.
/// Affects Hurt/HitBox size.
const FOLLOWER_SIZE: f32 = 40.0;

/// Damage sawblade does on hit.
const FOLLOWER_DMG: f32 = 2.0;

/// Texture ID of neutral sawblade.
pub const FOLLOWER_TEX_NEUTRAL: &str = "follower";
/// Texture ID of positively charged sawblade.
pub const FOLLOWER_TEX_POSITIVE: &str = "follower_plus";
/// Texture ID of negatively charged sawblade.
pub const FOLLOWER_TEX_NEGATIVE: &str = "follower_negative";

/// Knockback force dealt on hit by a sawblade.
const FOLLOWER_KNOCKBACK: f32 = 150.0;

/// Xp dropped on sawblade's death.
const FOLLOWER_XP: u32 = 30;

/// Handles sawblade's logic.
#[derive(Clone, Copy, Default, Debug)]
pub struct Follower {
    /// Charge of the sawblade.
    /// 1 => positive
    /// 0 => neutral
    /// -1 => negative
    pub charge: i8,
}

//-----------------------------------------------------------------------------
//ENTITY CREATION
//-----------------------------------------------------------------------------

/// Creates a sawblade.
/// # Arguments
/// * `pos` - position of the sawblade
/// * `dir` - direction the sawblade is initially heading
/// * `charge` - charge of the sawblade, same as asteroids
pub fn create_follower(pos: Vec2, dir: Vec2, charge: i8) -> EntityBuilder {
    let mut builder = EntityBuilder::default();
    builder.add_bundle((
        Enemy,
        Follower { charge },
        Position { x: pos.x, y: pos.y },
        Rotation {
            angle: fastrand::f32() * 2.0 * PI,
        },
        LinearTorgue {
            speed: fastrand::f32() * 30.0 - 15.0,
        },
        PhysicsMotion {
            vel: dir * FOLLOWER_SPEED,
            mass: FOLLOWER_MASS,
        },
        Sprite {
            texture: match charge {
                -1 => FOLLOWER_TEX_NEGATIVE,
                0 => FOLLOWER_TEX_NEUTRAL,
                1 => FOLLOWER_TEX_POSITIVE,
                _ => unimplemented!("Charges different than -1,0,1 are not implemented!"),
            },
            scale: FOLLOWER_SIZE / 512.0,
            color: WHITE,
            z_index: 1,
        },
        Team::Enemy,
        HurtBox {
            radius: FOLLOWER_SIZE / 2.0 - 4.0,
        },
        HitBox {
            radius: FOLLOWER_SIZE / 2.0 - 4.0,
        },
        KnockbackDealer {
            force: FOLLOWER_KNOCKBACK,
        },
        DamageDealer { dmg: FOLLOWER_DMG },
        Health {
            max_hp: FOLLOWER_HEALTH,
            hp: FOLLOWER_HEALTH,
        },
        BurstXpOnDeath {
            amount: FOLLOWER_XP,
        },
        MaxVelocity {
            max_velocity: FOLLOWER_SPEED * 2.0,
        },
    ));

    if charge != 0 {
        builder.add(ChargeReceiver {
            multiplier: 10.0 * charge as f32,
        });
    };

    builder
}

//-----------------------------------------------------------------------------
//SYSTEM PART
//-----------------------------------------------------------------------------

/// AI of the sawblade.
///
/// Makes the sawblade attracted to the player.
pub fn follower_ai(world: &mut World, dt: f32) {
    //get player's position
    let (_, &player_pos) = world
        .query_mut::<&Position>()
        .with::<&Player>()
        .into_iter()
        .next()
        .unwrap();
    //update velocity
    for (_, (pos, vel)) in world
        .query_mut::<(&Position, &mut PhysicsMotion)>()
        .with::<&Follower>()
    {
        //speed up towards player
        let acceleration = vec2(player_pos.x - pos.x, player_pos.y - pos.y).normalize_or_zero()
            * FOLLOWER_SPEED_CHANGE
            * dt;
        vel.vel += acceleration;
        //clamp speed
        if vel.vel.length() > FOLLOWER_SPEED {
            vel.vel = vel.vel.normalize_or_zero() * FOLLOWER_SPEED;
        }
    }
}

/// Spawns sawblade's trail.
pub fn follower_fx(world: &mut World, fx: &mut FxManager) {
    for (_, (follower, pos)) in world.query_mut::<(&Follower, &Position)>() {
        fx.burst_particles(
            Particle {
                pos: vec2(pos.x, pos.y),
                vel: vec2(0.0, 0.0),
                life: 0.4,
                max_life: 0.4,
                min_size: 0.0,
                max_size: 4.0,
                color: match follower.charge {
                    1 => RED,
                    0 => GREEN,
                    -1 => Color::new(0.0, 1.0, 1.0, 1.0),
                    _ => {
                        unimplemented!("Followers do not support charges different than 0,1,-1")
                    }
                },
            },
            0.0,
            0.0,
            1,
        );
    }
}

/// Spawns particles on sawblade's death.
pub fn follower_death(world: &mut World, fx: &mut FxManager) {
    for (_, (follower, hp, pos)) in world.query_mut::<(&Follower, &Health, &Position)>() {
        if hp.hp <= 0.0 {
            //spawn random particles on destroy
            for i in 1..=2 {
                fx.burst_particles(
                    Particle {
                        pos: vec2(pos.x, pos.y),
                        vel: vec2(30.0 * i as f32, 0.0),
                        life: 1.0,
                        max_life: 1.0,
                        min_size: 0.0,
                        max_size: 12.0,
                        color: LIGHTGRAY,
                    },
                    14.0,
                    2.0 * PI,
                    4 * i,
                );
            }
            fx.burst_particles(
                Particle {
                    pos: vec2(pos.x, pos.y),
                    vel: vec2(10.0, 0.0),
                    life: 1.0,
                    max_life: 1.0,
                    min_size: 0.0,
                    max_size: 15.0,
                    color: match follower.charge {
                        1 => RED,
                        0 => GREEN,
                        -1 => Color::new(0.0, 1.0, 1.0, 1.0),
                        _ => {
                            unimplemented!("Followers do not support charges different than 0,1,-1")
                        }
                    },
                },
                5.0,
                2.0 * PI,
                5,
            );
        }
    }
}
