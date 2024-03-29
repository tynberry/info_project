use hecs::World;
use macroquad::prelude::*;

use crate::{
    basic::{render::AssetManager, Position},
    game::state::GameState,
};

#[derive(Clone, Debug)]
pub struct Title {
    pub text: String,
    pub font: &'static str,
    pub size: f32,
    pub color: Color,
}

#[derive(Clone, Copy, Debug)]
pub struct Button {
    pub width: f32,
    pub height: f32,
    pub neutral_color: Color,
    pub hover_color: Color,
    pub active_color: Color,
    pub clicked: bool,
}

#[derive(Clone, Copy, Debug)]
pub struct StartButton;
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
            position.y + dimensions.offset_y / 2.0,
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

pub fn button_colors(world: &mut World) {
    for (_, (position, button, title)) in world.query_mut::<(&Position, &mut Button, &mut Title)>()
    {
        //check for overlap
        let mouse_pos = mouse_position();
        let hover = mouse_pos.0 <= position.x + button.width / 2.0
            && mouse_pos.0 >= position.x - button.width / 2.0
            && mouse_pos.1 <= position.y + button.height / 2.0
            && mouse_pos.1 >= position.y - button.height / 2.0;
        let click = hover && is_mouse_button_pressed(MouseButton::Left);
        //set color
        title.color = if click {
            button.active_color
        } else if hover {
            button.hover_color
        } else {
            button.neutral_color
        };
        //set clicked
        button.clicked = click;
    }
}

pub fn handle_buttons(world: &mut World) -> Option<GameState> {
    for (_, button) in world.query_mut::<&Button>().with::<&StartButton>() {
        if button.clicked {
            return Some(GameState::Running);
        }
    }
    None
}
