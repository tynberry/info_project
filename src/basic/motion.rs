use hecs::World;
use macroquad::math::{vec2, Vec2};

use super::Position;

const PIXEL_TO_METER: f32 = 0.01;

#[derive(Clone, Copy, Debug, Default)]
pub struct LinearMotion {
    pub vel: Vec2,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct PhysicsMotion {
    pub vel: Vec2,
    pub mass: f32,
}

impl PhysicsMotion {
    pub fn apply_force(&mut self, force: Vec2, dt: f32) {
        self.vel += force * dt / self.mass;
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct PhysicsDamping {
    pub mul_factor: f32,
    pub flat_factor: f32,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct ChargeSender {
    pub charge: f32,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct ChargeReceiver {
    pub charge: f32,
}

//-----------------------------------------------------------------------------
//SYSTEM PART
//-----------------------------------------------------------------------------

pub fn apply_motion(world: &mut World, dt: f32) {
    for (_, (linear, linear_pos)) in world.query_mut::<(&LinearMotion, &mut Position)>() {
        linear_pos.x += linear.vel.x * dt;
        linear_pos.y += linear.vel.y * dt;
    }

    for (_, (physics, physics_pos)) in world.query_mut::<(&PhysicsMotion, &mut Position)>() {
        physics_pos.x += physics.vel.x * dt;
        physics_pos.y += physics.vel.y * dt;
    }
}

pub fn apply_physics(world: &mut World, dt: f32) {
    //apply damping
    for (_, (physics, damping)) in world.query_mut::<(&mut PhysicsMotion, &PhysicsDamping)>() {
        //first mul factor
        physics.vel *= damping.mul_factor.powf(dt);
        //then flat factor
        if physics.vel.length_squared() <= damping.flat_factor {
            physics.vel = Vec2::ZERO;
        } else {
            physics.vel -= damping.flat_factor * dt * physics.vel.normalize_or_zero();
        }
    }

    //apply all charges O(n^2)
    for (a_ind, (a_charge, a_physics, a_pos)) in world
        .query::<(&ChargeReceiver, &mut PhysicsMotion, &Position)>()
        .into_iter()
    {
        for (b_ind, (b_charge, b_pos)) in world.query::<(&ChargeSender, &Position)>().into_iter() {
            //ignore same entities
            if a_ind == b_ind {
                continue;
            }
            //compute distance
            let distance = ((a_pos.x - b_pos.x).powi(2) + (a_pos.y - b_pos.y).powi(2)).sqrt();
            let distance = distance * PIXEL_TO_METER;
            if distance <= 0.01 {
                //distance too small to safely apply force
                continue;
            }
            //apply force
            let normal = vec2(a_pos.x - b_pos.x, a_pos.y - b_pos.y) / distance;
            a_physics.apply_force(
                (a_charge.charge * b_charge.charge / (distance.powi(2))) * normal,
                dt,
            );
        }
    }
}
