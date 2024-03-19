use hecs::{CommandBuffer, Entity, World};
use macroquad::prelude::*;

pub mod health;

//-----------------------------------------------------------------------------
//UTILS PART
//-----------------------------------------------------------------------------

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub enum Team {
    #[default]
    Neutral,
    Player,
    Enemy,
}

impl Team {
    #[inline]
    pub fn can_hurt(&self, other: &Team) -> bool {
        self != other
    }
}

//-----------------------------------------------------------------------------
//COMPONENT PART
//-----------------------------------------------------------------------------

#[derive(Clone, Copy, Debug, Default)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Rotation {
    pub angle: f32,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Wrapped;

#[derive(Clone, Copy, Debug, Default)]
pub struct DeleteOnWarp;

//DAMAGE COMPONENTS

#[derive(Clone, Copy, Debug, Default)]
pub struct Health {
    pub max_hp: f32,
    pub hp: f32,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct DamageDealer {
    pub dmg: f32,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct HurtBox {
    pub radius: f32,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct HitBox {
    pub radius: f32,
}

//-----------------------------------------------------------------------------
//EVENTS
//-----------------------------------------------------------------------------

#[derive(Clone, Copy, Debug)]
pub struct HitEvent {
    pub who: Entity,
    pub by: Entity,
}

//#[derive(Clone, Copy, Debug)]
//pub struct HurtEvent {
//    who: Entity,
//    by: Entity,
//}

//-----------------------------------------------------------------------------
//SYSTEM PART
//-----------------------------------------------------------------------------

pub fn ensure_wrapping(world: &mut World, cmd: &mut CommandBuffer) {
    for (_, wrap_pos) in world.query_mut::<&mut Position>().with::<&Wrapped>() {
        //if outside of screen tp them back
        //assumes position is center
        if wrap_pos.x > screen_width() {
            wrap_pos.x = 0.0;
        }
        if wrap_pos.x < 0.0 {
            wrap_pos.x = screen_width();
        }

        if wrap_pos.y > screen_height() {
            wrap_pos.y = 0.0;
        }
        if wrap_pos.y < 0.0 {
            wrap_pos.y = screen_height();
        }
    }

    for (wrap_id, wrap_pos) in world.query_mut::<&mut Position>().with::<&DeleteOnWarp>() {
        //if outside of screen tp them back
        //assumes position is center
        if wrap_pos.x > screen_width() + 100.0 {
            cmd.despawn(wrap_id);
        }
        if wrap_pos.x < -100.0 {
            cmd.despawn(wrap_id);
        }

        if wrap_pos.y > screen_height() + 100.0 {
            cmd.despawn(wrap_id);
        }
        if wrap_pos.y < -100.0 {
            cmd.despawn(wrap_id);
        }
    }
}

//DAMAGE SYSTEMS

pub fn ensure_damage(world: &mut World, events: &mut World) {
    //iterate through all hitable
    for (hit_id, (hit_pos, hit_box, hit_team)) in
        world.query::<(&Position, &HitBox, &Team)>().into_iter()
    {
        //iterate through all hurtting
        for (hurt_id, (hurt_pos, hurt_box, hurt_team)) in
            world.query::<(&Position, &HurtBox, &Team)>().into_iter()
        {
            //are they compatible?
            if !hurt_team.can_hurt(hit_team) {
                continue;
            }

            //are they touching?
            let dx = hit_pos.x - hurt_pos.x;
            let dy = hit_pos.y - hurt_pos.y;
            if dx * dx + dy * dy < (hurt_box.radius + hit_box.radius).powi(2) {
                //add hit event
                events.spawn((HitEvent {
                    who: hit_id,
                    by: hurt_id,
                },));
            }
        }
    }
}
