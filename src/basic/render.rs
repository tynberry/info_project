use enum_dispatch::enum_dispatch;
use hecs::World;
use macroquad::prelude::*;

use super::{Position, Rotation};

#[derive(Clone, Copy, Debug)]
pub struct Rectangle {
    pub width: f32,
    pub height: f32,
    pub color: Color,
    pub z_index: i16,
}

impl Renderable for Rectangle {
    fn render(&self, pos: &Position, rotation: Option<&Rotation>) {
        match rotation {
            Some(Rotation { angle }) => {
                draw_rectangle_ex(
                    pos.x,
                    pos.y,
                    self.width,
                    self.height,
                    DrawRectangleParams {
                        offset: vec2(0.5, 0.5),
                        rotation: *angle,
                        color: self.color,
                    },
                );
            }
            None => draw_rectangle(
                pos.x - self.width / 2.0,
                pos.y - self.height / 2.0,
                self.width,
                self.height,
                self.color,
            ),
        }
    }

    fn z_index(&self) -> i16 {
        self.z_index
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Circle {
    pub radius: f32,
    pub color: Color,
    pub z_index: i16,
}

impl Renderable for Circle {
    fn render(&self, pos: &Position, _rotation: Option<&Rotation>) {
        draw_circle(pos.x, pos.y, self.radius, self.color);
    }

    fn z_index(&self) -> i16 {
        self.z_index
    }
}

//-----------------------------------------------------------------------------
//TRAIT PART
//-----------------------------------------------------------------------------

#[enum_dispatch]
trait Renderable {
    fn render(&self, pos: &Position, rotation: Option<&Rotation>);
    fn z_index(&self) -> i16 {
        0
    }
}

#[enum_dispatch(Renderable)]
enum RenderJobs {
    Rectangle,
    Circle,
}

//-----------------------------------------------------------------------------
//SYSTEM PART
//-----------------------------------------------------------------------------

pub fn render_all(world: &mut World) {
    //gather all render jobs
    //circles
    let mut jobs: Vec<(RenderJobs, Position, Option<Rotation>)> = world
        .query_mut::<(&Circle, &Position, Option<&Rotation>)>()
        .into_iter()
        .map(|(_, (c, p, r))| (Into::<RenderJobs>::into(*c), *p, r.copied()))
        .collect();
    //rectangles
    jobs.extend(
        world
            .query_mut::<(&Rectangle, &Position, Option<&Rotation>)>()
            .into_iter()
            .map(|(_, (c, p, r))| (Into::<RenderJobs>::into(*c), *p, r.copied())),
    );
    //sort them by z_index
    jobs.sort_unstable_by_key(|a| a.0.z_index());
    //render all of them
    for job in jobs {
        job.0.render(&job.1, job.2.as_ref());
    }
}
