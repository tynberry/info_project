use hecs::{CommandBuffer, World};
use macroquad::{
    math::vec2,
    window::{screen_height, screen_width},
};

use crate::enemy;

const SPAWN_COOLDOWN: f32 = 7.0;
const SPAWN_MARGIN: f32 = 20.0;
const SPAWN_PUSHBACK: f32 = 10.0;

#[derive(Clone, Copy, Debug)]
pub struct EnemySpawner {
    pub spawn_timer: f32,
}

impl EnemySpawner {
    pub fn new() -> Self {
        Self { spawn_timer: 0.0 }
    }
}

impl Default for EnemySpawner {
    fn default() -> Self {
        Self::new()
    }
}

//------------------------------------------------------------------------------
//SYSTEM PART
//------------------------------------------------------------------------------

pub fn enemy_spawning(world: &mut World, cmd: &mut CommandBuffer, dt: f32) {
    //get spawner
    let (_, spawner) = world
        .query_mut::<&mut EnemySpawner>()
        .into_iter()
        .next()
        .unwrap();
    //move its timer
    spawner.spawn_timer -= dt;
    //spawn!
    if spawner.spawn_timer <= 0.0 {
        spawner.spawn_timer = SPAWN_COOLDOWN;
        //where?
        let side = fastrand::u8(0..4);
        let pos = match side {
            0 => {
                //TOP
                vec2(
                    fastrand::f32() * (screen_width() - 2.0 * SPAWN_MARGIN) + SPAWN_MARGIN,
                    -SPAWN_PUSHBACK,
                )
            }
            1 => {
                //BOTTOM
                vec2(
                    fastrand::f32() * (screen_width() - 2.0 * SPAWN_MARGIN) + SPAWN_MARGIN,
                    screen_height() + SPAWN_PUSHBACK,
                )
            }
            2 => {
                //LEFT
                vec2(
                    -SPAWN_PUSHBACK,
                    fastrand::f32() * (screen_height() - 2.0 * SPAWN_MARGIN) + SPAWN_MARGIN,
                )
            }
            3 => {
                //RIGHT
                vec2(
                    screen_width() + SPAWN_PUSHBACK,
                    fastrand::f32() * (screen_height() - 2.0 * SPAWN_MARGIN) + SPAWN_MARGIN,
                )
            }
            _ => unreachable!("Random number should not exceed range 0..4"),
        };
        //which way
        let dir = match side {
            0 => vec2(0.0, 1.0),
            1 => vec2(0.0, -1.0),
            2 => vec2(1.0, 0.0),
            3 => vec2(-1.0, 0.0),
            _ => unreachable!("Random number should not exceed range 0..4"),
        };
        //spawn an asteroid!!!
        cmd.spawn(enemy::create_asteroid(pos, dir));
    }
}
