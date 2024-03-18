use hecs::World;
use macroquad::prelude::*;

use crate::basic::Position;

#[derive(Debug)]
pub struct Player {
    vel_x: f32,
    vel_y: f32,
}

impl Player {
    pub fn new() -> Self {
        Self {
            vel_x: 0.0,
            vel_y: 0.0,
        }
    }
}

//-----------------------------------------------------------------------------
//SYSTEM PART
//-----------------------------------------------------------------------------

pub fn player_motion_update(world: &mut World, dt: f32) {
    //get player
    let (_, (player, player_pos)) = world
        .query_mut::<(&mut Player, &mut Position)>()
        .into_iter()
        .next()
        .unwrap();
    //motion friction
    player.vel_x *= 0.9_f32.powf(dt);
    player.vel_y *= 0.9_f32.powf(dt);
    //input handling 
    if is_key_down(KeyCode::Up) {
        player.vel_y -= 100.0 * dt;
    }
    if is_key_down(KeyCode::Down) {
        player.vel_y += 100.0 * dt;
    }
    if is_key_down(KeyCode::Left) {
        player.vel_x -= 100.0 * dt;
    }
    if is_key_down(KeyCode::Right) {
        player.vel_x += 100.0 * dt;
    }
    //euler integration 
    player_pos.x += player.vel_x * dt;
    player_pos.y += player.vel_y * dt;
}

pub fn player_render(world: &mut World) {
    //get player
    let (_, player_pos) = world
        .query_mut::<&mut Position>()
        .with::<&Player>()
        .into_iter()
        .next()
        .unwrap();
    //draw player 
    draw_rectangle(player_pos.x, player_pos.y, 20.0, 20.0, RED);
}