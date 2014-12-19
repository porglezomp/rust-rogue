extern crate image;

use std::collections::HashMap;
use std::rc::{Rc, Weak};

use sdl2::rect::Rect;
use sdl2::render::{Renderer, Texture, TextureAccess, BlendMode};
use sdl2::pixels::{PixelFormatFlag, Color};

use self::image::GenericImage;

pub const CHAR_WIDTH : i32 = 8;
pub const CHAR_HEIGHT : i32 = 14;
pub const LINE_SPACING : i32 = 2;
pub const MARGIN : i32 = 4;

pub struct Fonts {
    charmap: HashMap<char, Rect>,
    texture: Texture,
    renderer: Weak<Renderer>,
    background: Color,
}

impl Fonts {
    pub fn new(renderer: Rc<Renderer>) -> Fonts {
        // The characters in the same order they appear in the font file
        // so that we can map a letter to an image offset
        let charlist = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz \
0123456789,.\"'?!@_*#$%&()+-/:;<=>[\\]^`{|}~";
        // Produce all the image offsets for each letter and store them in
        // the font set
        let mut charmap = HashMap::new();
        let mut i: i32 = 0;
        for character in charlist.as_bytes().iter() {
            let newrect = Rect::new(i * CHAR_WIDTH, 0, CHAR_WIDTH, CHAR_HEIGHT);
            charmap.insert(*character as char, newrect);
            i += 1;
        }

        // Load the font into our texture
        let image = image::open(&Path::new("../font-large-alpha.png")).unwrap();
        let (tex_width, tex_height) = image.dimensions();
        let texture = renderer.create_texture(PixelFormatFlag::RGBA8888,
            TextureAccess::Static, tex_width as int, tex_height as int).unwrap();
        let _ = texture.update(None, image.raw_pixels().as_slice(), tex_width as int * 4);
        texture.set_blend_mode(BlendMode::Blend);
        
        Fonts {
            charmap: charmap,
            texture: texture,
            renderer: renderer.downgrade(),
            background: Color::RGB(0, 0, 0)
        }
    }

    /// Get the position in our font texture that corresponds to the character
    pub fn get_char_rect(&self, character: char) -> Rect {
        if !self.charmap.contains_key(&character) {
            panic!("No '{}' character in font!", character);
        }
        self.charmap[character]
    }

    /// Draw a character onto the screen at the position
    /// The position is column, row starting with 0, 0 at the upper left
    pub fn place_character_at_point(&self, character: char, x: i32, y: i32) {
        // Define blitting rectangles
        let source = self.get_char_rect(character);
        let dest = Rect::new(MARGIN + x * CHAR_WIDTH, MARGIN + y * (CHAR_HEIGHT + LINE_SPACING),
                             CHAR_WIDTH, CHAR_HEIGHT);
        let bg_rect = Rect::new(MARGIN + x * CHAR_WIDTH,
            MARGIN + y * (CHAR_HEIGHT + LINE_SPACING) - LINE_SPACING/2,
            CHAR_WIDTH, CHAR_HEIGHT + LINE_SPACING );

        let renderer = self.renderer.upgrade().unwrap();

        // Draw the background with the background color and then reset the draw color
        let old_color = renderer.get_draw_color().unwrap();
        renderer.set_draw_color(self.background);
        renderer.fill_rect(&bg_rect);
        renderer.set_draw_color(old_color);

        // Finally render our character on top of it
        let result = renderer.copy(&self.texture, Some(source), Some(dest));
        match result {
            Ok(())   => (),
            Err(err) => panic!("Error copying font texture from {} to {}: {}", source, dest, err)
        }
    }

    /// The foreground color is the color the letter itself is drawn with
    pub fn set_foreground_color(&self, r: u8, g: u8, b: u8) {
        self.texture.set_color_mod(r, g, b);
    }

    pub fn set_background_color(&mut self, r: u8, g: u8, b: u8) {
        self.background = Color::RGB(r, g, b);
    }
}