pub mod basic;
pub mod enemy;
mod player;
pub mod projectile;

use basic::{health::HealthDisplay, Position};
use hecs::CommandBuffer;
use macroquad::prelude::*;

#[macroquad::main("Warping Warp")]
async fn main() {
    //load assets to render
    let player_texture = load_texture("res/player.png").await.unwrap();
    let asteroid_texture = load_texture("res/asteroid.png").await.unwrap();

    //init world
    let mut world = hecs::World::default();
    //init events
    let mut events = hecs::World::default();

    //init cmd
    let mut cmd = CommandBuffer::new();

    //add player
    let player_id = world.spawn(player::new_entity(&player_texture));

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
        },
    ));

    //add enemy
    world.spawn(enemy::create_asteroid(
        vec2(-10.0, 300.0),
        vec2(1.0, 0.0),
        &asteroid_texture,
    ));

    world.spawn(enemy::create_shooter(vec2(200.0, 150.0), vec2(0.0, 0.0)));

    loop {
        let dt = get_frame_time();
        //UPDATE WORLD
        player::weapons(&mut world, &mut cmd, dt);
        player::motion_update(&mut world, dt);

        enemy::shooter_ai(&mut world, &mut cmd, dt);

        basic::motion::apply_physics(&mut world, dt);
        basic::motion::apply_motion(&mut world, dt);

        basic::ensure_wrapping(&mut world, &mut cmd);
        basic::ensure_damage(&mut world, &mut events);

        player::health(&mut world, &mut events, dt);
        enemy::health(&mut world, &mut events, &mut cmd);
        projectile::on_hurt(&mut world, &mut events, &mut cmd);

        //COMMAND BUFFER FLUSH
        cmd.run_on(&mut world);

        //CLEAR ALL EVENTS
        events.clear();

        //RENDERING PHASE
        clear_background(BLACK);

        basic::render::render_all(&mut world);

        basic::health::render_displays(&mut world);

        next_frame().await;
    }
}
