pub mod basic;
mod player;
pub mod projectile;

use basic::{Position, Rotation, Wrapped};
use hecs::CommandBuffer;
use macroquad::prelude::*;
use player::Player;
use projectile::Projectile;

#[macroquad::main("Warping Warp")]
async fn main() {
    //init world
    let mut world = hecs::World::default();

    //init cmd
    let mut cmd = CommandBuffer::new();

    //add player
    world.spawn((
        Player::new(),
        Position { x: 100.0, y: 100.0 },
        Rotation::default(),
        Wrapped,
    ));

    //add projectile
    world.spawn((Projectile::new(10.0), Position { x: 25.0, y: 50.0 }));

    loop {
        let dt = get_frame_time();
        //UPDATE WORLD
        player::weapons(&mut world, &mut cmd, dt);
        player::motion_update(&mut world, dt);

        projectile::motion(&mut world, dt);

        basic::ensure_wrapping(&mut world, &mut cmd);

        //COMMAND BUFFER FLUSH
        cmd.run_on(&mut world);

        //RENDERING PHASE
        clear_background(BLACK);

        projectile::render(&mut world);

        player::render(&mut world);

        next_frame().await;
    }
}
