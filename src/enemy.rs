use hecs::{CommandBuffer, World};
use macroquad::prelude::*;

use crate::basic::{
    motion::LinearMotion, render::Rectangle, DamageDealer, DeleteOnWarp, Health, HitBox, HitEvent,
    HurtBox, Position, Team,
};

#[derive(Clone, Copy, Debug, Default)]
pub struct Enemy;

//------------------------------------------------------------------------------
//ENTITY CREATION
//------------------------------------------------------------------------------

const ASTEROID_HEALTH: f32 = 1.0;
const ASTEROID_SPEED: f32 = 100.0;
const ASTEROID_SIZE: f32 = 30.0;
const ASTEROID_DMG: f32 = 0.5;

pub fn create_asteroid(
    pos: Vec2,
    dir: Vec2,
) -> (
    Enemy,
    Position,
    LinearMotion,
    Rectangle,
    HitBox,
    HurtBox,
    Health,
    DamageDealer,
    Team,
    DeleteOnWarp,
) {
    (
        Enemy,
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
        HitBox {
            radius: ASTEROID_SIZE,
        },
        HurtBox {
            radius: ASTEROID_SIZE,
        },
        Health {
            max_hp: ASTEROID_HEALTH,
            hp: ASTEROID_HEALTH,
        },
        DamageDealer { dmg: ASTEROID_DMG },
        Team::Enemy,
        DeleteOnWarp,
    )
}

//------------------------------------------------------------------------------
//SYSTEM PART
//------------------------------------------------------------------------------

pub fn health(world: &mut World, events: &mut World, cmd: &mut CommandBuffer) {
    //get enemy view
    let enemy_query = &mut world.query::<&mut Health>().with::<&Enemy>();
    let mut enemy_view = enemy_query.view();
    //get events concerning the player
    let hit_events = events.query_mut::<&HitEvent>().into_iter();
    for (_, event) in hit_events {
        //get the enemy
        let Some(enemy_hp) = enemy_view.get_mut(event.who) else {
            continue;
        };
        //get damage
        let Ok(damage) = world.get::<&DamageDealer>(event.by) else {
            continue;
        };
        //apply it
        enemy_hp.hp -= damage.dmg;
        //check for death
        if enemy_hp.hp <= 0.0 {
            //TODO DEATH
            //despawn for now
            cmd.despawn(event.who);
        }
    }
}
