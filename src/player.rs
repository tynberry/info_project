use hecs::World;
use macroquad::prelude::*;

use crate::{
    basic::{DamageDealer, DeleteOnWarp, Health, HitEvent, Position, Rotation},
    projectile::Projectile,
};

const PLAYER_ACCEL: f32 = 600.0;

const PLAYER_FIRE_COOLDOWN: f32 = 0.5;

const PLAYER_SIZE: f32 = 15.0;

#[derive(Debug)]
pub struct Player {
    vel_x: f32,
    vel_y: f32,

    fire_timer: f32,
}

impl Player {
    pub fn new() -> Self {
        Self {
            vel_x: 0.0,
            vel_y: 0.0,

            fire_timer: 0.0,
        }
    }
}

//-----------------------------------------------------------------------------
//SYSTEM PART
//-----------------------------------------------------------------------------

pub fn weapons(world: &mut World, cmd: &mut hecs::CommandBuffer, dt: f32) {
    //get player
    let (_, (player, player_angle, player_pos)) = world
        .query_mut::<(&mut Player, &Rotation, &Position)>()
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
        cmd.spawn((
            Projectile {
                size: 2.0,
                vel: Vec2::from_angle(player_angle.angle).rotate(Vec2::X) * 250.0
                    + vec2(player.vel_x, player.vel_y),
                team: crate::basic::Team::Player,
            },
            *player_pos,
            DeleteOnWarp,
        ));
    }
}

pub fn motion_update(world: &mut World, dt: f32) {
    //get player
    let (_, (player, player_angle, player_pos)) = world
        .query_mut::<(&mut Player, &mut Rotation, &mut Position)>()
        .into_iter()
        .next()
        .unwrap();
    //motion friction
    player.vel_x *= 0.9_f32.powf(dt);
    player.vel_y *= 0.9_f32.powf(dt);
    //follow mouse
    let mouse_pos = mouse_position();
    player_angle.angle = (mouse_pos.1 - player_pos.y).atan2(mouse_pos.0 - player_pos.x);
    //input handling
    if is_mouse_button_down(MouseButton::Left) {
        player.vel_x += player_angle.angle.cos() * PLAYER_ACCEL * dt;
        player.vel_y += player_angle.angle.sin() * PLAYER_ACCEL * dt;
    }
    //euler integration
    player_pos.x += player.vel_x * dt;
    player_pos.y += player.vel_y * dt;
}

pub fn health(world: &mut World, events: &mut World) {
    //get player
    let player_query = &mut world.query::<&mut Health>();
    let (player_id, player_hp) = player_query.into_iter().next().unwrap();
    //get events concerning the player
    let hit_events = events
        .query_mut::<&HitEvent>()
        .into_iter()
        .filter(|event| event.1.who == player_id);
    for (_, event) in hit_events {
        //get damage
        let Ok(damage) = world.get::<&DamageDealer>(event.by) else {
            continue;
        };
        //apply it
        player_hp.hp -= damage.dmg;
        //check for death
        if player_hp.hp <= 0.0 {
            //TODO DEATH
        }
    }
}

pub fn render(world: &mut World) {
    //get player
    let (_, (player_angle, player_pos)) = world
        .query_mut::<(&Rotation, &Position)>()
        .with::<&Player>()
        .into_iter()
        .next()
        .unwrap();
    //draw player
    let v1 = vec2(player_pos.x, player_pos.y)
        + Vec2::from_angle(player_angle.angle).rotate(Vec2::X) * PLAYER_SIZE;
    let v2 = vec2(player_pos.x, player_pos.y)
        + Vec2::from_angle(player_angle.angle + 4.0 * std::f32::consts::PI / 3.0).rotate(Vec2::X)
            * PLAYER_SIZE;
    let v3 = vec2(player_pos.x, player_pos.y)
        + Vec2::from_angle(player_angle.angle - 4.0 * std::f32::consts::PI / 3.0).rotate(Vec2::X)
            * PLAYER_SIZE;
    draw_triangle(v1, v2, v3, RED);
}
