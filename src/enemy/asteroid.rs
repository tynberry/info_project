use std::f32::consts::PI;

use hecs::{CommandBuffer, EntityBuilder, World};
use macroquad::prelude::*;

use crate::basic::{
    fx::{FxManager, Particle},
    motion::{
        ChargeReceiver, ChargeSender, KnockbackDealer, LinearMotion, LinearTorgue, PhysicsMotion,
    },
    render::Sprite,
    DamageDealer, DeleteOnWarp, Health, HitBox, HurtBox, Position, Rotation, Team,
};

use super::{charged::create_supercharged_asteroid, Enemy};

//ASTEROID STATS

pub(super) const ASTEROID_HEALTH: f32 = 1.0;
pub(super) const ASTEROID_SPEED: f32 = 50.0;
pub(super) const ASTEROID_MASS: f32 = 18.0;

pub(super) const ASTEROID_SIZE: f32 = 50.0;
pub(super) const ASTEROID_SCALE: f32 = ASTEROID_SIZE / 512.0;

pub(super) const ASTEROID_DMG: f32 = 1.5;

pub const ASTEROID_TEX_NEUTRAL: &str = "asteroid";
pub const ASTEROID_TEX_POSITIVE: &str = "asteroid_plus";
pub const ASTEROID_TEX_NEGATIVE: &str = "asteroid_negative";

pub(super) const ASTEROID_FORCE: f32 = 750.0;
pub(super) const ASTEROID_FORCE_F_RADIUS: f32 = 200.0;
pub(super) const ASTEROID_FORCE_RADIUS: f32 = 350.0;

pub(super) const ASTEROID_KNOCKBACK: f32 = 500.0;

//BIG ASTEROID STATS

const BIG_ASTEROID_HEALTH: f32 = 2.0;
const BIG_ASTEROID_SPEED: f32 = 45.0;
const BIG_ASTEROID_MASS: f32 = 30.0;

const BIG_ASTEROID_SIZE: f32 = 200.0;
const BIG_ASTEROID_SCALE: f32 = BIG_ASTEROID_SIZE / 512.0;

const BIG_ASTEROID_DMG: f32 = 3.0;

pub const BIG_ASTEROID_TEX_POSITIVE: &str = "asteroid_big_plus";
pub const BIG_ASTEROID_TEX_NEGATIVE: &str = "asteroid__big_minus";

const BIG_ASTEROID_FORCE: f32 = 950.0;
const BIG_ASTEROID_FORCE_F_RADIUS: f32 = 250.0;
const BIG_ASTEROID_FORCE_RADIUS: f32 = 400.0;

const BIG_ASTEROID_KNOCKBACK: f32 = 700.0;

#[derive(Clone, Copy, Debug)]
pub struct Asteroid;

#[derive(Clone, Copy, Debug)]
pub struct BigAsteroid;

//------------------------------------------------------------------------------
//ENTITY CREATION
//------------------------------------------------------------------------------

pub fn create_asteroid(pos: Vec2, dir: Vec2) -> EntityBuilder {
    let mut builder = EntityBuilder::new();
    builder.add_bundle((
        Enemy,
        Asteroid,
        Position { x: pos.x, y: pos.y },
        LinearMotion {
            vel: dir * ASTEROID_SPEED,
        },
        Sprite {
            texture: ASTEROID_TEX_NEUTRAL,
            scale: ASTEROID_SCALE,
            color: WHITE,
            z_index: 0,
        },
        HitBox {
            radius: ASTEROID_SIZE / 2.0 - 8.0,
        },
        HurtBox {
            radius: ASTEROID_SIZE / 2.0 - 8.0,
        },
        Health {
            max_hp: ASTEROID_HEALTH,
            hp: ASTEROID_HEALTH,
        },
        DamageDealer { dmg: ASTEROID_DMG },
        Team::Enemy,
        DeleteOnWarp,
        KnockbackDealer {
            force: ASTEROID_KNOCKBACK,
        },
    ));
    builder
}

#[allow(clippy::type_complexity)]
pub fn create_charged_asteroid(pos: Vec2, dir: Vec2, charge: i8) -> EntityBuilder {
    let texture = if charge > 0 {
        ASTEROID_TEX_POSITIVE
    } else {
        ASTEROID_TEX_NEGATIVE
    };

    let mut builder = EntityBuilder::default();

    builder.add_bundle((
        Enemy,
        Asteroid,
        Position { x: pos.x, y: pos.y },
        Rotation {
            angle: fastrand::f32() * 2.0 * PI,
        },
        LinearTorgue {
            speed: fastrand::f32() * 1.0 - 0.50,
        },
        PhysicsMotion {
            vel: dir * ASTEROID_SPEED,
            mass: ASTEROID_MASS,
        },
        Sprite {
            texture,
            scale: ASTEROID_SCALE,
            color: WHITE,
            z_index: 0,
        },
        HitBox {
            radius: ASTEROID_SIZE / 2.0,
        },
    ));
    builder.add_bundle((
        HurtBox {
            radius: ASTEROID_SIZE / 2.0,
        },
        Health {
            max_hp: ASTEROID_HEALTH,
            hp: ASTEROID_HEALTH,
        },
        DamageDealer { dmg: ASTEROID_DMG },
        Team::Enemy,
        DeleteOnWarp,
        ChargeSender {
            force: ASTEROID_FORCE * charge as f32,
            full_radius: ASTEROID_FORCE_F_RADIUS,
            no_radius: ASTEROID_FORCE_RADIUS,
        },
        ChargeReceiver {
            multiplier: charge as f32,
        },
        KnockbackDealer {
            force: ASTEROID_KNOCKBACK,
        },
    ));
    builder
}

