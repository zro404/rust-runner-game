use std::cmp;

use sdl2::rect::Rect;

use crate::types::Entity;

pub struct Physics {
    speed: i32,
    gravity: i32,
    did_collide: bool,
}

impl Physics {
    pub fn new(speed: i32, gravity: i32) -> Physics {
        Physics {
            speed,
            gravity,
            did_collide: false,
        }
    }

    pub fn get_collision_status(&self) -> bool {
        self.did_collide
    }

    pub fn reset_collision(&mut self) {
        self.did_collide = false;
    }

    pub fn run<'a>(&self, mut entity_list: Vec<Entity<'a>>) -> Vec<Entity<'a>> {
        // Handle player jump
        let vel_y = entity_list[0].velocity_y + self.gravity;
        entity_list[0].set_velocity_y(vel_y);
        entity_list[0].position.y = cmp::min(entity_list[0].position.y + vel_y, 460);

        // Move all entities except Player
        for i in 1..entity_list.len() {
            let mut e = entity_list[i];
            e.position.x -= self.speed;
            entity_list[i] = e;
        }

        // Check Collision

        entity_list
    }
}
