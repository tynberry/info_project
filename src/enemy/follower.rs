use std::f32::consts::PI;

use hecs::{EntityBuilder, World};
use macroquad::prelude::*;

use crate::{
    basic::{
        fx::{FxManager, Particle},
        motion::{KnockbackDealer, LinearTorgue, PhysicsMotion},
        render::Sprite,
        DamageDealer, Health, HitBox, HurtBox, Position, Rotation, Team,
    },
    player::Player,
};

use super::Enemy;

const FOLLOWER_HEALTH: f32 = 0.8;
const FOLLOWER_SPEED: f32 = 240.0;
const FOLLOWER_SPEED_CHANGE: f32 = 400.0;
const FOLLOWER_MASS: f32 = 4.0;

const FOLLOWER_SIZE: f32 = 40.0;

const FOLLOWER_DMG: f32 = 1.5;

pub const FOLLOWER_TEX_NEUTRAL: &str = "follower";
//pub const FOLLOWER_TEX_POSITIVE: &str = "follower_plus";
//pub const FOLLOWER_TEX_NEGATIVE: &str = "follower_negative";

const FOLLOWER_KNOCKBACK: f32 = 150.0;

#[derive(Clone, Copy, Default, Debug)]
pub struct Follower;

//-----------------------------------------------------------------------------
//ENTITY CREATION
//-----------------------------------------------------------------------------

pub fn create_follower(pos: Vec2, dir: Vec2) -> EntityBuilder {
    let mut builder = EntityBuilder::default();
    builder.add_bundle((
        Enemy,
        Follower,
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
            texture: FOLLOWER_TEX_NEUTRAL,
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
    ));
    builder
}

//-----------------------------------------------------------------------------
//SYSTEM PART
//-----------------------------------------------------------------------------

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

pub fn follower_death(world: &mut World, fx: &mut FxManager) {
    for (_, (hp, pos)) in world
        .query_mut::<(&Health, &Position)>()
        .with::<&Follower>()
    {
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
                    color: GREEN,
                },
                5.0,
                2.0 * PI,
                5,
            );
        }
    }
}
