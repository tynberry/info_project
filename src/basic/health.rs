//! Health, Damage and Collision handling systems and structs.
use hecs::{Entity, World};
use macroquad::{color::Color, shapes::draw_rectangle};

use crate::basic::Position;

use super::Team;

//-----------------------------------------------------------------------------
//EVENT PART
//-----------------------------------------------------------------------------

/// Event representing collision between two entities.
#[derive(Clone, Copy, Debug)]
pub struct HitEvent {
    /// Entity id of the entity that was hit.
    /// This entity has HitBox.
    pub who: Entity,
    /// Entity id of the entity that hit the `who` entity.
    /// This entity has HurtBox.
    pub by: Entity,
    /// Can the `by` entity deal damage to the `who` entity?
    pub can_hurt: bool,
}

//-----------------------------------------------------------------------------
//COMPONENT PART
//-----------------------------------------------------------------------------

/// Health of the entity. When `hp` <= 0.0, then the entity is dead.
#[derive(Clone, Copy, Debug, Default)]
pub struct Health {
    /// Max health the entity can have.
    /// Used to limit `heal` method.
    pub max_hp: f32,
    /// Amount of health the entity currently has.
    pub hp: f32,
}

impl Health {
    /// Increases `hp` by `amount` while no going over
    /// max health.
    pub fn heal(&mut self, amount: f32) {
        self.hp += amount;
        if self.hp > self.max_hp {
            self.hp = self.max_hp;
        }
    }
}

/// Denotes an entity that can deal damage to other ones.
#[derive(Clone, Copy, Debug, Default)]
pub struct DamageDealer {
    /// Amount of damage this entity does on hit.
    pub dmg: f32,
}

/// Circle around which the entity can hit entites with `HitBox`.
#[derive(Clone, Copy, Debug, Default)]
pub struct HurtBox {
    pub radius: f32,
}

/// Circle around which the entity can get hit by entites with `HurtBox`.
#[derive(Clone, Copy, Debug, Default)]
pub struct HitBox {
    pub radius: f32,
}

/// Component that shows a health bar that represents the entity's health
/// stored in `Health`.
#[derive(Clone, Copy, Debug)]
pub struct HealthDisplay {
    /// Entity whose `Health` is being shown.
    /// The entity must have `Health`.
    pub target: Entity,
    /// Width of the bar when health is at its maximum.
    pub max_width: f32,
    /// Height of the bar.
    pub height: f32,
    /// Color of foreground of the bar.
    /// Foreground shows the current amount of health.
    pub color: Color,
    /// Color of background of the bar.
    /// Background shows the max health the entity can have
    /// (According to its `Health` component).
    pub max_color: Color,
}

//-----------------------------------------------------------------------------
//SYSTEM PART
//-----------------------------------------------------------------------------

/// Renders `HealthDisplay`s
pub fn render_displays(world: &mut World) {
    //iterate over all displays
    for (_, (display, pos)) in world.query::<(&HealthDisplay, &Position)>().into_iter() {
        //get the entity in question
        let mut target = world.query_one::<&Health>(display.target).unwrap();
        let target_hp = target.get().unwrap();
        //render a rect for their health
        let current_width = ((target_hp.hp / target_hp.max_hp) * display.max_width).max(0.0);

        //draw background of max
        draw_rectangle(
            pos.x - display.max_width / 2.0,
            pos.y - display.height / 2.0,
            display.max_width,
            display.height,
            display.max_color,
        );
        //draw actual health
        draw_rectangle(
            pos.x - display.max_width / 2.0,
            pos.y - display.height / 2.0,
            current_width,
            display.height,
            display.color,
        );
    }
}

/// Handles collision detection between `HitBox`es and `HurtBox`es.
pub fn ensure_damage(world: &mut World, events: &mut World) {
    //iterate through all hitable
    for (hit_id, (hit_pos, hit_box, hit_team)) in
        world.query::<(&Position, &HitBox, &Team)>().into_iter()
    {
        //iterate through all hurtting
        for (hurt_id, (hurt_pos, hurt_box, hurt_team)) in
            world.query::<(&Position, &HurtBox, &Team)>().into_iter()
        {
            //ignore self collisions
            if hurt_id == hit_id {
                continue;
            }
            //are they touching?
            let dx = hit_pos.x - hurt_pos.x;
            let dy = hit_pos.y - hurt_pos.y;
            if dx * dx + dy * dy < (hurt_box.radius + hit_box.radius).powi(2) {
                //add hit event
                events.spawn((HitEvent {
                    who: hit_id,
                    by: hurt_id,
                    can_hurt: hurt_team.can_hurt(hit_team),
                },));
            }
        }
    }
}
