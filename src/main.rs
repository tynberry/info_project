pub mod basic;
mod player;
pub mod projectile;

use basic::{
    health::HealthDisplay, DamageDealer, Health, HitBox, HurtBox, Position, Rotation, Team, Wrapped,
};
use hecs::CommandBuffer;
use macroquad::prelude::*;
use player::Player;
use projectile::Projectile;

#[macroquad::main("Warping Warp")]
async fn main() {
    //init world
    let mut world = hecs::World::default();
    //init events
    let mut events = hecs::World::default();

    //init cmd
    let mut cmd = CommandBuffer::new();

    //add player
    let player_id = world.spawn((
        Player::new(),
        Position {
            x: screen_width() / 2.0,
            y: screen_height() / 2.0,
        },
        Rotation::default(),
        Health {
            hp: 10.0,
            max_hp: 10.0,
        },
        HitBox { radius: 7.0 },
        Team::Player,
        Wrapped,
    ));

    //add player health display
    world.spawn((
        Position {
            x: screen_width() / 2.0,
            y: screen_height() - 3.0,
        },
        HealthDisplay {
            target: player_id,
            max_width: 100.0,
            height: 6.0,
            color: RED,
        },
    ));

    //add projectile
    world.spawn((
        Projectile::new(10.0),
        Position { x: 125.0, y: 150.0 },
        Team::Enemy,
        HurtBox { radius: 10.0 },
        DamageDealer { dmg: 5.0 },
    ));

    loop {
        let dt = get_frame_time();
        //UPDATE WORLD
        player::weapons(&mut world, &mut cmd, dt);
        player::motion_update(&mut world, dt);

        projectile::motion(&mut world, dt);

        basic::ensure_wrapping(&mut world, &mut cmd);
        basic::ensure_damage(&mut world, &mut events);

        player::health(&mut world, &mut events);
        projectile::on_hurt(&mut world, &mut events, &mut cmd);

        //COMMAND BUFFER FLUSH
        cmd.run_on(&mut world);

        //CLEAR ALL EVENTS
        events.clear();

        //RENDERING PHASE
        clear_background(BLACK);

        projectile::render(&mut world);

        player::render(&mut world);

        basic::health::render_displays(&mut world);

        next_frame().await;
    }
}
