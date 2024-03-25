use hecs::{Entity, World};
use macroquad::{color::Color, shapes::draw_rectangle};

use crate::basic::Position;

use super::Team;

//-----------------------------------------------------------------------------
//EVENT PART
//-----------------------------------------------------------------------------

#[derive(Clone, Copy, Debug)]
pub struct HitEvent {
    pub who: Entity,
    pub by: Entity,
    pub can_hurt: bool,
}

//-----------------------------------------------------------------------------
//COMPONENT PART
//-----------------------------------------------------------------------------

#[derive(Clone, Copy, Debug, Default)]
pub struct Health {
    pub max_hp: f32,
    pub hp: f32,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct DamageDealer {
    pub dmg: f32,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct HurtBox {
    pub radius: f32,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct HitBox {
    pub radius: f32,
}

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

pub fn render_displays(world: &mut World) {
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

pub fn ensure_damage(world: &mut World, events: &mut World) {
    //iterate through all hitable
    for (hit_id, (hit_pos, hit_box, hit_team)) in
        world.query::<(&Position, &HitBox, &Team)>().into_iter()
    {
        //iterate through all hurtting
        for (hurt_id, (hurt_pos, hurt_box, hurt_team)) in
            world.query::<(&Position, &HurtBox, &Team)>().into_iter()
        {
            //are they touching?
            let dx = hit_pos.x - hurt_pos.x;
            let dy = hit_pos.y - hurt_pos.y;
            if dx * dx + dy * dy < (hurt_box.radius + hit_box.radius).powi(2) {
                //add hit event
                events.spawn((HitEvent {
                    who: hit_id,
                    by: hurt_id,
                    can_hurt: hurt_team.can_hurt(hit_team)
                },));
            }
        }
    }
}
