//! Enemy spawns used to compose a wave.
use hecs::CommandBuffer;

use super::*;

use macroquad::prelude::*;

use crate::{enemy, SPACE_HEIGHT, SPACE_WIDTH};

/// Collection of useful structures that are commonly used to
/// implement wave spawning.
pub struct WavePreamble<'a> {
    /// Current [World] of the game.
    pub world: &'a World,
    /// [CommandBuffer] to dereffer enemy spawning.
    pub cmd: &'a mut CommandBuffer,
    /// Current position of the [Player] so that some
    /// enemies can target it.
    pub player_pos: &'a Position,
}

//
//WAVE PART
//

/// Spawns 8 charged asteroids, each heading into the center.
///
/// Charges of asteroids in corners are opposite from charges from the asteroids
/// which come from the edges.
#[allow(dead_code)]
pub(super) fn center_crunch(cmd: &mut CommandBuffer) {
    //center crunch attack
    let charge = fastrand::i8(0..=1) * 2 - 1;
    //spawn them
    cmd.spawn(
        enemy::create_charged_asteroid(
            vec2(-SPAWN_PUSHBACK, SPACE_HEIGHT / 2.0),
            vec2(1.0, 0.0),
            charge,
        )
        .build(),
    );
    cmd.spawn(
        enemy::create_charged_asteroid(
            vec2(SPACE_WIDTH + SPAWN_PUSHBACK, SPACE_HEIGHT / 2.0),
            vec2(-1.0, 0.0),
            charge,
        )
        .build(),
    );
    cmd.spawn(
        enemy::create_charged_asteroid(
            vec2(SPACE_WIDTH / 2.0, -SPAWN_PUSHBACK),
            vec2(0.0, 1.0),
            charge,
        )
        .build(),
    );
    cmd.spawn(
        enemy::create_charged_asteroid(
            vec2(SPACE_WIDTH / 2.0, SPACE_HEIGHT + SPAWN_PUSHBACK),
            vec2(0.0, -1.0),
            charge,
        )
        .build(),
    );
    //spawn opposite charged corners
    cmd.spawn(
        enemy::create_charged_asteroid(
            vec2(-SPAWN_PUSHBACK, -SPAWN_PUSHBACK),
            vec2(1.0, 1.0),
            -charge,
        )
        .build(),
    );
    cmd.spawn(
        enemy::create_charged_asteroid(
            vec2(SPACE_WIDTH + SPAWN_PUSHBACK, -SPAWN_PUSHBACK),
            vec2(-1.0, 1.0),
            -charge,
        )
        .build(),
    );
    cmd.spawn(
        enemy::create_charged_asteroid(
            vec2(-SPAWN_PUSHBACK, SPACE_HEIGHT + SPAWN_PUSHBACK),
            vec2(1.0, -1.0),
            -charge,
        )
        .build(),
    );
    cmd.spawn(
        enemy::create_charged_asteroid(
            vec2(SPACE_WIDTH + SPAWN_PUSHBACK, SPACE_HEIGHT + SPAWN_PUSHBACK),
            vec2(-1.0, -1.0),
            -charge,
        )
        .build(),
    );
}

/// Initialised tripleshot timer.
///
/// Should be ran before `tripleshot`.
#[inline]
#[allow(dead_code)]
pub(super) fn tripleshot_init(timer: &mut f32) {
    *timer = 2.0;
}

/// Shoots three sets of three charged asteroids.
///
/// The asteroids are slightly spread.
#[allow(dead_code)]
pub(super) fn tripleshot(cmd: &mut CommandBuffer, timer: &f32, data: &mut u8) {
    //get side
    let side = get_side();
    let center = get_center_pos(side);
    let dir = get_dir(side);
    let charge = fastrand::i8(0..=1) * 2 - 1;
    //genarate triple shot function
    let mut shoot = || {
        cmd.spawn(enemy::create_charged_asteroid(center, dir * 1.6, charge).build());
        cmd.spawn(
            enemy::create_charged_asteroid(
                center + dir.perp() * 50.0,
                Vec2::from_angle(PI / 6.0).rotate(dir) * 1.3,
                -charge,
            )
            .build(),
        );
        cmd.spawn(
            enemy::create_charged_asteroid(
                center - dir.perp() * 50.0,
                Vec2::from_angle(-PI / 6.0).rotate(dir) * 1.3,
                -charge,
            )
            .build(),
        );
    };
    //get state
    *data = match *data & 0x03 {
        0 => {
            shoot();
            1
        }
        1 => {
            if *timer <= 1.33 {
                shoot();
                2
            } else {
                1
            }
        }

        2 => {
            if *timer <= 0.65 {
                shoot();
                3
            } else {
                2
            }
        }
        x => x,
    }
}

/// Initialised salvos.
#[inline]
#[allow(dead_code)]
pub(super) fn salvo_init(timer: &mut f32, base_time: f32) {
    *timer = base_time;
}

