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

/// Pixel Mod Ops

enum PixelMod {
    Set,
    Multiply,
    Add,
}


// General procedure
// 1. Create a black surface the size of the glyph
// 2. Tint the glyph what foreground color we would like
// 3. Blit the glyph onto the black surface with blendmode:add
// 4. Set black to be alpha
// 5. Create a surface with the bg color we want
// 6. Blit the colored glyph onto the background tile with blendmode:none

/// Return the individual sprite, with the color key
///
/// # Arguments
///
/// * spritesheet - the full spritesheet loaded as a Surface
/// * sprite_rect - a Rect defining the specific sprite's location on the spritesheet
/// * key - when specified, the color to be set as alpha, or None for default (black)
///
/// # Return Value
/// A Surface with the specific sprite
fn raw_sprite<'a>(spritesheet: &Surface,
                  sprite_rect: Rect,
                  key: Option<Color>)
                  -> Surface<'a> {
    let alpha_color = match key {
        Some(color) => color,
        None => Color::RGB(0, 0, 0),
    };
    let mut tile: Surface = create_tile(sprite_rect, alpha_color);
    spritesheet.blit(Some(sprite_rect), &mut tile, None).unwrap();
    tile.set_color_key(true, alpha_color);
    tile
}

/// Return a sprite with a colored foreground and background
///
/// # Arguments
///
/// * sprite   - The sprite to be colored, with the background transparent
///              (generally should be output of raw_sprite
/// * fg_color - if not None, the color to tint the sprite before blitting
///              onto the background
/// * bg_color - the background of the sprite - if None, the background won't
///              be set
///
/// # Return Value
///
/// A new Surface with the tinted sprite blitted onto the colored background
fn color_sprite<'a>(sprite: &Surface,
                    fg_color: Option<Color>,
                    bg_color: Option<Color>)
                    -> Surface<'a> {
    let bg: Color = match bg_color {
        None => Color::RGB(0, 0, 0),
        Some(c) => c,
    };
    let mut tile: Surface = create_tile(Rect::new(0, 0, sprite.width(), sprite.height()), bg);
    let mut glyph: Surface = create_tile(Rect::new(0, 0, sprite.width(), sprite.height()),
                                         Color::RGB(0, 0, 0));
    glyph.set_blend_mode(BlendMode::Blend);
    sprite.blit(None, &mut glyph, None).unwrap();
    if fg_color.is_some() {
        glyph.set_color_mod(fg_color.unwrap());
    }
    glyph.blit(None, &mut tile, None).unwrap();
    tile
}

fn create_tile<'a>(size_rect: Rect, color: Color)
                           -> Surface<'a> {
    let mut tile: Surface = Surface::new(size_rect.width(),
                                         size_rect.height(),
                                         PixelFormatEnum::RGB888
    ).unwrap();
    tile.fill_rect(None, color);
    tile
}
    
pub fn main() {

    let RED = Color::RGB(255, 0, 0);
    let BLUE = Color::RGB(0 , 0, 255);
    let WHITE = Color::RGB(255, 255, 255);
    let BLACK = Color::RGB(0, 0, 0);
    let GREEN = Color::RGB(0, 255, 0);
    let PINK = Color::RGB(255, 105, 180);
    
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
    let mut spritesheet: Texture = renderer.create_texture_from_surface(&spritesurf).unwrap();
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
        PixelFormatEnum::RGB888,
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
            renderer.set_draw_color(BLUE);
            renderer.clear();

            // For each sprite 'block' in the width and height of the screen...
            for i in 0..(WIDTH / SPRITE_W) + 1 {
                for j in 0..(HEIGHT / SPRITE_H) + 1 {
                    // Pick a random letter
                    let glyph_x = rng.gen_range(0, 16);
                    let glyph_y = rng.gen_range(3, 16);

                    // Get a random color for each hue
                    let r1 = rng.gen_range(0, 255);
                    let g1 = rng.gen_range(0, 255);
                    let b1 = rng.gen_range(0, 255);
                    let rand_rgb_1 = Color::RGB(r1, g1, b1);

                    // Get a random background for each hue
                    let r2 = rng.gen_range(0, 255);
                    let g2 = rng.gen_range(0, 255);
                    let b2 = rng.gen_range(0, 255);
                    let rand_rgb_2 = Color::RGB(r2, g2, b2);
                    

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

        
                    let raw_glyph: Surface = raw_sprite(&spritesurf,
                                                  src_rect,
                                                    Some(BLACK));
                    let glyph: Surface = color_sprite(&raw_glyph, Some(PINK), Some(GREEN));
                    //let glyph: Surface = color_sprite(&raw_glyph, Some(rand_rgb_1), Some(rand_rgb_2));
                    let mut glyph_tex: Texture = renderer.create_texture_from_surface(&glyph).unwrap();
                    renderer.copy(&glyph_tex, None, Some(dest_rect)).unwrap();
                    

                }
            }

            // Flip the double buffer to the screen
            buffer_tex = renderer.render_target().unwrap().reset().unwrap().unwrap();
            renderer.clear();
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
