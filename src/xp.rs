//! Xp orbs logic and creation

use std::f32::consts::PI;

use hecs::{CommandBuffer, EntityBuilder, World};
use macroquad::prelude::*;

use crate::{
    basic::{motion::PhysicsMotion, Health, HitEvent, HurtBox, Position, Team, Wrapped},
    player::Player,
};

/// Distance at which the orb is absorbed into the player.
const COLLECT_RADIUS: f32 = 10.0;
/// Max radius of the Xp orb.
const MAX_RADIUS: f32 = 3.0;
/// Coefficient of the hyperbolic relation ship used to determine the size of the orbs.
const RADIUS_COEFF: f32 = 0.1;
/// Min radius of the Xp orb.
const MIN_RADIUS: f32 = 1.0;

/// Distance from the player the orb is attracted at.
const ATTRACTION_RADIUS: f32 = 300.0;
/// Speed at which the orb is attracted.
const ATTRACTION_SPEED: f32 = 100.0;
/// Multiplier of the speed every second the orb is attracted.
/// Diminishes when player out of the attraction range.
/// Multiplicative.
const ATTRACTION_MULT_PER_SEC: f32 = 0.8;

/// Component that spawns xp orbs on entities death (hp <= 0.0).
#[derive(Clone, Copy, Debug, Default)]
pub struct BurstXpOnDeath {
    /// Total Xp that should be enclosed in the spawned xp orbs.
    pub amount: u32,
}

/// Xp orb component.
/// Gives Xp to player and is attracted by them.
#[derive(Clone, Copy, Debug, Default)]
pub struct XpOrb {
    /// Amount of xp this orb contains.
    pub amount: u32,
    /// Current speed multiplier of attraction speed.
    pub follow_mult: f32,
}

//-----------------------------------------------------------------------------
//ENTITY CREATION
//-----------------------------------------------------------------------------

/// Create a xp orb entity.
/// # Arguments
/// * `pos` - position of the orb
/// * `vel` - velocity of the orb
/// * `amount` - how much xp is in the orb
pub fn create_orb(pos: Vec2, vel: Vec2, amount: u32) -> EntityBuilder {
    let mut builder = EntityBuilder::new();

    builder.add_bundle((
        Position { x: pos.x, y: pos.y },
        PhysicsMotion {
            vel,
            mass: 0.25 * amount as f32,
        },
        XpOrb {
            amount,
            follow_mult: 0.0,
        },
        HurtBox {
            radius: COLLECT_RADIUS,
        },
        crate::basic::render::Circle {
            radius: MIN_RADIUS
                + (MAX_RADIUS - MIN_RADIUS) * (1.0 - 1.0 / (RADIUS_COEFF * amount as f32 + 1.0)),
            color: YELLOW,
            z_index: 0,
        },
        Team::Player,
        Wrapped,
    ));

    builder
}

//-----------------------------------------------------------------------------
//SYSTEM PART
//-----------------------------------------------------------------------------

/// Handles xp orb spawning on death of `BurstXpOnDeath` entites.
pub fn xp_bursts(world: &mut World, cmd: &mut CommandBuffer) {
    for (_, (burst, pos, health)) in world.query_mut::<(&BurstXpOnDeath, &Position, &Health)>() {
        //get spawning position
        let pos = vec2(pos.x, pos.y);
        //is the entity dead?
        if health.hp <= 0.0 {
            //spawn xp's if dead
            let mut big_xp = burst.amount / 2;
            let mut rest_xp = burst.amount - big_xp;
            while big_xp > 0 {
                //cannot emit large enough XP orbs
                if big_xp < 10 {
                    rest_xp += big_xp;
                    break;
                }
                //emit large xp orbs
                big_xp -= 10;
                let angle = fastrand::f32() * 2.0 * PI;
                let speed = fastrand::f32() * 20.0 + 5.0;
                cmd.spawn(
                    create_orb(pos, Vec2::from_angle(angle).rotate(Vec2::X) * speed, 10).build(),
                );
            }
            //emit rest XP
            while rest_xp > 0 {
                let amount = fastrand::u32(1..=5).min(rest_xp);
                rest_xp -= amount;
                let angle = fastrand::f32() * 2.0 * PI;
                let speed = fastrand::f32() * 30.0 + 10.0;
                cmd.spawn(
                    create_orb(pos, Vec2::from_angle(angle).rotate(Vec2::X) * speed, amount)
                        .build(),
                );
            }
        }
    }
}

/// Attracts `XpOrb` entites to the player, if in range.
pub fn xp_attraction(world: &mut World, dt: f32) {
    //find player
    let (_, &player_pos) = world
        .query_mut::<&Position>()
        .with::<&Player>()
        .into_iter()
        .next()
        .unwrap();

    for (_, (pos, vel, orb)) in world.query_mut::<(&Position, &mut PhysicsMotion, &mut XpOrb)>() {
        let delta = vec2(player_pos.x - pos.x, player_pos.y - pos.y);
        if delta.length() <= ATTRACTION_RADIUS {
            vel.vel = ATTRACTION_SPEED * delta.normalize_or_zero() * (1.0 + orb.follow_mult);
            orb.follow_mult += dt * ATTRACTION_MULT_PER_SEC;
        } else {
            orb.follow_mult -= dt * ATTRACTION_MULT_PER_SEC;
            if orb.follow_mult < 0.0 {
                orb.follow_mult = 0.0;
            }
        }
        //orb friction
        vel.vel *= 0.7_f32.powf(dt);
    }
}

/// Absorbs the xp orbs into player when in range.
pub fn xp_absorbtion(world: &mut World, events: &mut World, cmd: &mut CommandBuffer) {
    //find player
    let mut player_query = world.query::<&mut Player>();
    let (player_id, player) = player_query.iter().next().unwrap();
    //check events for collisions
    for (_, hit_event) in events.query_mut::<&HitEvent>() {
        //is the one hit a player?
        if hit_event.who != player_id {
            continue;
        }
        //is the orb who hit a xp orb?
        let Ok(orb) = world.get::<&XpOrb>(hit_event.by) else {
            continue;
        };

        //add the xp and DIE
        player.xp += orb.amount;
        cmd.despawn(hit_event.by);
    }
}
