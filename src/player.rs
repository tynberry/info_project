use std::f32::consts::PI;

use hecs::World;
use macroquad::prelude::*;

use crate::{
    basic::{
        fx::{FxManager, Particle},
        motion::{ChargeReceiver, ChargeSender, PhysicsMotion},
        render::Sprite,
        DamageDealer, Health, HitBox, HitEvent, Position, Rotation, Team, Wrapped,
    },
    projectile::{self, ProjectileType},
    world_mouse_pos, SPACE_HEIGHT, SPACE_WIDTH,
};

const PLAYER_ACCEL: f32 = 600.0;
const PLAYER_MASS: f32 = 10.0;

const PLAYER_CHARGE_FORCE: f32 = 200.0;
const PLAYER_CHARGE_FULL_RADIUS: f32 = 150.0;
const PLAYER_CHARGE_RADIUS: f32 = 300.0;

const PLAYER_MAX_BASE_HP: f32 = 10.0;
const PLAYER_BASE_HP_REGEN: f32 = 0.3;

const PLAYER_FIRE_COOLDOWN: f32 = 0.15;
const PLAYER_INVUL_COOLDOWN: f32 = 1.0;

pub const PLAYER_TEX_POSITIVE: &str = "player_plus";
pub const PLAYER_TEX_NEGATIVE: &str = "player_negative";

const PLAYER_SIZE: f32 = 30.0;

#[derive(Debug)]
pub struct Player {
    fire_timer: f32,

    invul_timer: f32,

    polarity: i8,

    dead_burst: bool,

    pub xp: u32,
}

impl Player {
    pub fn new() -> Self {
        Self {
            fire_timer: 0.0,
            invul_timer: 0.0,

            polarity: 1,

            dead_burst: false,

            xp: 0,
        }
    }
}

//-----------------------------------------------------------------------------
//ENTITY GEN
//-----------------------------------------------------------------------------

pub fn new_entity() -> (
    Player,
    Position,
    PhysicsMotion,
    Rotation,
    Health,
    HitBox,
    Team,
    Wrapped,
    Sprite,
    ChargeReceiver,
    ChargeSender,
) {
    (
        Player::new(),
        Position {
            x: SPACE_WIDTH / 2.0,
            y: SPACE_HEIGHT / 2.0,
        },
        PhysicsMotion {
            vel: Vec2::ZERO,
            mass: PLAYER_MASS,
        },
        Rotation::default(),
        Health {
            hp: PLAYER_MAX_BASE_HP,
            max_hp: PLAYER_MAX_BASE_HP,
        },
        HitBox { radius: 7.0 },
        Team::Player,
        Wrapped,
        Sprite {
            texture: PLAYER_TEX_POSITIVE,
            scale: PLAYER_SIZE / 512.0,
            color: WHITE,
            z_index: 0,
        },
        ChargeReceiver { multiplier: 0.2 },
        ChargeSender {
            force: PLAYER_CHARGE_FORCE,
            full_radius: PLAYER_CHARGE_FULL_RADIUS,
            no_radius: PLAYER_CHARGE_RADIUS,
        },
    )
}

//-----------------------------------------------------------------------------
//SYSTEM PART
//-----------------------------------------------------------------------------

pub fn weapons(world: &mut World, cmd: &mut hecs::CommandBuffer, dt: f32) {
    //get player
    let (_, (player, vel, angle, pos, charge_send, charge_receive)) = world
        .query_mut::<(
            &mut Player,
            &PhysicsMotion,
            &Rotation,
            &Position,
            &mut ChargeSender,
            &mut ChargeReceiver,
        )>()
        .into_iter()
        .next()
        .unwrap();
    //decrement timer
    player.fire_timer -= dt;
    //shoot
    if player.fire_timer <= 0.0 && is_mouse_button_down(MouseButton::Right) {
        //reset timer
        player.fire_timer = PLAYER_FIRE_COOLDOWN;
        //fire
        cmd.spawn(projectile::create_projectile(
            vec2(pos.x, pos.y),
            Vec2::from_angle(angle.angle).rotate(Vec2::X) * 250.0 + vec2(vel.vel.x, vel.vel.y),
            0.2,
            Team::Player,
            ProjectileType::Small {
                charge: -player.polarity,
            },
        ));
    }

    //polarity switching
    if is_key_pressed(KeyCode::A) {
        player.polarity = -player.polarity;
        //change charge
        charge_receive.multiplier = 1.0 * player.polarity as f32;
        charge_send.force = PLAYER_CHARGE_FORCE * player.polarity as f32;
    }
}

