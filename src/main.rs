extern crate sdl2;

use std::path::Path;

use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::surface::{Surface};
use sdl2::rect::Rect;
use sdl2::render::{BlendMode, Texture, TextureAccess, TextureQuery};

use sdl2::image::{INIT_PNG, LoadSurface, LoadTexture};

// Screen dimensions
const WIDTH:  u32 = 800;
const HEIGHT: u32 = 600;

// Information about the spritesheet.  We're using the tiles
// from the game 'Brogue' which are simple but nicely rendered
// ASCII characters, organized from the UTF-8 tabl
// into a 16 x 16 grid
// 'Z' = position 90
// 0-31 not used, 128-160 blank, 128-139 custom glyphs
const SPRITE_H: u32 = 28;
const SPRITE_W: u32 = 18;
const SPRITE_COLS: u32 = 16;
const SPRITE_ROWS: u32 = 16;
const SPRITE_SHEET: &'static str = "resources/BrogueFont5.png";

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("rust-sdl2 Tiling Demo", 800, 600)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let _image = sdl2::image::init(INIT_PNG);

    let mut renderer = window.renderer().build().unwrap();

    // Load the spritesheet as a surface so we can set the color key
    let mut spritesurf: Surface = Surface::from_file(Path::new(SPRITE_SHEET)).unwrap();
    // Make the background color transparent - this way we can set the background
    // of the sprite when we blit it
    spritesurf.set_color_key(true, Color::RGB(0, 0, 0)).unwrap();
    // convert the surface to a texture
    let mut spritesheet: Texture = renderer.create_texture_from_surface(spritesurf).unwrap();
    // Set the blend mode on the texture for antialiasing
    spritesheet.set_blend_mode(BlendMode::Add);


    // Create a 'punch' rect for getting individual sprites from the spritesheet
    let sprite_clip = Rect::new(0, 0, SPRITE_W, SPRITE_H);

    // Get information about the spritesheet, compare it to our expectations
    let tex_query: TextureQuery = spritesheet.query();
    assert!(tex_query.width == SPRITE_W * SPRITE_COLS);
    assert!(tex_query.height == SPRITE_H * SPRITE_ROWS);

    // Double buffer it up
    let buffer_tex: Texture = renderer.create_texture(
        PixelFormatEnum::RGBA8888,
        TextureAccess::Target,
        WIDTH,
        HEIGHT
    ).unwrap();

    renderer.set_draw_color(Color::RGB(0, 255, 0));
    renderer.clear();
    // Copy the sprite(s) onto the window
     renderer.copy(&spritesheet, None, None).expect("Render failed");

    renderer.present();
    let mut event_pump = sdl_context.event_pump().unwrap();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }
    }
}
