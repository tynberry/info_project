//! Contains components required to render UI.

use hecs::World;
use macroquad::prelude::*;

use crate::{
    basic::{render::AssetManager, Position},
    game::state::GameState,
    world_mouse_pos,
};

/// Represents the text that should be rendered at an entity.
#[derive(Clone, Debug)]
pub struct Title {
    /// Text to render.
    pub text: String,
    /// Font to render the text with.
    /// Represents asset location in AssetManager.
    pub font: &'static str,
    /// Size of the text.
    pub size: f32,
    /// Color of the text.
    pub color: Color,
}

/// Detects mouse interactions (hovering and activation) and changes color
/// of Titles depending on its state.
#[derive(Clone, Copy, Debug)]
pub struct Button {
    /// Width of the interaction area.
    pub width: f32,
    /// Height of the interaction area.
    pub height: f32,
    /// Color the button's title should have when not active nor hovered.
    pub neutral_color: Color,
    /// Color the button's title should have when hovered.
    pub hover_color: Color,
    /// Color the button's title should have when activated.
    pub active_color: Color,
    /// Is true when the button is activated (or clicked).
    pub clicked: bool,
}

/// Marker of the button which starts the game.
#[derive(Clone, Copy, Debug)]
pub struct StartButton;
//-----------------------------------------------------------------------------
//SYSTEM PART
//-----------------------------------------------------------------------------

/// Handles rendering the texts of Titles.
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
                font_size: title.size as u16 * 2,
                font_scale: 0.5,
                color: title.color,
                ..Default::default()
            },
        )
    }
}

/// Handles changing Title's color depending on the button's state.
/// Also sets Button's 'clicked' variable according to its state.
pub fn button_colors(world: &mut World) {
    for (_, (position, button, title)) in world.query_mut::<(&Position, &mut Button, &mut Title)>()
    {
        //check for overlap
        let mouse_pos = world_mouse_pos();
        let hover = mouse_pos.x <= position.x + button.width / 2.0
            && mouse_pos.x >= position.x - button.width / 2.0
            && mouse_pos.y <= position.y + button.height / 2.0
            && mouse_pos.y >= position.y - button.height / 2.0;
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

/// Handle special buttons.
/// Currently handles StartButton changing game state to Running.
pub fn handle_buttons(world: &mut World) -> Option<GameState> {
    for (_, button) in world.query_mut::<&Button>().with::<&StartButton>() {
        if button.clicked {
            return Some(GameState::Running);
        }
    }
    None
}
