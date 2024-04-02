use std::f32::consts::PI;

use hecs::{CommandBuffer, EntityBuilder, World};
use macroquad::prelude::*;

use crate::{
    basic::{
        motion::{ChargeReceiver, PhysicsMotion},
        Health, HitEvent, HurtBox, Position, Team,
    },
    player::{self, Player},
};

const MAX_RADIUS: f32 = 3.0;
const RADIUS_COEFF: f32 = 0.1;
const MIN_RADIUS: f32 = 1.0;

const ATTRACTION_RADIUS: f32 = 100.0;
const ATTRACTION_FORCE: f32 = 200.0;

#[derive(Clone, Copy, Debug, Default)]
pub struct BurstXpOnDeath {
    pub amount: u32,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct XpOrb {
    pub amount: u32,
}

//-----------------------------------------------------------------------------
//ENTITY CREATION
//-----------------------------------------------------------------------------

pub fn create_orb(pos: Vec2, vel: Vec2, amount: u32) -> EntityBuilder {
    let mut builder = EntityBuilder::new();

    builder.add_bundle((
        Position { x: pos.x, y: pos.y },
        PhysicsMotion {
            vel,
            mass: 0.25 * amount as f32,
        },
        XpOrb { amount },
        HurtBox { radius: MAX_RADIUS },
        crate::basic::render::Circle {
            radius: MIN_RADIUS
                + (MAX_RADIUS - MIN_RADIUS) * (1.0 - 1.0 / (RADIUS_COEFF * amount as f32 + 1.0)),
            color: YELLOW,
            z_index: 0,
        },
        Team::Player,
    ));

    builder
}

//-----------------------------------------------------------------------------
//SYSTEM PART
//-----------------------------------------------------------------------------

pub fn xp_bursts(world: &mut World, cmd: &mut CommandBuffer) {
    for (_, (burst, pos, health)) in world.query_mut::<(&BurstXpOnDeath, &Position, &Health)>() {
        let pos = vec2(pos.x, pos.y);
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

pub fn xp_attraction(world: &mut World, dt: f32) {
    //find player
    let (_, &player_pos) = world
        .query_mut::<&Position>()
        .with::<&Player>()
        .into_iter()
        .next()
        .unwrap();

    for (_, (pos, vel)) in world
        .query_mut::<(&Position, &mut PhysicsMotion)>()
        .with::<&XpOrb>()
    {
        let delta = vec2(player_pos.x - pos.x, player_pos.y - pos.y);
        if delta.length() <= ATTRACTION_RADIUS {
            vel.vel += ATTRACTION_FORCE * delta.normalize_or_zero() * dt;
            //friction the force in wrong directions
            let delta = delta.normalize_or_zero();
            let pro = vel.vel.dot(delta);
            let anti = vel.vel.perp_dot(delta);
            vel.vel = pro * delta + 0.5f32.powf(dt) * anti * delta.perp();
        }
    }
}

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
