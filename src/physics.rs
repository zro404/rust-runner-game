use std::cmp;

use crate::entity::EntityList;

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

    pub fn run<'a>(&mut self, mut entity_list: EntityList<'a>) -> EntityList<'a> {
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

        // Check Collision - Axis-Aligned Bounding Box
        let player_pos = entity_list.get(0).unwrap().position;
        for i in 1..entity_list.len() {
            let enemy_pos = entity_list.get(i).unwrap().position;
            if player_pos.x < enemy_pos.x + enemy_pos.w
                && player_pos.x + player_pos.w > enemy_pos.x
                && player_pos.y < enemy_pos.y + enemy_pos.h
                && player_pos.y + player_pos.h > enemy_pos.y
            {
                self.did_collide = true;
            }
        }

        // Destroy if entity leaves viewport
        let mut c = entity_list.len();
        for i in 1..entity_list.len() {
            if c == entity_list.len() - 1 {
                break;
            };
            let entity = entity_list.get(i).unwrap();
            if entity.position.x + entity.position.w < 0 {
                if i < entity_list.len() - 1 {
                    entity_list[i] = entity_list.pop().unwrap();
                } else {
                    entity_list.pop();
                }
                c -= 1;
            }
        }

        entity_list
    }
}
