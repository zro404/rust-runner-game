use sdl2::{rect::Rect, render::Texture};

pub struct Entity<'a> {
    pub texture: &'a Texture<'a>,
    pub sprite: Rect,
    pub position: Rect,
}

impl Entity<'_> {
    pub fn new<'a>(texture: &'a Texture<'a>, sprite: Rect, position: Rect) -> Entity {
        Entity {
            texture,
            sprite,
            position,
        }
    }
}
