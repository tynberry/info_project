use std::f32::consts::PI;

use hecs::{CommandBuffer, World};
use macroquad::math::{vec2, Vec2};

use crate::{basic::Position, enemy::Enemy, player::Player};

use self::wave::WavePreamble;

pub mod init;
pub mod state;
mod wave;

const SPAWN_INIT_COOLDOWN: f32 = 5.0;
const SPAWN_FALLBACK_COOLDOWN: f32 = 30.0;
const SPAWN_NO_ENEMY_TIMER: f32 = 3.0;
const SPAWN_MARGIN: f32 = 20.0;
const SPAWN_PUSHBACK: f32 = 10.0;

#[derive(Clone, Copy, Debug)]
pub struct EnemySpawner {
    pub wave_counter: u32,
    pub no_enemies: bool,
    pub wave_type: u8,
    pub last_wave_type: u8,
    pub state: SpawnState,
}

#[derive(Clone, Copy, Debug)]
pub enum SpawnState {
    Waiting { timer: f32 },
    Spawning { time: f32, data: u8 },
}

impl EnemySpawner {
    pub fn new() -> Self {
        Self {
            wave_counter: 0,
            no_enemies: true,
            wave_type: 0,
            last_wave_type: 0,
            state: SpawnState::Waiting {
                timer: SPAWN_INIT_COOLDOWN,
            },
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
    //advance state
    let new_state = match &mut spawner.state {
        SpawnState::Waiting { timer } => {
            //if there are no enemies, time travel the timer
            if enemy_count == 0 && !spawner.no_enemies {
                *timer = SPAWN_NO_ENEMY_TIMER;
                spawner.no_enemies = true;
            }
            //move the timer
            *timer -= dt;
            //change state
            if *timer <= 0.0 {
                Some(SpawnState::Spawning {
                    time: std::f32::NAN,
                    data: 0,
                })
            } else {
                None
            }
        }
        SpawnState::Spawning { time, data } => {
            if time.is_nan() {
                //init the wave
                spawner.wave_counter += 1;
                spawner.no_enemies = false;
                spawner.last_wave_type = spawner.wave_type;
                spawner.wave_type = fastrand::u8(0..=5);
                if spawner.wave_type == spawner.last_wave_type {
                    spawner.wave_type = fastrand::u8(0..=5);
                }
                //do the initial calls
                match spawner.wave_type {
                    0 => wave::center_crunch(cmd),
                    //1 => wave::tripleshot_init(time),
                    1 => wave::salvo_init(time, 3.0),
                    2 => wave::salvo_init(time, 5.0),
                    3 => wave::salvo_init(time, 4.0),
                    4 => wave::salvo_init(time, 3.0),
                    5 => wave::salvo_init(time, 3.0),
                    _ => unreachable!("Random number should not exceed its bounds!"),
                }
                //change states
                if time.is_nan() {
                    Some(SpawnState::Waiting {
                        timer: SPAWN_FALLBACK_COOLDOWN,
                    })
                } else {
                    None
                }
            } else {
                //move timer
                *time -= dt;
                //create wave preamble
                let preamble = WavePreamble {
                    world,
                    cmd,
                    player_pos: &player_pos,
                    timer: time,
                    data,
                };
                //do the another wave calls
                match spawner.wave_type {
                    1 => wave::salvo_body(preamble, 3.0, 7, wave::asteroid),
                    2 => wave::salvo_body(preamble, 5.0, 3, wave::big_asteroid),
                    3 => wave::salvo_body(preamble, 4.0, 5, wave::charged_asteroid),
                    4 => wave::salvo_body(preamble, 3.0, 4, wave::follower),
                    5 => wave::salvo_body(preamble, 3.0, 3, wave::mine),
                    _ => (),
                }
                //change states
                if *time <= 0.0 {
                    Some(SpawnState::Waiting {
                        timer: SPAWN_FALLBACK_COOLDOWN,
                    })
                } else {
                    None
                }
            }
        }
    };
    //apply new state
    if let Some(state) = new_state {
        spawner.state = state;
    };
}
