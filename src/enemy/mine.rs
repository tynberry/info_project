use std::f32::consts::PI;

use hecs::{CommandBuffer, EntityBuilder, World};
use macroquad::prelude::*;

use crate::{
    basic::{
        fx::{FxManager, Particle},
        motion::{ChargeReceiver, ChargeSender, KnockbackDealer, LinearTorgue, PhysicsMotion},
        render::Sprite,
        DamageDealer, DeleteOnWarp, Health, HitBox, HurtBox, Position, Rotation, Team,
    },
    projectile::ProjectileType,
};

use super::Enemy;

const MINE_HEALTH: f32 = 0.5;
const MINE_SPEED: f32 = 60.0;
const MINE_MASS: f32 = 4.0;

const MINE_SIZE: f32 = 60.0;

const MINE_DMG: f32 = 1.5;

pub const MINE_TEX_NEUTRAL: &str = "mine";
pub const MINE_TEX_POSITIVE: &str = "mine_plus";
pub const MINE_TEX_NEGATIVE: &str = "mine_negative";

const MINE_FORCE: f32 = 200.0;
const MINE_FORCE_F_RADIUS: f32 = 100.0;
const MINE_FORCE_RADIUS: f32 = 200.0;

const MINE_KNOCKBACK: f32 = 250.0;

const MINE_PROJ_SPEED: f32 = 200.0;
const MINE_PROJ_DMG: f32 = 1.5;

#[derive(Clone, Copy, Debug, Default)]
pub struct Mine {
    pub charge: i8,
}

//-----------------------------------------------------------------------------
//ENTITY CREATION
//-----------------------------------------------------------------------------

pub fn create_mine(pos: Vec2, dir: Vec2, charge: i8) -> EntityBuilder {
    let texture = match charge {
        1 => MINE_TEX_POSITIVE,
        -1 => MINE_TEX_NEGATIVE,
        0 => MINE_TEX_NEUTRAL,
        _ => panic!("Charge can only be 0,1,-1"),
    };

    let mut builder = EntityBuilder::default();

    builder.add_bundle((
        Enemy,
        Mine { charge },
        Position { x: pos.x, y: pos.y },
        Rotation {
            angle: fastrand::f32() * 2.0 * PI,
        },
        LinearTorgue {
            speed: fastrand::f32() * 1.0 - 0.50,
        },
        PhysicsMotion {
            vel: dir * MINE_SPEED,
            mass: MINE_MASS,
        },
        Sprite {
            texture,
            scale: MINE_SIZE / 512.0,
            color: WHITE,
            z_index: 0,
        },
        HitBox {
            radius: MINE_SIZE / 2.0,
        },
    ));
    builder.add_bundle((
        HurtBox {
            radius: MINE_SIZE / 2.0,
        },
        Health {
            max_hp: MINE_HEALTH,
            hp: MINE_HEALTH,
        },
        DamageDealer { dmg: MINE_DMG },
        Team::Enemy,
        DeleteOnWarp,
        ChargeSender {
            force: MINE_FORCE * charge as f32,
            full_radius: MINE_FORCE_F_RADIUS,
            no_radius: MINE_FORCE_RADIUS,
        },
        ChargeReceiver {
            multiplier: 0.5 * charge as f32,
        },
        KnockbackDealer {
            force: MINE_KNOCKBACK,
        },
    ));
    builder
}

//-----------------------------------------------------------------------------
//SYSTEM PART
//-----------------------------------------------------------------------------

pub fn mine_death(world: &mut World, cmd: &mut CommandBuffer, fx: &mut FxManager) {
    for (_, (health, pos, mine)) in world.query::<(&Health, &Position, &Mine)>().into_iter() {
        //check if it is dead
        if health.hp <= 0.0 {
            //spawn many smaller asteroids of the same charge
            for i in 0..24 {
                let dir = Vec2::from_angle(PI / 4.0 * (i as f32)).rotate(Vec2::X);
                let speed = match i {
                    x if (0..8).contains(&x) => MINE_PROJ_SPEED,
                    x if (8..16).contains(&x) => MINE_PROJ_SPEED / 2.0,
                    x if (16..24).contains(&x) => MINE_PROJ_SPEED / 3.0,
                    _ => unreachable!(),
                };

                cmd.spawn(crate::projectile::create_projectile(
                    vec2(pos.x, pos.y),
                    dir * speed,
                    MINE_PROJ_DMG,
                    Team::Enemy,
                    ProjectileType::Medium {
                        charge: mine.charge,
                    },
                ));
            }
            //spawn random particles on destroy
            for i in 1..5 {
                fx.burst_particles(
                    Particle {
                        pos: vec2(pos.x, pos.y),
                        vel: vec2(10.0 * i as f32, 0.0),
                        life: 1.0,
                        max_life: 1.0,
                        min_size: 0.0,
                        max_size: 5.0,
                        color: match mine.charge {
                            1 => RED,
                            0 => GREEN,
                            -1 => Color::new(0.0, 1.0, 1.0, 1.0),
                            _ => {
                                unimplemented!(
                                    "Followers do not support charges different than 0,1,-1"
                                )
                            }
                        },
                    },
                    5.0,
                    2.0 * PI,
                    3 * i,
                );
            }
        }
    }
}
