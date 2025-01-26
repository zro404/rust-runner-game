use sdl2::{rect::Rect, render::Texture};

use crate::entity::EntityList;

#[derive(Clone)]
pub struct AnimatedTexture<'a> {
    pub texture: &'a Texture<'a>,
    pub sprite: Rect,
    pub speed: u32,
    pub current_anim: u8,
    current_frame: u8,
    anim_array: Vec<u8>,
    paused: bool,
    frame_count: u32,
}

impl AnimatedTexture<'_> {
    pub fn new<'a>(
        texture: &'a Texture<'a>,
        sprite: Rect,
        speed: u32,
        anim_array: Vec<u8>,
    ) -> AnimatedTexture {
        AnimatedTexture {
            texture,
            sprite,
            speed,
            anim_array,
            paused: false,
            current_anim: 0,
            current_frame: 0,
            frame_count: 0,
        }
    }
    pub fn tick(&mut self) {
        if !self.paused {
            self.frame_count += 1;

            if self.frame_count >= self.speed {
                self.frame_count = 0;
                self.current_frame += 1;
                if self.current_frame >= self.anim_array[self.current_anim as usize] {
                    self.current_frame = 0;
                }
            }
        }
        self.sprite.y = self.sprite.h * self.current_anim as i32;
        self.sprite.x = self.sprite.w * self.current_frame as i32;
    }
    pub fn pause(&mut self) {
        self.paused = true;
    }
    pub fn resume(&mut self) {
        self.paused = false;
    }
    pub fn play(&mut self, anim: u8) {
        if anim as usize <= self.anim_array.len() {
            self.current_anim = anim;
            self.current_frame = 0;
        } else {
            panic!("Invalid Animation Index!");
        }
    }
}

pub struct AnimationPlayer {}

impl AnimationPlayer {
    pub fn new() -> AnimationPlayer {
        AnimationPlayer {}
    }

    pub fn tick<'a>(&self, mut entity_list: EntityList<'a>) -> EntityList<'a> {
        for i in 0..entity_list.len() {
            entity_list[i].animated_texture.tick();
        }

        entity_list
    }
}
