//! Particle system logic.

use std::collections::VecDeque;

use macroquad::prelude::*;

/// Particle to render
#[derive(Clone, Copy, Debug)]
pub struct Particle {
    /// Position of the particle.
    pub pos: Vec2,
    /// Velocity of the particle.
    pub vel: Vec2,
    /// Amount of time before the particle is deleted.
    /// It can be deleted sooner if too many particles were spawned before that.
    pub life: f32,
    /// Amount of time the particle can live at most.
    pub max_life: f32,
    /// Size of the particle when `life` is zero.
    pub min_size: f32,
    /// Size of the particle when `life == max_life`.
    pub max_size: f32,
    /// Color of the particle.
    pub color: Color,
}

/// Manager of all the particles.
#[derive(Debug)]
pub struct FxManager {
    /// Queue of all the particles to render.
    particles: VecDeque<Particle>,
    /// Max particles that can be spawned at once.
    pub max_particles: usize,
}

impl FxManager {
    /// Create a particle manager.
    /// # Arguments
    /// * `max_particle` - sets how many particles can be alive at once
    pub fn new(max_particles: usize) -> Self {
        Self {
            particles: VecDeque::with_capacity(max_particles),
            max_particles,
        }
    }

    /// Adds a particle to the manager.
    /// Removes the oldest particle if space is not available.
    /// # Arguments
    /// * `particle` - particle to add
    pub fn add_particle(&mut self, particle: Particle) {
        //make space if not enough space
        if self.particles.len() == self.max_particles {
            self.particles.pop_back();
        }
        //add particle
        self.particles.push_front(particle);
    }

    /// Spawns many particles with different velocities.
    /// Removes old particles if space is not enough.
    /// # Arguments
    /// * `base` - base particle to add
    /// * `vel_deviation` - random change in the base velocitie's length
    /// * `angle_deviation` - random change in the base velocitie's direction, in radians
    /// * `count` - how many particles should it spawn
    pub fn burst_particles(
        &mut self,
        base: Particle,
        vel_deviation: f32,
        angle_deviation: f32,
        count: usize,
    ) {
        //base velocity information to construct new velocity vectors
        let vel_normal = base.vel.normalize_or_zero();
        let vel_length = base.vel.length();

        //spawn `count` particles
        for _ in 0..count {
            //apply angle deviation
            let vel = Vec2::from_angle(fastrand::f32() * 2.0 * angle_deviation - angle_deviation)
                .rotate(vel_normal)
                * (vel_length + fastrand::f32() * 2.0 * vel_deviation - vel_deviation);
            //spawn it
            let mut particle = base;
            particle.vel = vel;
            self.add_particle(particle);
        }
    }

    /// Deletes all the particles.
    pub fn clear_particles(&mut self) {
        self.particles.clear();
    }

    /// Updates all the particles.
    /// # Arguments
    /// * `dt` - delta time  
    pub fn update_particles(&mut self, dt: f32) {
        for particle in &mut self.particles {
            particle.pos += particle.vel * dt;
            particle.life -= dt;
        }

        //delete too old particles
        self.particles.retain(|part| part.life > 0.0);
    }

    /// Render all the particles.
    pub fn render_particles(&self) {
        for particle in &self.particles {
            let size = (particle.life / particle.max_life)
                * (particle.max_size - particle.min_size)
                + particle.min_size;
            draw_rectangle(
                particle.pos.x - size / 2.0,
                particle.pos.y - size / 2.0,
                size,
                size,
                particle.color,
            );
        }
    }
}
