use std::{borrow::Borrow, cell::OnceCell, f32::consts::PI};

use hecs::{CommandBuffer, World};
use macroquad::math::{vec2, Vec2};

use crate::{basic::Position, enemy::Enemy, player::Player};

use self::wave::WavePreamble;

pub mod init;
pub mod state;
mod wave;

const INIT_CREDITS: f32 = 50.0;
const CREDITS_PER_SEC: f32 = 3.0;

const INIT_COOLDOWN: f32 = 5.0;
const MIN_SPAWN_COOLDOWN: f32 = 3.0;
const MAX_SPAWN_COOLDOWN: f32 = 5.0;

const MAX_ENTITIES: usize = 15;

const DOUBLE_CHANCE: f32 = 0.33;
const TRIPLE_CHANCE: f32 = 0.5; //chance when double was rolled

#[derive(Clone, Copy, Debug)]
struct Wave {
    cost: f32,
    gain: f32,
    weight: u32,
    spawn: fn(WavePreamble),
}

const WAVES: [Wave; 5] = [
    Wave {
        cost: 10.0,
        gain: 20.0,
        weight: 10,
        spawn: wave::asteroid,
    },
    Wave {
        cost: 15.0,
        gain: 20.0,
        weight: 20,
        spawn: wave::charged_asteroid,
    },
    Wave {
        cost: 40.0,
        gain: 10.0,
        weight: 30,
        spawn: wave::big_asteroid,
    },
    Wave {
        cost: 30.0,
        gain: 10.0,
        weight: 30,
        spawn: wave::follower,
    },
    Wave {
        cost: 40.0,
        gain: 10.0,
        weight: 30,
        spawn: wave::mine,
    },
];

const SPAWN_MARGIN: f32 = 20.0;
const SPAWN_PUSHBACK: f32 = 10.0;

#[derive(Clone, Copy, Debug)]
pub struct EnemySpawner {
    pub credits: f32,
    pub cooldown: f32,
}

impl EnemySpawner {
    pub fn new() -> Self {
        Self {
            credits: INIT_CREDITS,
            cooldown: INIT_COOLDOWN,
        }
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
    //count enemies
    let enemy_count = world.query_mut::<&Enemy>().into_iter().count();
    //get position of player
    let (_, &player_pos) = world
        .query_mut::<&Position>()
        .with::<&Player>()
        .into_iter()
        .next()
        .unwrap();
    //get spawner
    let spawner_query = &mut world.query::<&mut EnemySpawner>();
    let (_, spawner) = spawner_query.into_iter().next().unwrap();
    //give credits
    spawner.credits += CREDITS_PER_SEC * dt;
    //advance state
    spawner.cooldown -= dt;
    if spawner.cooldown > 0.0 {
        return;
    }
    //TOO MANY ENEMIES
    if enemy_count >= MAX_ENTITIES {
        //set new cooldown
        spawner.cooldown =
            (MAX_SPAWN_COOLDOWN - MIN_SPAWN_COOLDOWN) * fastrand::f32() + MIN_SPAWN_COOLDOWN;
        return;
    }
    //get weight sum
    let weight_sum = WAVES
        .iter()
        .filter(|wave| wave.cost <= spawner.credits)
        .fold(0, |acc, wave| acc + wave.weight);
    //cannot afford any
    if weight_sum == 0 {
        //set new cooldown
        spawner.cooldown =
            (MAX_SPAWN_COOLDOWN - MIN_SPAWN_COOLDOWN) * fastrand::f32() + MIN_SPAWN_COOLDOWN;
        return;
    }
    //randomly choose wave
    let mut value = fastrand::u32(0..weight_sum);
    let wave = 'outer: {
        for wave in WAVES {
            if wave.weight <= value {
                value -= wave.weight
            } else {
                break 'outer wave;
            };
        }
        WAVES[0]
    };
    //how many times?
    let double = fastrand::f32() <= DOUBLE_CHANCE;
    let triple = fastrand::f32() <= TRIPLE_CHANCE;
    let times = match (double, triple) {
        (true, true) => 3,
        (true, false) => 2,
        _ => 1,
    };
    //substract costs
    spawner.credits -= wave.cost * ((times - 1) as f32 * 0.5 + 1.0);
    //add gains
    spawner.credits += wave.gain * times as f32;
    if spawner.credits < 0.0 {
        spawner.credits = 0.0;
    }
    //SPAWN!!
    for _ in 0..times {
        (wave.spawn)(WavePreamble {
            world,
            cmd,
            player_pos: &player_pos,
        })
    }
    //set new cooldown
    spawner.cooldown =
        (MAX_SPAWN_COOLDOWN - MIN_SPAWN_COOLDOWN) * fastrand::f32() + MIN_SPAWN_COOLDOWN;
}