#[allow(clippy::type_complexity)]
pub fn create_big_asteroid(pos: Vec2, dir: Vec2, charge: i8) -> EntityBuilder {
    let texture = if charge > 0 {
        BIG_ASTEROID_TEX_POSITIVE
    } else {
        BIG_ASTEROID_TEX_NEGATIVE
    };

    let mut builder = EntityBuilder::default();
    builder.add_bundle((
        Enemy,
        BigAsteroid,
        Position { x: pos.x, y: pos.y },
        Rotation {
            angle: fastrand::f32() * 2.0 * PI,
        },
        LinearTorgue {
            speed: fastrand::f32() * 1.0 - 0.50,
        },
        PhysicsMotion {
            vel: dir * BIG_ASTEROID_SPEED,
            mass: BIG_ASTEROID_MASS,
        },
        Sprite {
            texture,
            scale: BIG_ASTEROID_SCALE,
            color: WHITE,
            z_index: 0,
        },
        HitBox {
            radius: BIG_ASTEROID_SIZE / 2.0 - 15.0,
        },
        HurtBox {
            radius: BIG_ASTEROID_SIZE / 2.0 - 15.0,
        },
        Health {
            max_hp: BIG_ASTEROID_HEALTH,
            hp: BIG_ASTEROID_HEALTH,
        },
        DamageDealer {
            dmg: BIG_ASTEROID_DMG,
        },
        Team::Enemy,
        DeleteOnWarp,
    ));
    builder.add_bundle((
        ChargeSender {
            force: BIG_ASTEROID_FORCE * charge as f32,
            full_radius: BIG_ASTEROID_FORCE_F_RADIUS,
            no_radius: BIG_ASTEROID_FORCE_RADIUS,
        },
        ChargeReceiver {
            multiplier: charge as f32,
        },
        KnockbackDealer {
            force: BIG_ASTEROID_KNOCKBACK,
        },
    ));
    builder
}

//------------------------------------------------------------------------------
//SYSTEM PART
//------------------------------------------------------------------------------

pub fn asteroid_death(world: &mut World, fx: &mut FxManager) {
    for (_, (health, pos)) in world
        .query_mut::<(&Health, &Position)>()
        .with::<&Asteroid>()
    {
        //check if it is dead
        if health.hp <= 0.0 {
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
        }
    }
}

pub fn big_asteroid_death(world: &mut World, cmd: &mut CommandBuffer, fx: &mut FxManager) {
    for (_, (health, pos, phys, charge)) in world
        .query::<(&Health, &Position, &PhysicsMotion, &ChargeSender)>()
        .with::<&BigAsteroid>()
        .into_iter()
    {
        //check if it is dead
        if health.hp <= 0.0 {
            //spawn many smaller asteroids of the same charge
            for i in 0..8 {
                let off =
                    Vec2::from_angle(PI / 2.0 * (i as f32) + if i >= 4 { PI / 4.0 } else { 0.0 })
                        .rotate(Vec2::X)
                        * ASTEROID_SIZE
                        * 1.3
                        * if i >= 4 { 1.25 } else { 1.0 };

                let dir =
                    Vec2::from_angle(PI / 2.0 * (i as f32) + if i >= 4 { PI / 4.0 } else { 0.0 })
                        .rotate(Vec2::X)
                        + phys.vel / BIG_ASTEROID_SPEED;

                //let charge = big_charge.force.signum() as i8;
                let charge = if i >= 4 { -1 } else { 1 } * charge.force.signum() as i8;

                if i < 4 {
                    create_supercharged_asteroid(vec2(off.x + pos.x, off.y + pos.y), dir, charge)(
                        world, cmd,
                    );
                } else {
                    cmd.spawn(
                        create_charged_asteroid(vec2(off.x + pos.x, off.y + pos.y), dir, charge)
                            .build(),
                    );
                }
            }
            //spawn random particles on destroy
            for i in 1..5 {
                fx.burst_particles(
                    Particle {
                        pos: vec2(pos.x, pos.y),
                        vel: vec2(45.0 * i as f32, 0.0),
                        life: 1.0,
                        max_life: 1.0,
                        min_size: 0.0,
                        max_size: 20.0,
                        color: LIGHTGRAY,
                    },
                    30.0,
                    2.0 * PI,
                    8 * i,
                );
            }
        }
    }
}
