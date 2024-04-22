//! Enemy spawning handling code.

use std::f32::consts::PI;

use hecs::{CommandBuffer, World};
use macroquad::math::{vec2, Vec2};

use crate::{basic::Position, enemy::Enemy, player::Player};

use self::wave::WavePreamble;

pub mod init;
pub mod state;
mod wave;

/// Credits Enemy spawner starts with.
/// Credits are used to spawn enemies.
const INIT_CREDITS: f32 = 50.0;
/// Credits Enemy spawner gets every second.
const CREDITS_PER_SEC: f32 = 3.0;

/// Initial cooldown when game starts.
const INIT_COOLDOWN: f32 = 5.0;
/// Minimal cooldown between individual enemy spawns.
const MIN_SPAWN_COOLDOWN: f32 = 1.0;
/// Maximal cooldown between individual enemy spawns.
const MAX_SPAWN_COOLDOWN: f32 = 3.0;

/// Minimal break time between waves.
const MIN_BREAK_COOLDOWN: f32 = 20.0;
/// Maximal break time between waves.
const MAX_BREAK_COOLDOWN: f32 = 40.0;
/// Break cooldown if all enemies are dead.
const NO_ENEMIES_BREAK_COOLDOWN: f32 = 3.0;

/// Minimal amount of spawns before a wave ends.
const MIN_SPAWNS_BEFORE_BREAK: u32 = 4;
/// Maximal amount of spawns before a wave ends.
const MAX_SPAWNS_BEFORE_BREAK: u32 = 7;

/// Max amount of enemy entities that can be at once.
const MAX_ENTITIES: usize = 15;

/// Chance to spawn an enemy twice.
const DOUBLE_CHANCE: f32 = 0.33;
/// Chance to spawn an enemy thrice.
/// It is chance when double spawn was rolled.
const TRIPLE_CHANCE: f32 = 0.5;

/// Defines a wave that can be spawned.
#[derive(Clone, Copy)]
struct EnemySpawns {
    /// Cost of spawning this enemy.
    /// It must be payed when spawned.
    cost: f32,
    /// Amount of credits the Enemy Spawner gets 
    /// after it paid this wave.
    gain: f32,
    /// Weight of this spawn.
    /// The higher the weight the higher the chance to choose this spawn.
    weight: u32,
    /// Function that spawns the enemy.
    spawn: &'static dyn Fn(&mut WavePreamble),
}

/// Multiplier that takes a enemy spawning function and returns a fuction that runs it `count` times.
const fn wave_mult(
    fun: impl Fn(&mut WavePreamble),
    count: usize,
) -> impl Fn(&mut WavePreamble<'_>) {
    move |preamble: &mut WavePreamble<'_>| {
        for _ in 0..count {
            fun(preamble)
        }
    }
}

/// List of all possible enemy spawns.
const ENEMY_SPAWNS: [EnemySpawns; 5] = [
    //spawn 4 asteroids
    EnemySpawns {
        cost: 10.0,
        gain: 20.0,
        weight: 15,
        spawn: &wave_mult(wave::asteroid, 4),
    },
    //spawn 3 supercharged asteroids
    EnemySpawns {
        cost: 15.0,
        gain: 20.0,
        weight: 20,
        spawn: &wave_mult(wave::charged_asteroid, 3),
    },
    //spawn 1 big asteroid
    EnemySpawns {
        cost: 40.0,
        gain: 10.0,
        weight: 30,
        spawn: &wave::big_asteroid,
    },
    //spawn 3 saw blades
    EnemySpawns {
        cost: 30.0,
        gain: 10.0,
        weight: 30,
        spawn: &wave_mult(wave::follower, 3),
    },
    //spawn 2 mines
    EnemySpawns {
        cost: 40.0,
        gain: 10.0,
        weight: 30,
        spawn: &wave_mult(wave::mine, 2),
    },
];

/// How far from the corners of the world space the enemy should spawn.
/// The enemy spawns farther that this.
const SPAWN_MARGIN: f32 = 20.0;
/// How fat the enemy should be "pushed back" to its edge to not be visible.
const SPAWN_PUSHBACK: f32 = 10.0;

/// Enemy Spawner struct, handles all of the wave logic.
#[derive(Clone, Copy, Debug)]
pub struct EnemySpawner {
    /// How many enemy spawns should we spawn before a break.
    pub before_break: u32,
    /// How many credits the spawner has.
    pub credits: f32,
    /// How long it waits before it either spawns another enemy or 
    /// starts another wave.
    pub cooldown: f32,
}

impl EnemySpawner {
    /// Create a default EnemySpawner
    pub fn new() -> Self {
        Self {
            before_break: MIN_SPAWNS_BEFORE_BREAK,
            credits: INIT_CREDITS,
            cooldown: INIT_COOLDOWN,
        }
    }
}

impl Default for EnemySpawner {
    /// Create a default EnemySpawner
    fn default() -> Self {
        Self::new()
    }
}

//------------------------------------------------------------------------------
//SYSTEM PART
//------------------------------------------------------------------------------

/// Handles the spawning of enemies and wave logic.
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
    //is break over due to lack of enemies
    if spawner.before_break == 0 && enemy_count == 0 {
        spawner.cooldown = NO_ENEMIES_BREAK_COOLDOWN;
        //new before break
        spawner.before_break = fastrand::u32(MIN_SPAWNS_BEFORE_BREAK..=MAX_SPAWNS_BEFORE_BREAK);
    }
    //advance state
    spawner.cooldown -= dt;
    if spawner.cooldown > 0.0 || spawner.before_break == 0 {
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
    let weight_sum = ENEMY_SPAWNS
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
        for wave in ENEMY_SPAWNS {
            if wave.weight <= value {
                value -= wave.weight
            } else {
                break 'outer wave;
            };
        }
        ENEMY_SPAWNS[0]
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
        (wave.spawn)(&mut WavePreamble {
            world,
            cmd,
            player_pos: &player_pos,
        })
    }
    //break time????
    if spawner.before_break == 1 {
        spawner.before_break = 0;
        //set new cooldown
        spawner.cooldown =
            (MAX_BREAK_COOLDOWN - MIN_BREAK_COOLDOWN) * fastrand::f32() + MIN_BREAK_COOLDOWN;
        return;
    }
    spawner.before_break -= 1;
    //set new cooldown
    spawner.cooldown =
        (MAX_SPAWN_COOLDOWN - MIN_SPAWN_COOLDOWN) * fastrand::f32() + MIN_SPAWN_COOLDOWN;
}
