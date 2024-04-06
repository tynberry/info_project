use hecs::{CommandBuffer, World};
use macroquad::prelude::*;

use crate::{
    basic::{HealthDisplay, Position},
    menu::{Button, StartButton, Title},
    player, score, SPACE_HEIGHT, SPACE_WIDTH,
};

use super::{
    state::{GameOverTimer, Pause},
    EnemySpawner,
};

pub fn init_game(world: &mut World) {
    //clear remains of the previous state
    world.clear();
    //add entities required to play the game
    //add player
    let player_id = world.spawn(player::new_entity());

    //add player health display
    world.spawn((
        Position {
            x: SPACE_WIDTH / 2.0,
            y: SPACE_HEIGHT - 6.0,
        },
        HealthDisplay {
            target: player_id,
            max_width: 300.0,
            height: 8.0,
            color: RED,
            max_color: Color {
                r: 0.4,
                g: 0.0,
                b: 0.0,
                a: 1.0,
            },
        },
    ));

    //add player's score display
    world.spawn(score::create_score_display(vec2(SPACE_WIDTH / 2.0, 20.0), player_id).build());

    //add enemy spawner
    world.spawn((EnemySpawner::default(),));
}

pub fn init_main_menu(world: &mut World) {
    //clear remains of the previous state
    world.clear();

    //add game title
    world.spawn((
        Position {
            x: SPACE_WIDTH / 2.0,
            y: 120.0,
        },
        Title {
            text: "MAGNET FURY".into(),
            font: "main_font",
            size: 100.0,
            color: WHITE,
        },
    ));

    //add start game button
    world.spawn((
        Position {
            x: SPACE_WIDTH / 2.0,
            y: 280.0,
        },
        Title {
            text: "START".into(),
            font: "main_font",
            size: 50.0,
            color: WHITE,
        },
        Button {
            width: 160.0,
            height: 50.0,
            neutral_color: WHITE,
            hover_color: LIGHTGRAY,
            active_color: GRAY,
            clicked: false,
        },
        StartButton,
    ));
}

pub fn init_pause(world: &mut World) {
    world.spawn((
        Position {
            x: SPACE_WIDTH / 2.0,
            y: SPACE_HEIGHT / 2.0,
        },
        Title {
            text: "PAUSED".into(),
            font: "main_font",
            size: 40.0,
            color: WHITE,
        },
        Pause,
    ));
}

pub fn clear_pause(world: &mut World) {
    let mut cmd = CommandBuffer::new();
    for (entity, _) in world.query_mut::<&Pause>() {
        cmd.despawn(entity)
    }
    cmd.run_on(world);
}

pub fn init_game_over(world: &mut World) {
    world.spawn((GameOverTimer { time: 0.0 },));

    world.spawn((
        Position {
            x: SPACE_WIDTH / 2.0,
            y: SPACE_HEIGHT / 2.0,
        },
        Title {
            text: "GAME OVER".into(),
            font: "main_font",
            size: 60.0,
            color: WHITE,
        },
    ));

    world.spawn((
        Position {
            x: SPACE_WIDTH / 2.0,
            y: SPACE_HEIGHT / 2.0 + 60.0,
        },
        Title {
            text: "Press escape to return to main menu".into(),
            font: "main_font",
            size: 40.0,
            color: WHITE,
        },
    ));

    //add highscore
    world.spawn(score::create_highscore_display(vec2(SPACE_WIDTH / 2.0, 45.0)).build());
}
