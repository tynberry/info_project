use hecs::{CommandBuffer, World};
use macroquad::prelude::*;

use crate::{
    basic::{self, fx::FxManager, render::AssetManager, Health},
    enemy,
    menu::{self, Title},
    player::{self, Player},
    projectile,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameState {
    MainMenu,
    Running,
    Paused,
    GameOver,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Pause;

#[derive(Clone, Copy, Debug, Default)]
pub struct GameOverTimer {
    pub(crate) time: f32,
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
            GameState::MainMenu => main_menu_update(world),
            GameState::Running => game_update(world, events, assets, dt, fx),
            GameState::Paused => pause_update(world),
            GameState::GameOver => game_over_update(world, dt),
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
            GameState::MainMenu => main_menu_render(world, assets),
            GameState::Running => game_render(world, fx, assets),
            GameState::Paused => pause_render(world, fx, assets),
            GameState::GameOver => game_over_render(world, fx, assets),
        }
    }
}

//-----------------------------------------------------------------------------
//MAIN MENU
//-----------------------------------------------------------------------------

fn main_menu_update(world: &mut World) -> Option<GameState> {
    let new_state = menu::handle_buttons(world);

    if matches!(new_state, Some(GameState::Running)) {
        super::init::init_game(world);
    }

    new_state
}

fn main_menu_render(world: &mut World, assets: &AssetManager) {
    menu::button_colors(world);
    menu::render_title(world, assets);
}

//-----------------------------------------------------------------------------
//GAME
//-----------------------------------------------------------------------------

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

    enemy::asteroid_death(world, fx);
    enemy::big_asteroid_death(world, &mut cmd, fx);

    //spawn enemies
    super::enemy_spawning(world, &mut cmd, dt);

    //Apply commands
    cmd.run_on(world);

    //pausing
    if is_key_pressed(KeyCode::Escape) {
        super::init::init_pause(world);
        return Some(GameState::Paused);
    }

    //check for game over
    let (_, player_hp) = world
        .query_mut::<&Health>()
        .with::<&Player>()
        .into_iter()
        .next()
        .unwrap();

    if player_hp.hp <= 0.0 {
        super::init::init_game_over(world);
        return Some(GameState::GameOver);
    }

    None
}

fn game_render(world: &mut World, fx: &mut FxManager, assets: &AssetManager) {
    player::visuals(world, fx);

    //actually render

    basic::render::render_all(world, assets);

    fx.render_particles();

    basic::health::render_displays(world);
}

//-----------------------------------------------------------------------------
//PAUSE
//-----------------------------------------------------------------------------

fn pause_update(world: &mut World) -> Option<GameState> {
    if is_key_pressed(KeyCode::Escape) {
        super::init::clear_pause(world);
        Some(GameState::Running)
    } else {
        None
    }
}

fn pause_render(world: &mut World, fx: &mut FxManager, assets: &AssetManager) {
    //first render the game
    game_render(world, fx, assets);
    //overlap with transparent black
    draw_rectangle(
        0.0,
        0.0,
        screen_width(),
        screen_height(),
        Color {
            r: 0.0,
            g: 0.0,
            b: 0.0,
            a: 0.3,
        },
    );
    //draw pause text
    menu::render_title(world, assets);
}

//-----------------------------------------------------------------------------
//GAME OVER
//-----------------------------------------------------------------------------

const FULL_FADE_TIME: f32 = 1.0;

fn game_over_update(world: &mut World, dt: f32) -> Option<GameState> {
    //move timer
    for (_, timer) in world.query_mut::<&mut GameOverTimer>() {
        timer.time += dt;
    }
    //escape to safety when in gameover
    if is_key_pressed(KeyCode::Escape) {
        super::init::init_main_menu(world);
        Some(GameState::MainMenu)
    } else {
        None
    }
}

fn game_over_render(world: &mut World, fx: &mut FxManager, assets: &AssetManager) {
    //get time
    let time = world
        .query_mut::<&GameOverTimer>()
        .into_iter()
        .next()
        .unwrap()
        .1
        .time;
    //first render the game
    game_render(world, fx, assets);
    //overlap with transparent black
    draw_rectangle(
        0.0,
        0.0,
        screen_width(),
        screen_height(),
        Color {
            r: 0.0,
            g: 0.0,
            b: 0.0,
            a: 0.5 * (time / FULL_FADE_TIME).min(1.0),
        },
    );
    //fade in the texts as well
    for (_, title) in world.query_mut::<&mut Title>() {
        title.color.a = (time / FULL_FADE_TIME).min(1.0);
    }
    //draw game over text
    menu::render_title(world, assets);
}
