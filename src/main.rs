//! This module initialises the game, all of its assets and 
//! required systems to function.
//! It also handles the main loop, update and render of Gamestates.
//! 


pub mod basic;
pub mod enemy;
pub mod game;
pub mod menu;
pub mod persist;
mod player;
pub mod projectile;
pub mod score;
pub mod xp;

use basic::{fx::FxManager, render::AssetManager};
use enemy::{
    charged::ASTEROID_OUTLINE_TEX,
    follower::{FOLLOWER_TEX_NEGATIVE, FOLLOWER_TEX_NEUTRAL, FOLLOWER_TEX_POSITIVE},
    mine::{MINE_TEX_NEGATIVE, MINE_TEX_NEUTRAL, MINE_TEX_POSITIVE},
    ASTEROID_TEX_NEGATIVE, ASTEROID_TEX_NEUTRAL, ASTEROID_TEX_POSITIVE, BIG_ASTEROID_TEX_NEGATIVE,
    BIG_ASTEROID_TEX_POSITIVE,
};
use game::state::GameState;
use macroquad::prelude::*;
use persist::Persistent;
use player::{PLAYER_TEX_NEGATIVE, PLAYER_TEX_POSITIVE};
use projectile::{
    PROJ_MED_TEX_NEG, PROJ_MED_TEX_NEUTRAL, PROJ_MED_TEX_POS, PROJ_SMALL_TEX_NEG,
    PROJ_SMALL_TEX_POS,
};

/// Internal logical space width.
/// Values outside this range are not rendered.
pub const SPACE_WIDTH: f32 = 1280.0;
/// Internal logical space height.
/// Values outside this range are not rendered.
pub const SPACE_HEIGHT: f32 = 720.0;

/// Returns the position of the mouse in world coordinates.
pub fn world_mouse_pos() -> Vec2 {
    let (mx, my) = mouse_position();
    let camera = &Camera2D::from_display_rect(Rect {
        x: 0.0,
        y: SPACE_HEIGHT,
        w: SPACE_WIDTH,
        h: -SPACE_HEIGHT,
    });
    camera.screen_to_world(vec2(mx, my))
}

/// Texture assets id, location, lookup table.
const TEXTURES: [(&str, &str); 19] = [
    (ASTEROID_TEX_NEUTRAL, "res/asteroid.png"),
    (ASTEROID_TEX_POSITIVE, "res/asteroid_plus.png"),
    (ASTEROID_TEX_NEGATIVE, "res/asteroid_minus.png"),
    (ASTEROID_OUTLINE_TEX, "res/asteroid_outline.png"),
    (BIG_ASTEROID_TEX_POSITIVE, "res/asteroid_big_plus.png"),
    (BIG_ASTEROID_TEX_NEGATIVE, "res/asteroid_big_minus.png"),
    (PLAYER_TEX_POSITIVE, "res/player_plus.png"),
    (PLAYER_TEX_NEGATIVE, "res/player_minus.png"),
    (PROJ_SMALL_TEX_NEG, "res/smal_proj_minus.png"),
    (PROJ_SMALL_TEX_POS, "res/smal_proj_plus.png"),
    (PROJ_MED_TEX_NEUTRAL, "res/medium_proj_neutral.png"),
    (PROJ_MED_TEX_NEG, "res/medium_proj_minus.png"),
    (PROJ_MED_TEX_POS, "res/medium_proj_plus.png"),
    (FOLLOWER_TEX_NEUTRAL, "res/saw_blade.png"),
    (FOLLOWER_TEX_POSITIVE, "res/saw_blade_plus.png"),
    (FOLLOWER_TEX_NEGATIVE, "res/saw_blade_minus.png"),
    (MINE_TEX_NEUTRAL, "res/mine_neutral.png"),
    (MINE_TEX_POSITIVE, "res/mine_plus.png"),
    (MINE_TEX_NEGATIVE, "res/mine_minus.png"),
];

/// Sound assets id, location, lookup table.
const SOUNDS: [(&str, &str); 3] = [
    ("player_jet", "res/sound/movement.wav"),
    ("knockback", "res/sound/boing.wav"),
    ("pew_pew", "res/sound/pew_pew.wav"),
];

/// Returns requested properties of the window.
/// It sets the title and window size.
fn conf() -> Conf {
    Conf {
        window_title: "Magnet fury".to_owned(),
        window_width: SPACE_WIDTH as i32,
        window_height: SPACE_HEIGHT as i32,
        ..Default::default()
    }
}

/// Entry point of the application.
#[macroquad::main(conf)]
async fn main() {
    //load persitent as a resource
    let mut persist = Persistent::load().await.unwrap_or_default();

    //load assets to render
    let mut assets = AssetManager::default();
    for (asset_id, asset_path) in TEXTURES {
        assets.load_texture(asset_id, asset_path).await.unwrap();
    }
    for (asset_id, asset_path) in SOUNDS {
        assets.load_sound(asset_id, asset_path).await.unwrap();
    }

    //load font
    assets
        .load_font("main_font", "res/NotoSans-Regular.ttf")
        //.load_font("main_font", "res/ShantellSans-Medium.ttf")
        .await
        .unwrap();

    //init particle system
    let mut fx = FxManager::new(1024);

    //init world
    let mut world = hecs::World::default();
    //init events
    let mut events = hecs::World::default();
    //init game state
    let mut state = GameState::MainMenu;

    //init game
    game::init::init_main_menu(&mut world);

    loop {
        let dt = get_frame_time();
        //UPDATE WORLD

        state.update(&mut world, &mut events, &assets, dt, &mut fx, &mut persist);

        //CLEAR ALL EVENTS
        events.clear();

        //RENDERING PHASE
        clear_background(Color::new(0.0, 0.05, 0.1, 1.0));

        //UPDATE VISUALS
        set_camera(&Camera2D::from_display_rect(Rect {
            x: 0.0,
            y: SPACE_HEIGHT,
            w: SPACE_WIDTH,
            h: -SPACE_HEIGHT,
        }));

        fx.update_particles(dt);

        state.render(&mut world, &mut events, &assets, dt, &mut fx, &persist);

        next_frame().await;
    }
}
