pub mod basic;
pub mod enemy;
pub mod game;
mod player;
pub mod projectile;

use basic::{health::HealthDisplay, render::AssetManager, Position};
use enemy::{
    ASTEROID_TEX_NEGATIVE, ASTEROID_TEX_NEUTRAL, ASTEROID_TEX_POSITIVE, BIG_ASTEROID_TEX_NEGATIVE,
    BIG_ASTEROID_TEX_POSITIVE,
};
use game::EnemySpawner;
use hecs::CommandBuffer;
use macroquad::prelude::*;
use player::{PLAYER_TEX_NEGATIVE, PLAYER_TEX_POSITIVE};
use projectile::{PROJ_SMALL_TEX_NEG, PROJ_SMALL_TEX_POS};

const ASSETS: [(&str, &str); 9] = [
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
    for (asset_id, asset_path) in ASSETS {
        assets.load_texture(asset_id, asset_path).await.unwrap();
    }

    //init world
    let mut world = hecs::World::default();
    //init events
    let mut events = hecs::World::default();

    //init cmd
    let mut cmd = CommandBuffer::new();

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

    loop {
        let dt = get_frame_time();
        //UPDATE WORLD
        //PLAYER
        player::weapons(&mut world, &mut cmd, dt);
        player::motion_update(&mut world, dt);

        //ENEMY AI
        enemy::shooter_ai(&mut world, &mut cmd, dt);

        //GLOBAL SYSTEMS
        basic::motion::apply_physics(&mut world, dt);
        basic::motion::apply_motion(&mut world, dt);

        basic::ensure_wrapping(&mut world, &mut cmd, &assets);
        basic::ensure_damage(&mut world, &mut events);
        basic::motion::apply_knockback(&mut world, &mut events);

        //AFTER EFFECTS
        player::health(&mut world, &mut events, dt);
        enemy::health(&mut world, &mut events, &mut cmd);
        projectile::on_hurt(&mut world, &mut events, &mut cmd);

        enemy::big_asteroid(&mut world, &mut cmd);

        //spawn enemies
        game::enemy_spawning(&mut world, &mut cmd, dt);

        //COMMAND BUFFER FLUSH
        cmd.run_on(&mut world);

        //CLEAR ALL EVENTS
        events.clear();

        //RENDERING PHASE
        clear_background(BLACK);

        //update visuals

        player::visuals(&mut world);

        //actually render

        basic::render::render_all(&mut world, &assets);

        basic::health::render_displays(&mut world);

        next_frame().await;
    }
}
