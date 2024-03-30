use hecs::{CommandBuffer, World};
use macroquad::prelude::*;

pub mod fx;
pub mod health;
pub mod motion;
pub mod render;

pub use health::*;

use self::render::{AssetManager, Sprite};

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

//-----------------------------------------------------------------------------
//EVENTS
//-----------------------------------------------------------------------------

//-----------------------------------------------------------------------------
//SYSTEM PART
//-----------------------------------------------------------------------------

pub fn ensure_wrapping(world: &mut World, cmd: &mut CommandBuffer, assets: &AssetManager) {
    for (_, pos) in world.query_mut::<&mut Position>().with::<&Wrapped>() {
        //if outside of screen tp them back
        //assumes position is center
        if pos.x > screen_width() {
            pos.x = 0.0;
        }
        if pos.x < 0.0 {
            pos.x = screen_width();
        }

        if pos.y > screen_height() {
            pos.y = 0.0;
        }
        if pos.y < 0.0 {
            pos.y = screen_height();
        }
    }

    for (id, (pos, sprite)) in world
        .query_mut::<(&mut Position, Option<&Sprite>)>()
        .with::<&DeleteOnWarp>()
    {
        //calculate how far back it must be to be destroyed
        let pushback = 'here: {
            match sprite {
                Some(sprite) => {
                    //get underlying texture
                    let Some(texture) = assets.get_texture(sprite.texture) else {
                        break 'here 50.0;
                    };
                    //get biggest side and scale it
                    let side = texture.width().max(texture.height());
                    side * sprite.scale + 5.0
                }
                None => 50.0,
            }
        };
        //if outside of screen tp them back
        //assumes position is center
        if pos.x > screen_width() + pushback {
            cmd.despawn(id);
        }
        if pos.x < -pushback {
            cmd.despawn(id);
        }

        if pos.y > screen_height() + pushback {
            cmd.despawn(id);
        }
        if pos.y < -pushback {
            cmd.despawn(id);
        }
    }
}
