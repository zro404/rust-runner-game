use sdl2::{rect::Rect, render::Texture};

#[derive(Clone, Copy)]
pub struct Entity<'a> {
    pub texture: &'a Texture<'a>,
    pub sprite: Rect,
    pub position: Rect,
    pub velocity_y: i32,
}

impl Entity<'_> {
    pub fn new<'a>(texture: &'a Texture<'a>, sprite: Rect, position: Rect) -> Entity {
        Entity {
            texture,
            sprite,
            position,
            velocity_y: 0,
        }
    }

    pub fn set_velocity_y(&mut self, vel: i32) {
        self.velocity_y = vel;
    }
}
