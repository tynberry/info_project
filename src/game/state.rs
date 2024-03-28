use hecs::{CommandBuffer, World};

use crate::{
    basic::{self, fx::FxManager, render::AssetManager},
    enemy, player, projectile,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameState {
    MainMenu,
    Running,
    Paused,
}

impl GameState {
    pub fn update(
        &mut self,
        world: &mut World,
        events: &mut World,
        assets: &AssetManager,
        dt: f32,
        fx: &mut FxManager,
    ) {
        let new_state = match self {
            GameState::MainMenu => todo!(),
            GameState::Running => game_update(world, events, assets, dt, fx),
            GameState::Paused => todo!(),
        };
        if let Some(state) = new_state {
            *self = state;
        }
    }

    pub fn render(
        &self,
        world: &mut World,
        _events: &mut World,
        assets: &AssetManager,
        _dt: f32,
        fx: &mut FxManager,
    ) {
        match self {
            GameState::MainMenu => todo!(),
            GameState::Running => game_render(world, fx, assets),
            GameState::Paused => todo!(),
        }
    }
}

fn game_update(
    world: &mut World,
    events: &mut World,
    assets: &AssetManager,
    dt: f32,
    fx: &mut FxManager,
) -> Option<GameState> {
    //Command buffer
    let mut cmd = CommandBuffer::new();
    //PLAYER
    player::weapons(world, &mut cmd, dt);
    player::motion_update(world, dt);

    //ENEMY AI
    enemy::shooter_ai(world, &mut cmd, dt);

    //GLOBAL SYSTEMS
    basic::motion::apply_physics(world, dt);
    basic::motion::apply_motion(world, dt);

    basic::ensure_wrapping(world, &mut cmd, assets);
    basic::ensure_damage(world, events);
    basic::motion::apply_knockback(world, events);

    //AFTER EFFECTS
    player::health(world, events, dt);
    enemy::health(world, events, &mut cmd);
    projectile::on_hurt(world, events, &mut cmd);

    enemy::asteroid(world, fx);
    enemy::big_asteroid(world, &mut cmd, fx);

    //spawn enemies
    super::enemy_spawning(world, &mut cmd, dt);

    //Apply commands
    cmd.run_on(world);

    None
}

fn game_render(world: &mut World, fx: &mut FxManager, assets: &AssetManager) {
    player::visuals(world, fx);

    //actually render

    basic::render::render_all(world, assets);

    fx.render_particles();

    basic::health::render_displays(world);
}
