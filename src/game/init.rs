use hecs::World;
use macroquad::prelude::*;

use crate::{
    basic::{HealthDisplay, Position},
    menu::Title,
    player,
};

use super::EnemySpawner;

pub fn init_game(world: &mut World) {
    //clear remains of the previous state
    world.clear();
    //add entities required to play the game
    //add player
    let player_id = world.spawn(player::new_entity());

    //add player health display
    world.spawn((
        Position {
            x: screen_width() / 2.0,
            y: screen_height() - 3.0,
        },
        HealthDisplay {
            target: player_id,
            max_width: 250.0,
            height: 6.0,
            color: RED,
            max_color: Color {
                r: 0.4,
                g: 0.0,
                b: 0.0,
                a: 1.0,
            },
        },
    ));

    //add enemy spawner
    world.spawn((EnemySpawner::default(),));
}

pub fn init_main_menu(world: &mut World) {
    //clear remains of the previous state
    world.clear();

    //add game title
    world.spawn((
        Position {
            x: screen_width() / 2.0,
            y: 100.0,
        },
        Title {
            text: "THE GAME".into(),
            font: "main_font",
            size: 50.0,
            color: WHITE,
        },
    ));
}
