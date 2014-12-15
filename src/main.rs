extern crate sdl2;

use std::rc::Rc;

use sdl2::video::{WindowPos, Window, OPENGL};
use sdl2::timer::{delay};
use sdl2::event::{poll_event, Event};
use sdl2::render::Renderer;

use font::Fonts;
use font::{MARGIN, CHAR_WIDTH, CHAR_HEIGHT, LINE_SPACING};

mod font;

const NUM_CHARS_X : i32 = 80;
const NUM_CHARS_Y : i32 = 24;

fn main() {
    let width = (NUM_CHARS_X * CHAR_WIDTH + 2 * MARGIN) as int;
    let height = (NUM_CHARS_Y * (CHAR_HEIGHT + LINE_SPACING) + 2 * MARGIN) as int;

    sdl2::init(sdl2::INIT_VIDEO);
    let window = match Window::new("Hello, Window!", WindowPos::PosCentered,
        WindowPos::PosCentered, width, height, OPENGL) {
        Ok(window) => window,
        Err(error) => panic!("Failed to create window: {}", error)
    };

    let renderer = Renderer::from_window(window,
        sdl2::render::RenderDriverIndex::Auto,
        sdl2::render::RendererFlags::empty()).unwrap();
    let renderer = Rc::new(renderer);

    let mut fonts = Fonts::new(renderer.clone());
    fonts.set_background_color(23, 54, 89);
    fonts.set_foreground_color(255, 128, 196);

    let mut message = String::new();
    loop {
        renderer.clear();
        let should_quit = events(&fonts, &mut message);
        if should_quit { break; }
        for (i, character) in message.chars().enumerate() {
            fonts.place_character_at_point(character, i as i32 % NUM_CHARS_X, i as i32 / NUM_CHARS_X);
        }
        renderer.present();
        delay(1000/60)
    }

    sdl2::quit()
}

fn events(fonts: &Fonts, message: &mut String) -> bool {
    loop {
        match poll_event() {
            Event::None => return false,
            Event::Quit(_) => return true,
            Event::Window(_, _, _, _, _) => (),
            Event::MouseMotion(_, _, _, _, _, _, _, _) => (),
            Event::KeyDown(_, _, sdl2::keycode::KeyCode::Backspace, _, _, _) => {
                message.pop();
            }
            Event::TextInput(_, _, text) => {
                message.push(text.char_at(0));
            }
            event => (), //println!("Event {}", event)
        }
    }
}