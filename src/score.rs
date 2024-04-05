use hecs::{Entity, EntityBuilder, World};
use macroquad::{color::WHITE, math::Vec2};

use crate::{basic::Position, menu::Title, persist::Persistent, player::Player};

#[derive(Clone, Copy, Debug)]
pub struct ScoreDisplay {
    pub player: Entity,
}

#[derive(Clone, Copy, Debug)]
pub struct HighScoreDisplay;

//-----------------------------------------------------------------------------
//ENTITY CREATION
//-----------------------------------------------------------------------------

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

pub fn score_display(world: &mut World, persist: &Persistent) {
    for (_, (title, display)) in world.query::<(&mut Title, &ScoreDisplay)>().into_iter() {
        //read score
        let score = world.get::<&Player>(display.player).unwrap().xp;
        //write it
        title.text = format!("Score: {}", score * 10);
    }

    for (_, title) in world
        .query_mut::<&mut Title>()
        .with::<&HighScoreDisplay>()
        .into_iter()
    {
        //write it
        title.text = format!("High Score: {}", persist.high_score * 10);
    }
}
