use macroquad::prelude::*;

use crate::basic::{
    motion::LinearMotion, render::Rectangle, DamageDealer, HurtBox, Position, Team,
};

const ASTEROID_SPEED: f32 = 100.0;
const ASTEROID_SIZE: f32 = 30.0;
const ASTEROID_DMG: f32 = 0.5;

pub fn create_asteroid(
    pos: Vec2,
    dir: Vec2,
) -> (
    Position,
    LinearMotion,
    Rectangle,
    HurtBox,
    DamageDealer,
    Team,
) {
    (
        Position { x: pos.x, y: pos.y },
        LinearMotion {
            vel: dir * ASTEROID_SPEED,
        },
        Rectangle {
            width: ASTEROID_SIZE,
            height: ASTEROID_SIZE - 5.0,
            color: BLUE,
            z_index: 1,
        },
        HurtBox {
            radius: ASTEROID_SIZE,
        },
        DamageDealer { dmg: ASTEROID_DMG },
        Team::Enemy,
    )
}
