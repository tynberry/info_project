pub mod basic;
pub mod enemy;
pub mod game;
pub mod menu;
mod player;
pub mod projectile;

use basic::{fx::FxManager, render::AssetManager};
use enemy::{
    ASTEROID_TEX_NEGATIVE, ASTEROID_TEX_NEUTRAL, ASTEROID_TEX_POSITIVE, BIG_ASTEROID_TEX_NEGATIVE,
    BIG_ASTEROID_TEX_POSITIVE,
};
use game::state::GameState;
use macroquad::prelude::*;
use player::{PLAYER_TEX_NEGATIVE, PLAYER_TEX_POSITIVE};
use projectile::{PROJ_SMALL_TEX_NEG, PROJ_SMALL_TEX_POS};

const TEXTURES: [(&str, &str); 9] = [
    (ASTEROID_TEX_NEUTRAL, "res/asteroid.png"),
    (ASTEROID_TEX_POSITIVE, "res/asteroid_plus.png"),
    (ASTEROID_TEX_NEGATIVE, "res/asteroid_minus.png"),
    (BIG_ASTEROID_TEX_POSITIVE, "res/asteroid_big_plus.png"),
    (BIG_ASTEROID_TEX_NEGATIVE, "res/asteroid_big_minus.png"),
    (PLAYER_TEX_POSITIVE, "res/player_plus.png"),
    (PLAYER_TEX_NEGATIVE, "res/player_minus.png"),
    (PROJ_SMALL_TEX_NEG, "res/smal_proj_minus.png"),
    (PROJ_SMALL_TEX_POS, "res/smal_proj_plus.png"),
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
        clear_background(BLACK);

        //UPDATE VISUALS

        fx.update_particles(dt);

        state.render(&mut world, &mut events, &assets, dt, &mut fx);

        next_frame().await;
    }
}
