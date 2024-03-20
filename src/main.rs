pub mod basic;
mod player;
pub mod projectile;

use basic::{health::HealthDisplay, Position, Team};
use hecs::CommandBuffer;
use macroquad::prelude::*;

#[macroquad::main("Warping Warp")]
async fn main() {
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
        },
    ));

    //add projectile
    world.spawn(projectile::create_projectile(
        vec2(150.0, 150.0),
        vec2(0.0, 0.0),
        10.0,
        5.0,
        Team::Enemy,
    ));

    loop {
        let dt = get_frame_time();
        //UPDATE WORLD
        player::weapons(&mut world, &mut cmd, dt);
        player::motion_update(&mut world, dt);

        basic::motion::apply_motion(&mut world, dt);

        basic::ensure_wrapping(&mut world, &mut cmd);
        basic::ensure_damage(&mut world, &mut events);

        player::health(&mut world, &mut events, dt);
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
