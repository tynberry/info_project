use hecs::World;
use macroquad::math::{vec2, Vec2};

use super::{HitEvent, Position, Rotation};

#[derive(Clone, Copy, Debug, Default)]
pub struct LinearMotion {
    pub vel: Vec2,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct LinearTorgue {
    pub speed: f32,
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
    pub force: f32,
    pub full_radius: f32,
    pub no_radius: f32,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct ChargeReceiver {
    pub multiplier: f32,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct KnockbackDealer {
    pub force: f32,
}

//-----------------------------------------------------------------------------
//SYSTEM PART
//-----------------------------------------------------------------------------

pub fn apply_motion(world: &mut World, dt: f32) {
    for (_, (linear, pos)) in world.query_mut::<(&LinearMotion, &mut Position)>() {
        pos.x += linear.vel.x * dt;
        pos.y += linear.vel.y * dt;
    }

    for (_, (torgue, rotation)) in world.query_mut::<(&LinearTorgue, &mut Rotation)>() {
        rotation.angle += torgue.speed * dt;
    }

    for (_, (physics, pos)) in world.query_mut::<(&PhysicsMotion, &mut Position)>() {
        pos.x += physics.vel.x * dt;
        pos.y += physics.vel.y * dt;
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
            //distance to small to safely get normal
            if distance <= 0.1 {
                continue;
            }
            //compute force portion over radius
            let force = if distance >= b_charge.no_radius {
                //no force
                continue;
            } else if distance > b_charge.full_radius {
                //partial force
                (b_charge.no_radius - distance) / (b_charge.no_radius - b_charge.full_radius)
                    * b_charge.force
            } else {
                //full force
                b_charge.force
            };
            //apply force
            let normal = vec2(a_pos.x - b_pos.x, a_pos.y - b_pos.y) / distance;
            a_physics.apply_force(a_charge.multiplier * force * normal, dt);
        }
    }
}

pub fn apply_knockback(world: &mut World, event: &mut World) {
    //for all events
    for (_, event) in event.query_mut::<&HitEvent>() {
        //is the producer equal to the consumer?
        if event.who == event.by {
            continue;
        }
        //is the producer a knockback dealer?
        let Ok(deal_ent) = world.entity(event.by) else {
            continue;
        };

        let Some(deal) = deal_ent.get::<&KnockbackDealer>() else {
            continue;
        };

        let Some(deal_pos) = deal_ent.get::<&Position>() else {
            continue;
        };
        //is the consumer a victim?
        let Ok(victim_ent) = world.entity(event.who) else {
            continue;
        };

        let Some(mut victim_vel) = victim_ent.get::<&mut PhysicsMotion>() else {
            continue;
        };

        let Some(victim_pos) = victim_ent.get::<&Position>() else {
            continue;
        };
        //deal force
        let normal = vec2(victim_pos.x - deal_pos.x, victim_pos.y - deal_pos.y).normalize_or_zero();
        victim_vel.apply_force(normal * deal.force, 1.0);
    }
}
