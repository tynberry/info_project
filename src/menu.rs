use hecs::World;
use macroquad::prelude::*;

use crate::basic::{render::AssetManager, Position};

#[derive(Clone, Debug, Default)]
pub struct Title {
    pub text: String,
    pub font: &'static str,
    pub size: f32,
    pub color: Color,
}

//-----------------------------------------------------------------------------
//SYSTEM PART
//-----------------------------------------------------------------------------

pub fn render_title(world: &mut World, assets: &AssetManager) {
    for (_, (title, position)) in world.query_mut::<(&Title, &Position)>() {
        //get font to render
        let font = assets.get_font(title.font);
        //render it center aligned
        let dimensions = measure_text(title.text.as_str(), font, title.size as u16, 1.0);
        draw_text_ex(
            title.text.as_str(),
            position.x - dimensions.width / 2.0,
            position.y - dimensions.height / 2.0,
            TextParams {
                font,
                font_size: title.size as u16,
                font_scale: 1.0,
                color: title.color,
                ..Default::default()
            },
        )
    }
}
