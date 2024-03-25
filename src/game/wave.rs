use hecs::CommandBuffer;

use super::*;

use macroquad::prelude::*;

use crate::enemy;

pub(super) fn center_crunch(cmd: &mut CommandBuffer) {
    //center crunch attack
    let charge = fastrand::i8(0..=1) * 2 - 1;
    //spawn them
    cmd.spawn(enemy::create_charged_asteroid(
        vec2(-SPAWN_PUSHBACK, screen_height() / 2.0),
        vec2(1.0, 0.0),
        charge,
    ));
    cmd.spawn(enemy::create_charged_asteroid(
        vec2(screen_width() + SPAWN_PUSHBACK, screen_height() / 2.0),
        vec2(-1.0, 0.0),
        charge,
    ));
    cmd.spawn(enemy::create_charged_asteroid(
        vec2(screen_width() / 2.0, -SPAWN_PUSHBACK),
        vec2(0.0, 1.0),
        charge,
    ));
    cmd.spawn(enemy::create_charged_asteroid(
        vec2(screen_width() / 2.0, screen_height() + SPAWN_PUSHBACK),
        vec2(0.0, -1.0),
        charge,
    ));
    //spawn opposite charged corners
    cmd.spawn(enemy::create_charged_asteroid(
        vec2(-SPAWN_PUSHBACK, -SPAWN_PUSHBACK),
        vec2(1.0, 1.0),
        -charge,
    ));
    cmd.spawn(enemy::create_charged_asteroid(
        vec2(screen_width() + SPAWN_PUSHBACK, -SPAWN_PUSHBACK),
        vec2(-1.0, 1.0),
        -charge,
    ));
    cmd.spawn(enemy::create_charged_asteroid(
        vec2(-SPAWN_PUSHBACK, screen_height() + SPAWN_PUSHBACK),
        vec2(1.0, -1.0),
        -charge,
    ));
    cmd.spawn(enemy::create_charged_asteroid(
        vec2(
            screen_width() + SPAWN_PUSHBACK,
            screen_height() + SPAWN_PUSHBACK,
        ),
        vec2(-1.0, -1.0),
        -charge,
    ));
}

#[inline]
pub(super) fn tripleshot_init(timer: &mut f32) {
    *timer = 2.0;
}

pub(super) fn tripleshot(cmd: &mut CommandBuffer, timer: &f32, data: &mut u8) {
    //get side
    let side = get_side();
    let center = get_center_pos(side);
    let dir = get_dir(side);
    let charge = fastrand::i8(0..=1) * 2 - 1;
    //genarate triple shot function
    let mut shoot = || {
        cmd.spawn(enemy::create_charged_asteroid(center, dir * 1.6, charge));
        cmd.spawn(enemy::create_charged_asteroid(
            center + dir.perp() * 50.0,
            Vec2::from_angle(PI / 6.0).rotate(dir) * 1.3,
            -charge,
        ));
        cmd.spawn(enemy::create_charged_asteroid(
            center - dir.perp() * 50.0,
            Vec2::from_angle(-PI / 6.0).rotate(dir) * 1.3,
            -charge,
        ));
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

pub(super) fn one_side_opposites(cmd: &mut CommandBuffer) {
    //one side, both polarities, equal count
    let side = get_side();
    //spawn them
    for _ in 0..3 {
        cmd.spawn(enemy::create_charged_asteroid(
            get_spawn_pos(side),
            get_dir(side),
            1,
        ));
        cmd.spawn(enemy::create_charged_asteroid(
            get_spawn_pos(side),
            get_dir(side),
            -1,
        ));
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
fn get_center_pos(side: u8) -> Vec2 {
    match side {
        0 => {
            //TOP
            vec2(screen_width() / 2.0, -SPAWN_PUSHBACK)
        }
        1 => {
            //BOTTOM
            vec2(screen_width() / 2.0, screen_height() + SPAWN_PUSHBACK)
        }
        2 => {
            //LEFT
            vec2(-SPAWN_PUSHBACK, screen_height() / 2.0)
        }
        3 => {
            //RIGHT
            vec2(screen_width() + SPAWN_PUSHBACK, screen_height() / 2.0)
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
