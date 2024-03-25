use hecs::{CommandBuffer, World};
use macroquad::{
    math::{vec2, Vec2},
    window::{screen_height, screen_width},
};

use crate::enemy::{self, Enemy};

const SPAWN_FALLBACK_COOLDOWN: f32 = 10.0;
const SPAWN_NO_ENEMY_TIMER: f32 = 3.0;
const SPAWN_MARGIN: f32 = 20.0;
const SPAWN_PUSHBACK: f32 = 10.0;

#[derive(Clone, Copy, Debug)]
pub struct EnemySpawner {
    pub wave_counter: u32,
    pub no_enemies: bool,
    pub wave_type: u8,
    pub state: SpawnState
}

#[derive(Clone, Copy, Debug)]
pub enum SpawnState {
    Waiting{timer: f32},
    Spawning{time: f32}
}

impl EnemySpawner {
    pub fn new() -> Self {
        Self { wave_counter: 0, no_enemies: true, wave_type: 0, state: SpawnState::Waiting { timer: SPAWN_FALLBACK_COOLDOWN } }
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
    //get spawner
    let (_, spawner) = world
        .query_mut::<&mut EnemySpawner>()
        .into_iter()
        .next()
        .unwrap();
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
                Some(SpawnState::Spawning { time: std::f32::NAN } )
            }else{
                None
            }
        },
        SpawnState::Spawning { time } => {
            if time.is_nan() {
                //init the wave
                spawner.wave_counter += 1;
                spawner.no_enemies = false;
                spawner.wave_type =  fastrand::u8(0..3);
                //do the initial calls
                match spawner.wave_type {
                    0 => center_crunch(cmd),
                    1 => opposite_sides(cmd),
                    2 => one_side_opposites(cmd),
                    _ => unreachable!("Random number should not exceed its bounds!")
                }
                //change states
                if time.is_nan() {
                    Some(SpawnState::Waiting { timer: SPAWN_FALLBACK_COOLDOWN })
                }else{
                    None
                }
            }else{
                //move timer 
                *time -= dt;
                //do the another wave calls
                match spawner.wave_type {
                    _ => ()
                }
                //change states
                if *time <= 0.0 {
                    Some(SpawnState::Waiting { timer: SPAWN_FALLBACK_COOLDOWN })
                }else{
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

fn center_crunch(cmd: &mut CommandBuffer) {
    //center crunch attack 
    let charge = fastrand::i8(0..1) * 2 - 1;
    //spawn them 
    cmd.spawn(enemy::create_charged_asteroid(vec2(-SPAWN_PUSHBACK, screen_height() / 2.0), vec2(1.0, 0.0),charge));
    cmd.spawn(enemy::create_charged_asteroid(vec2(screen_width()+SPAWN_PUSHBACK, screen_height() / 2.0), vec2(-1.0, 0.0),charge));
    cmd.spawn(enemy::create_charged_asteroid(vec2( screen_width() / 2.0,-SPAWN_PUSHBACK), vec2(0.0, 1.0),charge));
    cmd.spawn(enemy::create_charged_asteroid(vec2(screen_width() / 2.0, screen_height()+SPAWN_PUSHBACK), vec2(0.0, -1.0),charge));
}

fn opposite_sides(cmd: &mut CommandBuffer) {
    //two opposite sides, opposite polarities
    let side = get_side();
    let opposite_side = get_opposite_side(side);
    //spawn them 
    for _ in 0..4 {
        cmd.spawn(enemy::create_charged_asteroid(get_spawn_pos(side), get_dir(side), 1));
        cmd.spawn(enemy::create_charged_asteroid(get_spawn_pos(opposite_side), get_dir(opposite_side), -1));
    }
}

fn one_side_opposites(cmd: &mut CommandBuffer) {
    //one side, both polarities, equal count
    let side = get_side();
    //spawn them 
    for _ in 0..3 {
        cmd.spawn(enemy::create_charged_asteroid(get_spawn_pos(side), get_dir(side), 1));
        cmd.spawn(enemy::create_charged_asteroid(get_spawn_pos(side), get_dir(side), -1));
    }
}

//-----------------------------------------------------------------------------
//GENERAL FUNCTIONS
//-----------------------------------------------------------------------------

#[inline]
fn get_side() -> u8 {
    fastrand::u8(0..4)
}

#[inline]
fn get_opposite_side(side: u8) -> u8 {
    match side {
        0 => 1,
        1 => 0,
        2 => 3,
        3 => 2,
        _ => panic!("Not a valid side number, {side}")
    }
}

#[inline]
fn get_spawn_pos(side: u8) -> Vec2 {
    match side {
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
    }
}

#[inline]
fn get_dir(side: u8) -> Vec2 {
    match side {
        0 => vec2(0.0, 1.0),
        1 => vec2(0.0, -1.0),
        2 => vec2(1.0, 0.0),
        3 => vec2(-1.0, 0.0),
        _ => unreachable!("Random number should not exceed range 0..4"),
    }
}