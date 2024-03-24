use hecs::{CommandBuffer, World};
use macroquad::prelude::*;

use crate::{
    basic::{
        motion::{ChargeReceiver, ChargeSender, KnockbackDealer, LinearMotion, PhysicsMotion},
        render::{Circle, Sprite},
        DamageDealer, DeleteOnWarp, Health, HitBox, HitEvent, HurtBox, Position, Team,
    },
    player::Player,
    projectile::{self, ProjectileType},
};

const ASTEROID_HEALTH: f32 = 1.0;
const ASTEROID_SPEED: f32 = 100.0;
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

const SHOOTER_HEALTH: f32 = 1.0;
const SHOOTER_SPEED: f32 = 100.0;
const SHOOTER_SIZE: f32 = 30.0;
const SHOOTER_DMG: f32 = 0.5;

const SHOOTER_FIRE_COOLDOWN: f32 = 1.5;
const SHOOTER_PROJ_SPEED: f32 = 80.0;

#[derive(Clone, Copy, Debug, Default)]
pub struct Enemy;

#[derive(Clone, Copy, Debug, Default)]
pub struct ShooterAI {
    pub fire_timer: f32,
}

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

pub fn create_shooter(
    pos: Vec2,
    dir: Vec2,
) -> (
    Enemy,
    Position,
    LinearMotion,
    Circle,
    HitBox,
    HurtBox,
    Health,
    DamageDealer,
    Team,
    DeleteOnWarp,
    ShooterAI,
) {
    (
        Enemy,
        Position { x: pos.x, y: pos.y },
        LinearMotion {
            vel: dir * SHOOTER_SPEED,
        },
        crate::basic::render::Circle {
            radius: SHOOTER_SIZE,
            color: PURPLE,
            z_index: 1,
        },
        HitBox {
            radius: SHOOTER_SIZE,
        },
        HurtBox {
            radius: SHOOTER_SIZE,
        },
        Health {
            max_hp: SHOOTER_HEALTH,
            hp: SHOOTER_HEALTH,
        },
        DamageDealer { dmg: SHOOTER_DMG },
        Team::Enemy,
        DeleteOnWarp,
        ShooterAI { fire_timer: 0.0 },
    )
}

//------------------------------------------------------------------------------
//SYSTEM PART
//------------------------------------------------------------------------------

pub fn health(world: &mut World, events: &mut World, cmd: &mut CommandBuffer) {
    //get enemy view
    let enemy_query = &mut world.query::<&mut Health>().with::<&Enemy>();
    let mut enemy_view = enemy_query.view();
    //get events concerning the player
    let hit_events = events.query_mut::<&HitEvent>().into_iter();
    for (_, event) in hit_events {
        //get the enemy
        let Some(enemy_hp) = enemy_view.get_mut(event.who) else {
            continue;
        };
        //get damage
        let Ok(damage) = world.get::<&DamageDealer>(event.by) else {
            continue;
        };
        //apply it
        enemy_hp.hp -= damage.dmg;
        //check for death
        if enemy_hp.hp <= 0.0 {
            //TODO DEATH
            //despawn for now
            cmd.despawn(event.who);
        }
    }
}

pub fn shooter_ai(world: &mut World, cmd: &mut CommandBuffer, dt: f32) {
    //cache player target
    let player_pos = world
        .query_mut::<&Position>()
        .with::<&Player>()
        .into_iter()
        .map(|(_, pos)| *pos)
        .next()
        .unwrap();
    //make all shooters target player
    for (_, (shooter, shooter_pos)) in world.query_mut::<(&mut ShooterAI, &Position)>() {
        //decrement fire cooldown
        shooter.fire_timer -= dt;
        //shoot if you can
        if shooter.fire_timer <= 0.0 {
            //reset timer
            shooter.fire_timer = SHOOTER_FIRE_COOLDOWN;
            //target
            let delta = vec2(player_pos.x - shooter_pos.x, player_pos.y - shooter_pos.y)
                .normalize_or_zero();
            //shoot!
            cmd.spawn(projectile::create_projectile(
                vec2(shooter_pos.x, shooter_pos.y),
                delta * SHOOTER_PROJ_SPEED,
                2.0,
                Team::Enemy,
                ProjectileType::Small { charge: 0 },
            ));
        }
    }
}
