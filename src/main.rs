pub mod basic;
pub mod enemy;
pub mod game;
pub mod menu;
mod player;
pub mod projectile;

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
use player::{PLAYER_TEX_NEGATIVE, PLAYER_TEX_POSITIVE};
use projectile::{
    PROJ_MED_TEX_NEG, PROJ_MED_TEX_NEUTRAL, PROJ_MED_TEX_POS, PROJ_SMALL_TEX_NEG,
    PROJ_SMALL_TEX_POS,
};

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

#[macroquad::main("Warping Warp")]
async fn main() {
    //load assets to render
    let mut assets = AssetManager::default();
    for (asset_id, asset_path) in TEXTURES {
        assets.load_texture(asset_id, asset_path).await.unwrap();
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

        state.update(&mut world, &mut events, &assets, dt, &mut fx);

        //CLEAR ALL EVENTS
        events.clear();

        //RENDERING PHASE
        clear_background(Color::new(0.0, 0.05, 0.1, 1.0));

        //UPDATE VISUALS

        fx.update_particles(dt);

        state.render(&mut world, &mut events, &assets, dt, &mut fx);

        next_frame().await;
    }
}
