use std::collections::VecDeque;

use macroquad::prelude::*;

#[derive(Clone, Copy, Debug)]
pub struct Particle {
    pub pos: Vec2,
    pub vel: Vec2,
    pub life: f32,
    pub max_life: f32,
    pub min_size: f32,
    pub max_size: f32,
    pub color: Color,
}

#[derive(Debug)]
pub struct FxManager {
    particles: VecDeque<Particle>,
    pub max_particles: usize,
}

impl FxManager {
    pub fn new(max_particles: usize) -> Self {
        Self {
            particles: VecDeque::with_capacity(max_particles),
            max_particles,
        }
    }

    pub fn add_particle(&mut self, particle: Particle) {
        //make space if not enough space
        if self.particles.len() == self.max_particles {
            self.particles.pop_back();
        }
        //add particle
        self.particles.push_front(particle);
    }

    pub fn burst_particles(&mut self, base: Particle, angle_deviation: f32, count: usize) {
        for _ in 0..count {
            //apply angle deviation
            let vel = Vec2::from_angle(fastrand::f32() * 2.0 * angle_deviation - angle_deviation)
                .rotate(base.vel);
            //spawn it
            let mut particle = base;
            particle.vel = vel;
            self.add_particle(particle);
        }
    }

    pub fn clear_particles(&mut self) {
        self.particles.clear();
    }

    pub fn update_particles(&mut self, dt: f32) {
        for particle in &mut self.particles {
            particle.pos += particle.vel * dt;
            particle.life -= dt;
        }

        //delete too old particles
        self.particles.retain(|part| part.life > 0.0);
    }

    pub fn render_particles(&self) {
        for particle in &self.particles {
            let size = (particle.life / particle.max_life)
                * (particle.max_size - particle.min_size)
                + particle.min_size;
            draw_rectangle(particle.pos.x, particle.pos.y, size, size, particle.color);
        }
    }
}
