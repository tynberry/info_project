use hecs::World;
use macroquad::math::Vec2;

use super::Position;

#[derive(Clone, Copy, Debug, Default)]
pub struct LinearMotion {
    pub vel: Vec2,
}

//-----------------------------------------------------------------------------
//SYSTEM PART
//-----------------------------------------------------------------------------

pub fn apply_motion(world: &mut World, dt: f32) {
    for (_, (linear, linear_pos)) in world.query_mut::<(&LinearMotion, &mut Position)>() {
        linear_pos.x += linear.vel.x * dt;
        linear_pos.y += linear.vel.y * dt;
    }
}
