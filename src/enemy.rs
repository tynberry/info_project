//! General enemy components

pub mod asteroid;
pub mod charged;
pub mod follower;
pub mod mine;

pub use asteroid::*;

use hecs::{CommandBuffer, World};

use crate::basic::{DamageDealer, Health, HitEvent};

///Marker of enemy entities.
///Every enemy should have this marker.
#[derive(Clone, Copy, Debug, Default)]
pub struct Enemy;

//------------------------------------------------------------------------------
//SYSTEM PART
//------------------------------------------------------------------------------

/// Handles hurting of enemies by hostile hurt events.
/// Calculates resulting health and despawns dead (hp <= 0.0) enemies.
pub fn health(world: &mut World, events: &mut World, cmd: &mut CommandBuffer) {
    {
        //get enemy view
        let enemy_query = &mut world.query::<&mut Health>().with::<&Enemy>();
        let mut enemy_view = enemy_query.view();
        //get events concerning the player
        let hit_events = events.query_mut::<&HitEvent>().into_iter();
        for (_, event) in hit_events {
            //can be hurt by it?
            if !event.can_hurt {
                continue;
            }
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
        }
    }

    //despawn dead enemies
    for (enemy_id, health) in world.query_mut::<&Health>().with::<&Enemy>() {
        if health.hp <= 0.0 {
            cmd.despawn(enemy_id);
        }
    }
}
