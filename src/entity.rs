use sdl2::rect::Rect;

use crate::animation::AnimatedTexture;

pub type EntityList<'a> = Vec<Entity<'a>>;

#[derive(Clone, Copy)]
pub struct Entity<'a> {
    pub animated_texture: AnimatedTexture<'a>,
    pub position: Rect,
    pub velocity_y: i32,
}

impl Entity<'_> {
    pub fn new<'a>(texture: AnimatedTexture<'a>, position: Rect) -> Entity {
        Entity {
            animated_texture: texture,
            position,
            velocity_y: 0,
        }
    }

    pub fn set_velocity_y(&mut self, vel: i32) {
        self.velocity_y = vel;
    }
}
