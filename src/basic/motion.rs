//! Motion and physics components and systems.
use hecs::World;
use macroquad::{
    audio::{self, PlaySoundParams},
    math::{vec2, Vec2},
};

use super::{render::AssetManager, HitEvent, Position, Rotation};

/// Moves an entity in a linear way.
/// It does not accelerate, decelerate, change directions
/// after being set nor is affected by physics, knockback or charges.
#[derive(Clone, Copy, Debug, Default)]
pub struct LinearMotion {
    /// Velocity of the entity.
    pub vel: Vec2,
}

/// Component that rotates an entity constantly.
/// It does not change speed nor direction after being set.
#[derive(Clone, Copy, Debug, Default)]
pub struct LinearTorgue {
    /// Speed of rotation in radians per second.
    pub speed: f32,
}

/// Introduces an entity into physics simulation.
/// Entity with this component can be affected by charges or knockback.
#[derive(Clone, Copy, Debug, Default)]
pub struct PhysicsMotion {
    /// Velocity of the entity.
    pub vel: Vec2,
    /// Mass of the entity.
    /// Affects the velocity change by a force.
    pub mass: f32,
}

impl PhysicsMotion {
    /// Applies a force on the entity.
    /// # Arguments
    /// * `force` - the force to apply
    /// * `dt` - how long was the force be applied
    pub fn apply_force(&mut self, force: Vec2, dt: f32) {
        self.vel += force * dt / self.mass;
    }
}

/// Makes an entity to slow down its motions on its own.
/// It only dampens [PhysicsMotion].
#[derive(Clone, Copy, Debug, Default)]
pub struct PhysicsDamping {
    /// By this factor the entity's velocity is multiplied
    /// every second. It is integrated with respect to time.
    pub mul_factor: f32,
    /// By how many length units the entity's velocity is
    /// being reduced every second. It is integrated with
    /// respect to time.
    ///
    /// This factor cannot make the entity's  
    /// velocity face opposite direction than it already faced.
    pub flat_factor: f32,
}

/// Limits an entity's velocity to some amount.
/// It only affects [PhysicsMotion].
#[derive(Clone, Copy, Debug, Default)]
pub struct MaxVelocity {
    /// Max velocity the entity can achieve.
    pub max_velocity: f32,
}

/// Makes an entity produce electric field.
/// This field affects all entities with [ChargeReceiver].
#[derive(Clone, Copy, Debug, Default)]
pub struct ChargeSender {
    /// Force that is applied on all affected entites.
    pub force: f32,
    /// Distance from the entity where the force is applied
    /// at full strength.
    pub full_radius: f32,
    /// Distance from the entity where the force is first zero.
    /// All entites closer than `no_radius` are affected by force.
    pub no_radius: f32,
}

/// Makes an entity respond to electric fields.
#[derive(Clone, Copy, Debug, Default)]
pub struct ChargeReceiver {
    /// Multiplier to the force received.
    pub multiplier: f32,
}

/// Makes an entity temporalily immune to charge forces.
#[derive(Clone, Copy, Debug, Default)]
pub struct ChargeDisable {
    /// Time before the entity becomes affected by charges.
    pub timer: f32,
}

/// Makes an entity deal knockback to other entities.
/// This applies when any two entities collide with each other
/// no matter their team.
#[derive(Clone, Copy, Debug, Default)]
pub struct KnockbackDealer {
    /// Force magnitude that is dealt on collision.
    pub force: f32,
}

//-----------------------------------------------------------------------------
//SYSTEM PART
//-----------------------------------------------------------------------------

/// Add [LinearMotion], [LinearTorgue] and [PhysicsMotion]
/// velocities to entities' positions and/or rotations.
pub fn apply_motion(world: &mut World, dt: f32) {
    //apply linear motion
    for (_, (linear, pos)) in world.query_mut::<(&LinearMotion, &mut Position)>() {
        pos.x += linear.vel.x * dt;
        pos.y += linear.vel.y * dt;
    }

    //apply linear torgue
    for (_, (torgue, rotation)) in world.query_mut::<(&LinearTorgue, &mut Rotation)>() {
        rotation.angle += torgue.speed * dt;
    }

    //apply physics motion
    for (_, (physics, pos)) in world.query_mut::<(&PhysicsMotion, &mut Position)>() {
        pos.x += physics.vel.x * dt;
        pos.y += physics.vel.y * dt;
    }
}

/// Advance physics simulation.
/// Handles the logic of [PhysicsDamping], [MaxVelocity] and charges.
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

    //apply max velocity
    for (_, (vel, max)) in world.query_mut::<(&mut PhysicsMotion, &MaxVelocity)>() {
        if vel.vel.length_squared() > max.max_velocity.powi(2) {
            vel.vel = vel.vel.normalize_or_zero() * max.max_velocity;
        }
    }

    //apply all charges O(n^2)
    //iterate through all charge receivers
    for (a_ind, (a_charge, a_physics, a_pos, a_disable)) in world
        .query::<(
            &ChargeReceiver,
            &mut PhysicsMotion,
            &Position,
            Option<&mut ChargeDisable>,
        )>()
        .into_iter()
    {
        //is charge receiving disabled?
        if let Some(disabler) = a_disable {
            disabler.timer -= dt;
            if disabler.timer > 0.0 {
                continue;
            }
        }

        //apply all charge sources
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

/// Applies knockback dealt by [KnockbackDealer].
///
/// Only affects entities with [PhysicsMotion].
pub fn apply_knockback(world: &mut World, event: &mut World, assets: &AssetManager) {
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

        //get required components from the dealer
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

        //get required components from the victim
        let Some(mut victim_vel) = victim_ent.get::<&mut PhysicsMotion>() else {
            continue;
        };

        let Some(victim_pos) = victim_ent.get::<&Position>() else {
            continue;
        };
        //deal force
        let normal = vec2(victim_pos.x - deal_pos.x, victim_pos.y - deal_pos.y).normalize_or_zero();
        victim_vel.apply_force(normal * deal.force, 1.0);
        //play sound to knockback
        audio::play_sound(
            assets.get_sound("knockback").unwrap(),
            PlaySoundParams {
                looped: false,
                volume: 0.5,
            },
        );
    }
}
