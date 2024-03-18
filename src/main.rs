pub mod basic;
mod player;

use basic::{Position, Rotation, Wrapped};
use macroquad::prelude::*;
use player::Player;

#[macroquad::main("Warping Warp")]
async fn main() {
    //init world
    let mut world = hecs::World::default();

    //add player 
    world.spawn((Player::new(), Position{
        x: 100.0,
        y: 100.0,
    }, Rotation::default(), Wrapped));

    loop {
        let dt = get_frame_time();
        //UPDATE WORLD
        player::player_motion_update(&mut world, dt);

        basic::ensure_wrapping(&mut world);

        //RENDERING PHASE
        clear_background(BLACK);

        player::player_render(&mut world);

        next_frame().await;
    }
}
