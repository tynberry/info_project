use std::f32::consts::PI;

use hecs::{CommandBuffer, Entity, EntityBuilder, World};
use macroquad::prelude::*;

use crate::{
    basic::{
        fx::{FxManager, Particle},
        motion::{
            ChargeReceiver, ChargeSender, KnockbackDealer, LinearTorgue, MaxVelocity, PhysicsMotion,
        },
        render::Sprite,
        DamageDealer, DeleteOnWarp, Health, HitBox, HurtBox, Position, Rotation, Team,
    },
    player::Player,
    projectile::{self, ProjectileType},
    xp::BurstXpOnDeath,
};

use super::asteroid::*;
use super::{Enemy, ASTEROID_TEX_NEGATIVE, ASTEROID_TEX_POSITIVE};

pub const ASTEROID_OUTLINE_TEX: &str = "asteroid_outline";
const ASTEROID_OUTLINE_SCALE: f32 = ASTEROID_SIZE / 544.0;

const ASTEROID_CHARGED_FIRE_COOLDOWN: f32 = 4.0;
const ASTEROID_CHARGED_PROJ_DMG: f32 = 1.5;
const ASTEROID_CHARGED_PROJ_SPEED: f32 = 180.0;

const ASTEROID_CHARGED_XP: u32 = 15;

#[derive(Clone, Copy, Debug)]
pub struct ChargedAsteroid {
    pub cooldown: f32,
    pub outline: Entity,
    pub charge: i8,
}

//-----------------------------------------------------------------------------
//ENTITY CREATION
//-----------------------------------------------------------------------------

#[allow(clippy::type_complexity)]
pub fn create_supercharged_asteroid(
    pos: Vec2,
    dir: Vec2,
    charge: i8,
) -> impl FnOnce(&World, &mut CommandBuffer) {
    let texture = if charge > 0 {
        ASTEROID_TEX_POSITIVE
    } else {
        ASTEROID_TEX_NEGATIVE
    };

    let angle = fastrand::f32() * 2.0 * PI;

    let mut charged_builder = EntityBuilder::default();

    charged_builder.add_bundle((
        Enemy,
        Position { x: pos.x, y: pos.y },
        Rotation { angle },
        LinearTorgue {
            speed: fastrand::f32() * 1.0 - 0.50,
        },
        PhysicsMotion {
            vel: dir * ASTEROID_SPEED,
            mass: ASTEROID_MASS,
        },
        Sprite {
            texture,
            scale: ASTEROID_SCALE,
            color: WHITE,
            z_index: 0,
        },
        HitBox {
            radius: ASTEROID_SIZE / 2.0,
        },
    ));
    charged_builder.add_bundle((
        HurtBox {
            radius: ASTEROID_SIZE / 2.0,
        },
        Health {
            max_hp: ASTEROID_HEALTH,
            hp: ASTEROID_HEALTH,
        },
        DamageDealer { dmg: ASTEROID_DMG },
        Team::Enemy,
        DeleteOnWarp,
        ChargeSender {
            force: ASTEROID_FORCE * charge as f32 / 4.0,
            full_radius: 0.0,
            no_radius: ASTEROID_FORCE_F_RADIUS / 1.5,
        },
        ChargeReceiver {
            multiplier: charge as f32,
        },
        KnockbackDealer {
            force: ASTEROID_KNOCKBACK,
        },
        BurstXpOnDeath {
            amount: ASTEROID_CHARGED_XP,
        },
        MaxVelocity {
            max_velocity: ASTEROID_SPEED * 2.0,
        },
    ));

    move |world, cmd| {
        //get outline entity
        let outline_id = world.reserve_entity();
        //embed into charged asteroid
        charged_builder.add(ChargedAsteroid {
            cooldown: ASTEROID_CHARGED_FIRE_COOLDOWN,
            outline: outline_id,
            charge,
        });
        //spawn outline
        cmd.insert(
            outline_id,
            (
                Sprite {
                    texture: ASTEROID_OUTLINE_TEX,
                    scale: ASTEROID_OUTLINE_SCALE,
                    color: BLACK,
                    z_index: 1,
                },
                Position { x: pos.x, y: pos.y },
                Rotation { angle },
            ),
        );
        //spawn charged asteroid
        cmd.spawn(charged_builder.build());
    }
}

//-----------------------------------------------------------------------------
//SYSTEM PART
//-----------------------------------------------------------------------------

pub fn supercharged_asteroid_ai(world: &mut World, cmd: &mut CommandBuffer, dt: f32) {
    //get player pos
    let (_, &player_pos) = world
        .query_mut::<&Position>()
        .with::<&Player>()
        .into_iter()
        .next()
        .unwrap();

    for (_, (charged, pos)) in world.query_mut::<(&mut ChargedAsteroid, &Position)>() {
        //fire logic
        charged.cooldown -= dt;
        if charged.cooldown <= 0.0 {
            charged.cooldown = ASTEROID_CHARGED_FIRE_COOLDOWN;

            let delta_x = player_pos.x - pos.x;
            let delta_y = player_pos.y - pos.y;
            let delta = vec2(delta_x, delta_y).normalize_or_zero();

            cmd.spawn(projectile::create_projectile(
                vec2(pos.x, pos.y),
                delta * ASTEROID_CHARGED_PROJ_SPEED,
                ASTEROID_CHARGED_PROJ_DMG,
                Team::Enemy,
                ProjectileType::Medium {
                    charge: charged.charge,
                },
            ));
        }
    }
}

pub fn supercharged_asteroid_death(world: &mut World, cmd: &mut CommandBuffer) {
    for (_, (charged, health)) in world.query_mut::<(&ChargedAsteroid, &Health)>() {
        if health.hp <= 0.0 {
            cmd.despawn(charged.outline);
        }
    }
}

pub fn supercharged_asteroid_visual(world: &mut World, fx: &mut FxManager) {
    //CHARGING OUTLINE
    for (_, (charged, pos, angle)) in world
        .query::<(&ChargedAsteroid, &Position, &Rotation)>()
        .into_iter()
    {
        //get your outline
        let mut outline = world
            .query::<(&mut Position, &mut Rotation, &mut Sprite)>()
            .without::<&ChargedAsteroid>();
        let mut outline = outline.view();
        let (outline_pos, outline_angle, outline_sprite) =
            outline.get_mut(charged.outline).unwrap();

        outline_pos.x = pos.x;
        outline_pos.y = pos.y;

        outline_angle.angle = angle.angle;

        let color_unit = (1.0 - charged.cooldown / ASTEROID_CHARGED_FIRE_COOLDOWN).min(1.0);
        outline_sprite.color = if charged.charge > 0 {
            Color {
                r: color_unit,
                a: 1.0,
                ..Default::default()
            }
        } else {
            Color {
                r: 0.0,
                g: color_unit,
                b: color_unit,
                a: 1.0,
            }
        }
    }
    //DEATH PARTICLES
    for (_, (health, pos)) in world
        .query_mut::<(&Health, &Position)>()
        .with::<&Asteroid>()
    {
        //check if it is dead
        if health.hp <= 0.0 {
            //spawn random particles on destroy
            for i in 1..=2 {
                fx.burst_particles(
                    Particle {
                        pos: vec2(pos.x, pos.y),
                        vel: vec2(30.0 * i as f32, 0.0),
                        life: 1.0,
                        max_life: 1.0,
                        min_size: 0.0,
                        max_size: 12.0,
                        color: LIGHTGRAY,
                    },
                    14.0,
                    2.0 * PI,
                    4 * i,
                );
            }
        }
    }
}
