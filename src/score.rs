//! Score displays

use hecs::{Entity, EntityBuilder, World};
use macroquad::{color::WHITE, math::Vec2};

use crate::{basic::Position, menu::Title, persist::Persistent, player::Player};

/// Displays current score.
#[derive(Clone, Copy, Debug)]
pub struct ScoreDisplay {
    /// Entity ID to the player
    pub player: Entity,
}

/// Displays high score from Persistent (save file).
#[derive(Clone, Copy, Debug)]
pub struct HighScoreDisplay;

//-----------------------------------------------------------------------------
//ENTITY CREATION
//-----------------------------------------------------------------------------

/// Creates score display entity
/// # Arguments
/// - `pos` - position of the score display
/// - `player` - entity ID of the player
pub fn create_score_display(pos: Vec2, player: Entity) -> EntityBuilder {
    let mut builder = EntityBuilder::new();

    builder.add(Position { x: pos.x, y: pos.y });

    builder.add(Title {
        text: "Score: 0".to_string(),
        font: "main_font",
        size: 24.0,
        color: WHITE,
    });

    builder.add(ScoreDisplay { player });

    builder
}

/// Creates high score display entity
/// ## Params
/// - `pos` - position of the score display
pub fn create_highscore_display(pos: Vec2) -> EntityBuilder {
    let mut builder = EntityBuilder::new();

    builder.add(Position { x: pos.x, y: pos.y });

    builder.add(Title {
        text: "Score: 0".to_string(),
        font: "main_font",
        size: 24.0,
        color: WHITE,
    });

    builder.add(HighScoreDisplay);

    builder
}

//-----------------------------------------------------------------------------
//SYSTEM PART
//-----------------------------------------------------------------------------

/// Synchronizes the titles and current score/highscores.
pub fn score_display(world: &mut World, persist: &Persistent) {
    //synchronize score displays
    for (_, (title, display)) in world.query::<(&mut Title, &ScoreDisplay)>().into_iter() {
        //read score
        let score = world.get::<&Player>(display.player).unwrap().xp;
        //write it
        title.text = format!("Score: {}", score * 10);
    }

    //synchronize highscore displays
    for (_, title) in world
        .query_mut::<&mut Title>()
        .with::<&HighScoreDisplay>()
        .into_iter()
    {
        //write it
        title.text = format!("High Score: {}", persist.high_score * 10);
    }
}
