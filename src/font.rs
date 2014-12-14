extern crate image;

use std::collections::HashMap;
use std::rc::{Rc, Weak};

use sdl2::rect::Rect;
use sdl2::render::{Renderer, Texture, TextureAccess};
use sdl2::pixels::PixelFormatFlag;

use self::image::GenericImage;

pub const CHAR_WIDTH : i32 = 8;
pub const CHAR_HEIGHT : i32 = 14;
pub const LINE_SPACING : i32 = 2;
pub const MARGIN : i32 = 4;

pub struct Fonts {
    // characters: &'static str,
    charmap: HashMap<char, Rect>,
    texture: Texture,
    renderer: Weak<Renderer>

}

impl Fonts {
    pub fn new(renderer: Rc<Renderer>) -> Fonts {
        let charlist = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz \
0123456789,.\"'?!@_*#$%&()+-/:;<=>[\\]^`{|}~";
        
        let mut charmap = HashMap::new();
        let mut i: i32 = 0;
        for character in charlist.as_bytes().iter() {
            let newrect = Rect::new(i * CHAR_WIDTH, 0, CHAR_WIDTH, CHAR_HEIGHT);
            charmap.insert(*character as char, newrect);
            i += 1;
        }

        // Load the font into our texture
        let image = image::open(&Path::new("../font-large.png")).unwrap();
        let (tex_width, tex_height) = image.dimensions();
        let texture = renderer.create_texture(PixelFormatFlag::ARGB8888,
            TextureAccess::Static, tex_width as int, tex_height as int).unwrap();
        let _ = texture.update(None, image.raw_pixels().as_slice(), tex_width as int * 4);

        Fonts {
            // characters: charlist,
            charmap: charmap,
            texture: texture,
            renderer: renderer.downgrade()
        }
    }

    // Get the position in our font texture that corresponds to the character
    pub fn get_char_rect(&self, character: char) -> Rect {
        if !self.charmap.contains_key(&character) {
            panic!("No '{}' character in font!", character);
        }
        self.charmap[character]
    }

    pub fn place_character_at_point(&self, character: char, x: i32, y: i32) {
        let source = self.get_char_rect(character);
        let dest = Rect::new(MARGIN + x * CHAR_WIDTH, MARGIN + y * (CHAR_HEIGHT + LINE_SPACING),
                             CHAR_WIDTH, CHAR_HEIGHT);
        let renderer = self.renderer.upgrade().unwrap();
        let result = renderer.copy(&self.texture, Some(source), Some(dest));
        match result {
            Ok(())   => (),
            Err(err) => panic!("Error copying font texture from {} to {}: {}", source, dest, err)
        }
    }
}