use macroquad::prelude::*;

use crate::basic::{
    motion::{ChargeReceiver, ChargeSender, KnockbackDealer, LinearMotion, PhysicsMotion},
    render::Sprite,
    DamageDealer, DeleteOnWarp, Health, HitBox, HurtBox, Position, Team,
};

use super::Enemy;

const ASTEROID_HEALTH: f32 = 1.0;
const ASTEROID_SPEED: f32 = 50.0;
const ASTEROID_MASS: f32 = 18.0;

const ASTEROID_SIZE: f32 = 50.0;
const ASTEROID_SCALE: f32 = ASTEROID_SIZE / 512.0;

const ASTEROID_DMG: f32 = 0.5;

pub const ASTEROID_TEX_NEUTRAL: &str = "asteroid";
pub const ASTEROID_TEX_POSITIVE: &str = "asteroid_plus";
pub const ASTEROID_TEX_NEGATIVE: &str = "asteroid_negative";

const ASTEROID_FORCE: f32 = 750.0;
const ASTEROID_FORCE_F_RADIUS: f32 = 200.0;
const ASTEROID_FORCE_RADIUS: f32 = 350.0;

const ASTEROID_KNOCKBACK: f32 = 500.0;

//------------------------------------------------------------------------------
//ENTITY CREATION
//------------------------------------------------------------------------------

pub fn create_asteroid(
    pos: Vec2,
    dir: Vec2,
) -> (
    Enemy,
    Position,
    LinearMotion,
    Sprite,
    HitBox,
    HurtBox,
    Health,
    DamageDealer,
    Team,
    DeleteOnWarp,
    KnockbackDealer,
) {
    (
        Enemy,
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
    )
}

#[allow(clippy::type_complexity)]
pub fn create_charged_asteroid(
    pos: Vec2,
    dir: Vec2,
    charge: i8,
) -> (
    Enemy,
    Position,
    PhysicsMotion,
    Sprite,
    HitBox,
    HurtBox,
    Health,
    DamageDealer,
    Team,
    DeleteOnWarp,
    ChargeSender,
    ChargeReceiver,
    KnockbackDealer,
) {
    let texture = if charge > 0 {
        ASTEROID_TEX_POSITIVE
    } else {
        ASTEROID_TEX_NEGATIVE
    };

    (
        Enemy,
        Position { x: pos.x, y: pos.y },
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
    )
}
