extern crate sdl2;
extern crate time;
extern crate rand;

use std::path::Path;
use std::time::{Duration, Instant};
use rand::{thread_rng, Rng};

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

    // Initialize an RNG
    let mut rng = thread_rng();

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
    let mut buffer_tex: Texture = renderer.create_texture(
        PixelFormatEnum::RGBA8888,
        TextureAccess::Target,
        WIDTH,
        HEIGHT
    ).unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();

    // Get the time
    let mut now = Instant::now();
    let mut blit_now = true;

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }
        // Every two seconds fill the entire screen with random sprites
        // of every color!  
        
        if blit_now == true {

            // Set the SDL render target to be our double buffer
            renderer.render_target().unwrap().set(buffer_tex);
            renderer.clear();

            // For each sprite 'block' in the width and height of the screen...
            for i in 0..(WIDTH / SPRITE_W) + 1 {
                for j in 0..(HEIGHT / SPRITE_H) + 1 {
                    // Pick a random letter
                    let glyph_x = rng.gen_range(0, 16);
                    let glyph_y = rng.gen_range(3, 16);

                    // Get a random color for each hue
                    let r = rng.gen_range(0, 255);
                    let g = rng.gen_range(0, 255);
                    let b = rng.gen_range(0, 255);
                    let rand_rgb = Color::RGB(r, g, b);
                    

                    // Blit the random glyph from the spritesheet to the double buffer
                    let dest_rect = Rect::new(
                        (i * SPRITE_W) as i32, 
                        (j * SPRITE_H) as i32, 
                        SPRITE_W, 
                        SPRITE_H);
                    let src_rect = Rect::new(
                        (glyph_x * SPRITE_W) as i32, 
                        (glyph_y * SPRITE_H) as i32,
                        SPRITE_W,
                        SPRITE_H);
                    renderer.set_draw_color(rand_rgb);
                    //renderer.set_draw_color(Color::RGB(128, 128, 128));
                    renderer.fill_rect(dest_rect).unwrap();
                    //spritesheet.set_color_mod(255, 0, 0);
                    spritesheet.set_blend_mode(BlendMode::Mod);
                    renderer.copy(&spritesheet, Option::Some(src_rect), Option::Some(dest_rect)).unwrap();

                }
            }

            // Flip the double buffer to the screen
            buffer_tex = renderer.render_target().unwrap().reset().unwrap().unwrap();
            renderer.copy(&buffer_tex, None, None).expect("Render failed");
            renderer.present();
            // done blitting
            blit_now = false;
        }
        if now.elapsed() > Duration::from_secs(2) {
            blit_now = true;
            now = Instant::now();
        } 
    }
}