/// Spawns an asteroid from a random edge.
pub(super) fn asteroid(preamble: &mut WavePreamble) {
    let side = get_side();
    let dir = get_dir(side);
    let pos = get_spawn_pos(side) - dir * 120.0;
    let charge = fastrand::i8(0..=1) * 2 - 1;
    preamble
        .cmd
        .spawn(enemy::create_charged_asteroid(pos, dir, charge).build());
}

/// Spawns a big asteroid from a random edge.
pub(super) fn big_asteroid(preamble: &mut WavePreamble) {
    let side = get_side();
    let dir = get_dir(side);
    let pos = get_spawn_pos(side) - dir * 120.0;
    let charge = fastrand::i8(0..=1) * 2 - 1;
    preamble
        .cmd
        .spawn(enemy::create_big_asteroid(pos, dir, charge).build());
}

/// Spawns a charged asteroid from a random edge.
pub(super) fn charged_asteroid(preamble: &mut WavePreamble) {
    let side = get_side();
    let dir = get_dir(side);
    let pos = get_spawn_pos(side) - dir * SPAWN_PUSHBACK;
    let charge = fastrand::i8(0..=1) * 2 - 1;
    enemy::charged::create_supercharged_asteroid(pos, dir, charge)(preamble.world, preamble.cmd);
}

/// Spawns a sawblade from a random edge.
pub(super) fn follower(preamble: &mut WavePreamble) {
    let side = get_side();
    let dir = get_dir(side);
    let pos = get_spawn_pos(side) - dir * SPAWN_PUSHBACK;
    let charge = fastrand::i8(-1..=1);
    preamble
        .cmd
        .spawn(enemy::follower::create_follower(pos, dir, charge).build())
}

/// Spawns a mine from a random edge.
pub(super) fn mine(preamble: &mut WavePreamble) {
    let side = get_side();
    let dir = get_dir(side);
    let pos = get_spawn_pos(side) - dir * SPAWN_PUSHBACK;
    let charge = fastrand::i8(-1..=1);
    preamble
        .cmd
        .spawn(enemy::mine::create_mine(pos, dir, charge).build())
}

//------------------------------------------------------------------------------
//HELPER FUNCTIONS
//------------------------------------------------------------------------------

/// Returns a random side.
///
/// * 0 = TOP
/// * 1 = BOTTOM
/// * 2 = LEFT
/// * 3 = RIGHT
#[inline]
fn get_side() -> u8 {
    fastrand::u8(0..4)
}

/// Returns a number representing the opposite side from `side`.
#[inline]
#[allow(dead_code)]
fn get_opposite_side(side: u8) -> u8 {
    match side {
        0 => 1,
        1 => 0,
        2 => 3,
        3 => 2,
        _ => panic!("Not a valid side number, {side}"),
    }
}

/// Returns a random valid position resprecting `SPAWN_MARGIN` and `SPAWN_PUSHBACK` so that
/// the enemy is spawned on side `side`.
#[inline]
fn get_spawn_pos(side: u8) -> Vec2 {
    match side {
        0 => {
            //TOP
            vec2(
                fastrand::f32() * (SPACE_WIDTH - 2.0 * SPAWN_MARGIN) + SPAWN_MARGIN,
                -SPAWN_PUSHBACK,
            )
        }
        1 => {
            //BOTTOM
            vec2(
                fastrand::f32() * (SPACE_WIDTH - 2.0 * SPAWN_MARGIN) + SPAWN_MARGIN,
                SPACE_HEIGHT + SPAWN_PUSHBACK,
            )
        }
        2 => {
            //LEFT
            vec2(
                -SPAWN_PUSHBACK,
                fastrand::f32() * (SPACE_HEIGHT - 2.0 * SPAWN_MARGIN) + SPAWN_MARGIN,
            )
        }
        3 => {
            //RIGHT
            vec2(
                SPACE_WIDTH + SPAWN_PUSHBACK,
                fastrand::f32() * (SPACE_HEIGHT - 2.0 * SPAWN_MARGIN) + SPAWN_MARGIN,
            )
        }
        _ => unreachable!("Random number should not exceed range 0..4"),
    }
}

/// Returns the center of side `side`.
#[inline]
fn get_center_pos(side: u8) -> Vec2 {
    match side {
        0 => {
            //TOP
            vec2(SPACE_WIDTH / 2.0, -SPAWN_PUSHBACK)
        }
        1 => {
            //BOTTOM
            vec2(SPACE_WIDTH / 2.0, SPACE_HEIGHT + SPAWN_PUSHBACK)
        }
        2 => {
            //LEFT
            vec2(-SPAWN_PUSHBACK, SPACE_HEIGHT / 2.0)
        }
        3 => {
            //RIGHT
            vec2(SPACE_WIDTH + SPAWN_PUSHBACK, SPACE_HEIGHT / 2.0)
        }
        _ => unreachable!("Random number should not exceed range 0..4"),
    }
}

/// Returns a normal invard (towards center of the screen) unit vector of
/// side `side`.
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
