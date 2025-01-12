mod animation;
mod entity;
mod physics;

use animation::{AnimatedTexture, AnimationPlayer};
use entity::{Entity, EntityList};
use physics::Physics;

use std::time::Duration;

use sdl2::{
    event::Event,
    image::{self, InitFlag, LoadTexture},
    keyboard::Keycode,
    pixels::Color,
    rect::Rect,
    ttf,
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

    let animation_player = AnimationPlayer::new();

    let font_ctx = ttf::init().unwrap();
    let font = font_ctx.load_font("assets/ARCADECLASSIC.TTF", 24)?;

    let texture_creator = canvas.texture_creator();

    let game_over_text = "Game Over!";
    // render a surface, and convert it to a texture bound to the canvas
    let surface = font
        .render(game_over_text)
        .blended(Color::RGBA(255, 255, 255, 255))
        .map_err(|e| e.to_string())?;
    let game_over_texture = texture_creator
        .create_texture_from_surface(&surface)
        .map_err(|e| e.to_string())?;

    let player_texture = texture_creator.load_texture("assets/player.png")?;
    let enemy_texture = texture_creator.load_texture("assets/enemy.png")?;

    let player_anim = AnimatedTexture::new(&player_texture, Rect::new(20, 22, 18, 17), 10);
    let enemy_anim = AnimatedTexture::new(&enemy_texture, Rect::new(0, 0, 64, 64), 10);

    let player = Entity::new(player_anim, Rect::new(230, 460, 64, 64));
    let enemy = Entity::new(enemy_anim, Rect::new(600, 460, 64, 64));

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
                    if physics.get_collision_status() {
                        physics.reset_collision();
                        // Delete all enemies on screen
                        for _ in 1..entity_list.len() {
                            entity_list.pop();
                        }
                    } else if entity_list[0].position.y == 460 {
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

        // Render
        canvas.set_draw_color(Color::RGB(100, 100, 100));
        canvas.clear();

        // Pause on collision
        if physics.get_collision_status() {
            canvas.copy(
                &game_over_texture,
                None,
                Rect::new(
                    ((W_WIDTH - 20 * game_over_text.len() as u32) / 2) as i32,
                    ((W_HEIGHT / 2) - 30) as i32,
                    20 * game_over_text.len() as u32,
                    64,
                ),
            )?;
        } else {
            // Update
            entity_list = physics.run(entity_list);

            entity_list = animation_player.tick(entity_list);

            // Ground
            canvas.set_draw_color(Color::RGB(100, 200, 100));
            canvas.fill_rect(Rect::new(0, (W_HEIGHT as i32) - 196, W_WIDTH, 196))?;

            for e in &entity_list {
                canvas.copy(
                    e.animated_texture.texture,
                    e.animated_texture.sprite,
                    e.position,
                )?;
            }
        }

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}
