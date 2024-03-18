use hecs::World;
use macroquad::prelude::*;

use crate::basic::{Position, Team};

#[derive(Clone, Copy, Debug)]
pub struct Projectile {
    pub size: f32,
    pub vel: Vec2,
    pub team: Team,
}

impl Projectile {
    pub fn new(size: f32) -> Self {
        Self {
            size,
            vel: Vec2::new(0.0, 0.0),
            team: Team::Player,
        }
    }
}

//-----------------------------------------------------------------------------
//SYSTEM PART
//-----------------------------------------------------------------------------
pub fn motion(world: &mut World, dt: f32) {
    //move all particles
    for (_, (proj, proj_pos)) in world.query_mut::<(&Projectile, &mut Position)>() {
        proj_pos.x += proj.vel.x * dt;
        proj_pos.y += proj.vel.y * dt;
    }
}

pub fn render(world: &mut World) {
    //render all projectiles
    for (_, (proj, proj_pos)) in world.query_mut::<(&Projectile, &Position)>() {
        draw_circle(proj_pos.x, proj_pos.y, proj.size, GREEN);
    }
}
