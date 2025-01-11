mod physics;
mod types;

use physics::Physics;
use types::{Entity, EntityList};

use std::time::Duration;

use sdl2::{
    event::Event,
    image::{self, InitFlag, LoadTexture},
    keyboard::Keycode,
    pixels::Color,
    rect::Rect,
};

extern crate sdl2;

static W_WIDTH: u32 = 1280;
static W_HEIGHT: u32 = 720;
static JUMP_VELOCITY: i32 = -30;
static GRAVITY: i32 = 2;
static SPEED: i32 = 7;

fn main() -> Result<(), String> {
    let ctx = sdl2::init()?;
    let video_subsystem = ctx.video()?;

    let _image_context = image::init(InitFlag::PNG | InitFlag::JPG)?;

    let window = video_subsystem
        .window("runner game", W_WIDTH, W_HEIGHT)
        .position_centered()
        .build()
        .expect("Could not create window");

    let mut canvas = window
        .into_canvas()
        .build()
        .expect("Could not create canvas");

    let mut event_pump = ctx.event_pump()?;

    let mut entity_list = EntityList::new();

    let mut physics = Physics::new(SPEED, GRAVITY);

    let texture_creator = canvas.texture_creator();

    let player_texture = texture_creator.load_texture("assets/player.png")?;
    let enemy_texture = texture_creator.load_texture("assets/enemy.png")?;

    let player = Entity::new(
        &player_texture,
        Rect::new(20, 22, 18, 18),
        Rect::new(230, 460, 64, 64),
    );

    let enemy = Entity::new(
        &enemy_texture,
        Rect::new(0, 0, 64, 64),
        Rect::new(600, 460, 64, 64),
    );

    entity_list.push(player);
    entity_list.push(enemy);

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(Keycode::Space),
                    ..
                } => {
                    if entity_list[0].position.y == 460 {
                        entity_list[0].set_velocity_y(JUMP_VELOCITY)
                    }
                }
                Event::KeyUp {
                    keycode: Some(Keycode::Space),
                    ..
                } => {
                    if entity_list[0].velocity_y < 0 {
                        entity_list[0].set_velocity_y(0)
                    }
                }
                _ => {}
            }
        }

        // Update
        entity_list = physics.run(entity_list);

        // Close on collision
        if physics.get_collision_status() {
            break 'running;
        }

        // Render
        canvas.set_draw_color(Color::RGB(100, 100, 100));
        canvas.clear();

        // Ground
        canvas.set_draw_color(Color::RGB(100, 200, 100));
        canvas.fill_rect(Rect::new(0, (W_HEIGHT as i32) - 196, W_WIDTH, 196))?;

        for e in &entity_list {
            canvas.copy(e.texture, e.sprite, e.position)?;
        }

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}
