use hecs::{CommandBuffer, World};
use macroquad::prelude::*;

//-----------------------------------------------------------------------------
//UTILS PART
//-----------------------------------------------------------------------------

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Team {
    Player,
    Enemy,
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
