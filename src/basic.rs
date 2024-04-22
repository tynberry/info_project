//! Basic, general types, that can be used to a wide range of entities.
use hecs::{CommandBuffer, World};
use macroquad::prelude::*;

pub mod fx;
pub mod health;
pub mod motion;
pub mod render;

pub use health::*;

use crate::{SPACE_HEIGHT, SPACE_WIDTH};

use self::render::{AssetManager, Sprite};

//-----------------------------------------------------------------------------
//UTILS PART
//-----------------------------------------------------------------------------

///Team of the entity.
///This determines hurting between entities.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub enum Team {
    #[default]
    Neutral,
    Player,
    Enemy,
}

impl Team {
    /// Can the `self` team hurt the `other` team.
    #[inline]
    pub fn can_hurt(&self, other: &Team) -> bool {
        self != other
    }
}

//-----------------------------------------------------------------------------
//COMPONENT PART
//-----------------------------------------------------------------------------

/// Position of an entity in World coordinates.
/// Represents the center of the entity.
#[derive(Clone, Copy, Debug, Default)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

/// Rotation of an entity along its center.
#[derive(Clone, Copy, Debug, Default)]
pub struct Rotation {
    pub angle: f32,
}

/// Marker of entites that should wrap around when going out of bounds.
#[derive(Clone, Copy, Debug, Default)]
pub struct Wrapped;

/// Marker of entities that should be deleted entirely when out of bounds.
#[derive(Clone, Copy, Debug, Default)]
pub struct DeleteOnWarp;

//-----------------------------------------------------------------------------
//EVENTS
//-----------------------------------------------------------------------------

//-----------------------------------------------------------------------------
//SYSTEM PART
//-----------------------------------------------------------------------------

/// Handles the wrapping and deletion of entities marked by Wrapped or DeleteOnWarp.
pub fn ensure_wrapping(world: &mut World, cmd: &mut CommandBuffer, assets: &AssetManager) {
    //handle Wrapped wraping
    for (_, pos) in world.query_mut::<&mut Position>().with::<&Wrapped>() {
        //if outside of screen tp them back
        //assumes position is center
        if pos.x > SPACE_WIDTH {
            pos.x = 0.0;
        }
        if pos.x < 0.0 {
            pos.x = SPACE_WIDTH;
        }

        if pos.y > SPACE_HEIGHT {
            pos.y = 0.0;
        }
        if pos.y < 0.0 {
            pos.y = SPACE_HEIGHT;
        }
    }

    //handle DeleteOnWarp deleting
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
        //if outside of screen tp delete them
        //assumes position is center
        if pos.x > SPACE_WIDTH + pushback {
            cmd.despawn(id);
        }
        if pos.x < -pushback {
            cmd.despawn(id);
        }

        if pos.y > SPACE_HEIGHT + pushback {
            cmd.despawn(id);
        }
        if pos.y < -pushback {
            cmd.despawn(id);
        }
    }
}
