use hecs::World;
use macroquad::prelude::*;

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

//-----------------------------------------------------------------------------
//SYSTEM PART
//-----------------------------------------------------------------------------

pub fn ensure_wrapping(world: &mut World) {
    for (_, wrap_pos) in world.query_mut::<&mut Position>().with::<&Wrapped>() {
        //if outside of screen tp them back
        //assumes position is center
        if wrap_pos.x > screen_width() {
            wrap_pos.x = 0.0;
        }
        if wrap_pos.x < 0.0{
            wrap_pos.x = screen_width();
        }

        if wrap_pos.y > screen_height() {
            wrap_pos.y = 0.0;
        }
        if wrap_pos.y < 0.0{
            wrap_pos.y = screen_height();
        }
    }
}