pub fn motion_update(world: &mut World, dt: f32) {
    //get player
    let (_, (vel, angle, pos)) = world
        .query_mut::<(&mut PhysicsMotion, &mut Rotation, &mut Position)>()
        .with::<&Player>()
        .into_iter()
        .next()
        .unwrap();
    //motion friction
    if is_mouse_button_down(MouseButton::Left) {
        vel.vel.x *= 0.7_f32.powf(dt);
        vel.vel.y *= 0.7_f32.powf(dt);
    } else {
        vel.vel.x *= 0.3_f32.powf(dt);
        vel.vel.y *= 0.3_f32.powf(dt);
    }
    //follow mouse
    let mouse_pos = world_mouse_pos();
    angle.angle = (mouse_pos.y - pos.y).atan2(mouse_pos.x - pos.x);
    //input handling
    if is_mouse_button_down(MouseButton::Left) {
        vel.vel.x += angle.angle.cos() * PLAYER_ACCEL * dt;
        vel.vel.y += angle.angle.sin() * PLAYER_ACCEL * dt;
    }
    //euler integration
    pos.x += vel.vel.x * dt;
    pos.y += vel.vel.y * dt;
}

pub fn health(world: &mut World, events: &mut World, dt: f32) {
    //get player
    let player_query = &mut world.query::<(&mut Player, &mut Health)>();
    let (player_id, (player, player_hp)) = player_query.into_iter().next().unwrap();
    //move invul frames
    player.invul_timer -= dt;
    if player.invul_timer > 0.0 {
        return;
    }
    //health regen
    player_hp.heal(PLAYER_BASE_HP_REGEN * dt);
    //get events concerning the player
    let hit_events = events
        .query_mut::<&HitEvent>()
        .into_iter()
        .filter(|event| event.1.who == player_id);
    for (_, event) in hit_events {
        //can they hurt you?
        if !event.can_hurt {
            continue;
        }
        //get damage
        let Ok(damage) = world.get::<&DamageDealer>(event.by) else {
            continue;
        };
        //apply it
        player_hp.hp -= damage.dmg;
        //set invul frames
        player.invul_timer = PLAYER_INVUL_COOLDOWN;
        //check for death
        if player_hp.hp <= 0.0 {
            //TODO DEATH
        }
    }
}

pub fn visuals(world: &mut World, fx: &mut FxManager) {
    //get player
    let (_, (player, pos, rotation, sprite, health)) = world
        .query_mut::<(&mut Player, &Position, &Rotation, &mut Sprite, &Health)>()
        .into_iter()
        .next()
        .unwrap();

    //change texture based on polarity
    sprite.texture = if player.polarity > 0 {
        PLAYER_TEX_POSITIVE
    } else {
        PLAYER_TEX_NEGATIVE
    };

    //emit fumes if running
    if is_mouse_button_down(MouseButton::Left) {
        fx.burst_particles(
            Particle {
                pos: vec2(pos.x, pos.y) + Vec2::from_angle(rotation.angle).rotate(-Vec2::X) * 15.0,
                vel: Vec2::from_angle(rotation.angle).rotate(-Vec2::X) * 100.0,
                life: fastrand::f32() * 0.8 + 0.2,
                max_life: 1.0,
                min_size: 1.0,
                max_size: 4.0,
                color: ORANGE,
            },
            4.0,
            PI / 8.0,
            7,
        )
    }

    //explode if dead
    if health.hp <= 0.0 && !player.dead_burst {
        player.dead_burst = true;
        //make player's sprite not visible
        sprite.scale = 0.0;
        //emit dead particle
        for i in 1..5 {
            fx.burst_particles(
                Particle {
                    pos: vec2(pos.x, pos.y),
                    vel: vec2(45.0 * i as f32, 0.0),
                    life: 1.0,
                    max_life: 1.0,
                    min_size: 0.0,
                    max_size: 20.0,
                    color: RED,
                },
                30.0,
                2.0 * PI,
                8 * i,
            );
        }
    }
}
