use hecs::CommandBuffer;

use super::*;

use macroquad::prelude::*;

use crate::{enemy, SPACE_HEIGHT, SPACE_WIDTH};

pub struct WavePreamble<'a> {
    pub world: &'a World,
    pub cmd: &'a mut CommandBuffer,
    pub player_pos: &'a Position,
    pub timer: &'a mut f32,
    pub data: &'a mut u8,
}

//
//WAVE PART
//

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

#[inline]
#[allow(dead_code)]
pub(super) fn tripleshot_init(timer: &mut f32) {
    *timer = 2.0;
}

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

#[inline]
pub(super) fn salvo_init(timer: &mut f32, base_time: f32) {
    *timer = base_time;
}

pub(super) fn asteroid(_: &World, cmd: &mut CommandBuffer) {
    let side = get_side();
    let dir = get_dir(side);
    let pos = get_spawn_pos(side) - dir * 120.0;
    let charge = fastrand::i8(0..=1) * 2 - 1;
    cmd.spawn(enemy::create_charged_asteroid(pos, dir, charge).build());
}

pub(super) fn big_asteroid(_: &World, cmd: &mut CommandBuffer) {
    let side = get_side();
    let dir = get_dir(side);
    let pos = get_spawn_pos(side) - dir * 120.0;
    let charge = fastrand::i8(0..=1) * 2 - 1;
    cmd.spawn(enemy::create_big_asteroid(pos, dir, charge).build());
}

pub(super) fn charged_asteroid(world: &World, cmd: &mut CommandBuffer) {
    let side = get_side();
    let dir = get_dir(side);
    let pos = get_spawn_pos(side) - dir * SPAWN_PUSHBACK;
    let charge = fastrand::i8(0..=1) * 2 - 1;
    enemy::charged::create_supercharged_asteroid(pos, dir, charge)(world, cmd);
}

pub(super) fn follower(_: &World, cmd: &mut CommandBuffer) {
    let side = get_side();
    let dir = get_dir(side);
    let pos = get_spawn_pos(side) - dir * SPAWN_PUSHBACK;
    let charge = fastrand::i8(-1..=1);
    cmd.spawn(enemy::follower::create_follower(pos, dir, charge).build())
}

pub(super) fn mine(_: &World, cmd: &mut CommandBuffer) {
    let side = get_side();
    let dir = get_dir(side);
    let pos = get_spawn_pos(side) - dir * SPAWN_PUSHBACK;
    let charge = fastrand::i8(-1..=1);
    cmd.spawn(enemy::mine::create_mine(pos, dir, charge).build())
}

pub(super) fn salvo_body(
    preamble: WavePreamble,
    base_time: f32,
    count: u8,
    shoot: impl Fn(&World, &mut CommandBuffer),
) {
    *preamble.data = match *preamble.data {
        0 => {
            shoot(preamble.world, preamble.cmd);
            1
        }
        x if x < count => {
            if *preamble.timer <= (base_time / (count) as f32) * (count - x) as f32 {
                shoot(preamble.world, preamble.cmd);
                x + 1
            } else {
                x
            }
        }
        x => x,
    }
}

//------------------------------------------------------------------------------
//HELPER FUNCTIONS
//------------------------------------------------------------------------------

#[inline]
fn get_side() -> u8 {
    fastrand::u8(0..4)
}

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
