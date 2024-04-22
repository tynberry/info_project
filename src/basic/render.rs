//! Rendering objects and logic.

use enum_dispatch::enum_dispatch;
use hecs::World;
use macroquad::{
    audio::{load_sound, Sound},
    prelude::*,
};

use super::{Position, Rotation};

/// Manager of all the assets used.
/// Stores textures, fonts and sounds in one place so that they
/// can be accessed with simple `str` lookup.
#[derive(Debug, Default)]
pub struct AssetManager {
    /// Texture storage
    textures: fnv::FnvHashMap<&'static str, Texture2D>,
    /// Font storage
    fonts: fnv::FnvHashMap<&'static str, Font>,
    /// Sound storage
    sound: fnv::FnvHashMap<&'static str, Sound>,
}

impl AssetManager {
    /// Loads a texture from texture file (.png,...) into `AssetManager`.
    ///
    /// Returns an error when something went bad during loading.
    /// # Arguments
    /// * `id` - id using which the texture can be requested
    /// * `path` - path of the texture file
    pub async fn load_texture(
        &mut self,
        id: &'static str,
        path: &str,
    ) -> Result<(), macroquad::Error> {
        //load it
        let texture = load_texture(path).await?;
        //save it
        self.textures.insert(id, texture);
        Ok(())
    }

    /// Gets a texture from storage.
    ///
    /// Returns `None` if the texture is not present.
    /// # Arguments
    /// * `id` - id passed when loading the texture
    pub fn get_texture(&self, id: &'static str) -> Option<&Texture2D> {
        self.textures.get(id)
    }

    /// Loads a font from font file (.ttf) into `AssetManager`.
    ///
    /// Returns an error when something went bad during loading.
    /// # Arguments
    /// * `id` - id using which the font can be requested
    /// * `path` - path of the font file
    pub async fn load_font(
        &mut self,
        id: &'static str,
        path: &str,
    ) -> Result<(), macroquad::Error> {
        //load it
        let font = load_ttf_font(path).await?;
        //save it
        self.fonts.insert(id, font);
        Ok(())
    }

    /// Gets a font from storage.
    ///
    /// Returns `None` if the font is not present.
    /// # Arguments
    /// * `id` - id passed when loading the font
    pub fn get_font(&self, id: &'static str) -> Option<&Font> {
        self.fonts.get(id)
    }

    /// Loads a sound from sound file (.wav,...) into `AssetManager`.
    ///
    /// Returns an error when something went bad during loading.
    /// # Arguments
    /// * `id` - id using which the sound can be requested
    /// * `path` - path of the sound file
    pub async fn load_sound(
        &mut self,
        id: &'static str,
        path: &str,
    ) -> Result<(), macroquad::Error> {
        //load it
        let sound = load_sound(path).await?;
        //save it
        self.sound.insert(id, sound);
        Ok(())
    }

    /// Gets a sound from storage.
    ///
    /// Returns `None` if the sound is not present.
    /// # Arguments
    /// * `id` - id passed when loading the sound
    pub fn get_sound(&self, id: &'static str) -> Option<&Sound> {
        self.sound.get(id)
    }
}

//-----------------------------------------------------------------------------
//COMPONENT PART
//-----------------------------------------------------------------------------

/// Renders a rectangle centered at entity's position. 
#[derive(Clone, Copy, Debug)]
pub struct Rectangle {
    /// Width of the rectangle.
    pub width: f32,
    /// Height of the rectangle.
    pub height: f32,
    /// Color of the rectangle.
    pub color: Color,
    /// Z index the rectangle should be rendered at.
    pub z_index: i16,
}

impl Renderable for Rectangle {
    fn render(&self, pos: &Position, rotation: Option<&Rotation>, _: &AssetManager) {
        match rotation {
            Some(Rotation { angle }) => {
                draw_rectangle_ex(
                    pos.x,
                    pos.y,
                    self.width,
                    self.height,
                    DrawRectangleParams {
                        offset: vec2(0.5, 0.5),
                        rotation: *angle,
                        color: self.color,
                    },
                );
            }
            None => draw_rectangle(
                pos.x - self.width / 2.0,
                pos.y - self.height / 2.0,
                self.width,
                self.height,
                self.color,
            ),
        }
    }

