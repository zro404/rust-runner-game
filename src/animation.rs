use sdl2::{rect::Rect, render::Texture};

use crate::entity::EntityList;

#[derive(Clone, Copy)]
pub struct AnimatedTexture<'a> {
    pub texture: &'a Texture<'a>,
    pub sprite: Rect,
    pub speed: u32,
    frame_count: u32,
}

impl AnimatedTexture<'_> {
    pub fn new<'a>(texture: &'a Texture<'a>, sprite: Rect, speed: u32) -> AnimatedTexture {
        AnimatedTexture {
            texture,
            sprite,
            speed,
            frame_count: 0,
        }
    }
    pub fn tick(&mut self) {
        self.frame_count += 1;

        if self.frame_count == self.speed {
            self.frame_count = 0;
        }
    }
}

pub struct AnimationPlayer {}

impl AnimationPlayer {
    pub fn new() -> AnimationPlayer {
        AnimationPlayer {}
    }

    pub fn tick<'a>(&self, mut entity_list: EntityList<'a>) -> EntityList<'a> {
        for i in 1..entity_list.len() {
            entity_list[i].animated_texture.tick();
        }

        entity_list
    }
}
