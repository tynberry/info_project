use hecs::{Entity, World};
use macroquad::{color::Color, shapes::draw_rectangle};

use crate::basic::Position;

use super::Health;

#[derive(Clone, Copy, Debug)]
pub struct HealthDisplay {
    pub target: Entity,
    pub max_width: f32,
    pub height: f32,
    pub color: Color,
}

//-----------------------------------------------------------------------------
//SYSTEM PART
//-----------------------------------------------------------------------------

pub fn render_health(world: &mut World) {
    //iterate over all displays
    for (_, (display, display_pos)) in world.query::<(&HealthDisplay, &Position)>().into_iter() {
        //get the entity in question
        let mut target = world.query_one::<&Health>(display.target).unwrap();
        let target_hp = target.get().unwrap();
        //render a rect for their health
        let current_width = (target_hp.hp / target_hp.max_hp) * display.max_width;

        draw_rectangle(
            display_pos.x - current_width / 2.0,
            display_pos.y - display.height / 2.0,
            current_width,
            display.height,
            display.color,
        );
    }
}