    fn z_index(&self) -> i16 {
        self.z_index
    }
}

/// Renders a circle centered at entity's position.
#[derive(Clone, Copy, Debug)]
pub struct Circle {
    /// Radius of the circle.
    pub radius: f32,
    /// Color of the circle. 
    pub color: Color,
    /// Z index the circle should be rendered at.
    pub z_index: i16,
}

impl Renderable for Circle {
    fn render(&self, pos: &Position, _rotation: Option<&Rotation>, _: &AssetManager) {
        draw_circle(pos.x, pos.y, self.radius, self.color);
    }

    fn z_index(&self) -> i16 {
        self.z_index
    }
}

/// Renders a texture cented at entity's position.
#[derive(Clone, Debug)]
pub struct Sprite {
    /// Texture ID of the texture to render. 
    pub texture: &'static str,
    /// Scale of the texture. 
    pub scale: f32,
    /// Tint of the texture. 
    /// This color gets multiplied with texture's. 
    pub color: Color,
    /// Z index the texture should be rendered at.
    pub z_index: i16,
}

impl Renderable for Sprite {
    fn render(&self, pos: &Position, rotation: Option<&Rotation>, assets: &AssetManager) {
        //fetch texture
        let Some(texture) = assets.get_texture(self.texture) else {
            return;
        };
        //render itself
        let width = texture.width() * self.scale;
        let height = texture.height() * self.scale;

        draw_texture_ex(
            texture,
            pos.x - width / 2.0,
            pos.y - height / 2.0,
            self.color,
            DrawTextureParams {
                dest_size: Some(vec2(width, height)),
                rotation: rotation.map(|rot| rot.angle).unwrap_or(0.0),
                ..Default::default()
            },
        );
    }

    fn z_index(&self) -> i16 {
        self.z_index
    }
}

//-----------------------------------------------------------------------------
//TRAIT PART
//-----------------------------------------------------------------------------

/// Types that can rendered on the screen.
#[enum_dispatch]
trait Renderable {
    /// Renders the type. 
    /// # Arguments 
    /// * `pos` - position of the center 
    /// * `rotation` - rotation of the object 
    /// * `assets` - `AssetManager` containg all the assets
    fn render(&self, pos: &Position, rotation: Option<&Rotation>, assets: &AssetManager);
    /// Returns an index of a z layer the type should be rendered at.
    /// Higher z index makes the type rendered over types with lower z index.
    fn z_index(&self) -> i16 {
        0
    }
}

#[enum_dispatch(Renderable)]
enum RenderJobs {
    Rectangle,
    Circle,
    Sprite,
}

//-----------------------------------------------------------------------------
//SYSTEM PART
//-----------------------------------------------------------------------------

/// Renders `Rectangle`s, `Circle`s and `Sprite`s on the screen.
pub fn render_all(world: &mut World, assets: &AssetManager) {
    //gather all render jobs
    //circles
    let mut jobs: Vec<(RenderJobs, Position, Option<Rotation>)> = world
        .query_mut::<(&Circle, &Position, Option<&Rotation>)>()
        .into_iter()
        .map(|(_, (c, p, r))| (Into::<RenderJobs>::into(*c), *p, r.copied()))
        .collect();
    //rectangles
    jobs.extend(
        world
            .query_mut::<(&Rectangle, &Position, Option<&Rotation>)>()
            .into_iter()
            .map(|(_, (c, p, r))| (Into::<RenderJobs>::into(*c), *p, r.copied())),
    );
    //sprites
    jobs.extend(
        world
            .query_mut::<(&Sprite, &Position, Option<&Rotation>)>()
            .into_iter()
            .map(|(_, (c, p, r))| (Into::<RenderJobs>::into(c.clone()), *p, r.copied())),
    );
    //sort them by z_index
    jobs.sort_unstable_by_key(|a| a.0.z_index());
    //render all of them
    for job in jobs {
        job.0.render(&job.1, job.2.as_ref(), assets);
    }
}
