#![allow(unused_must_use)]

extern crate sdl2;

use std::rc::Rc;

use sdl2::video::{WindowPos, Window, OPENGL};
use sdl2::timer::{delay};
use sdl2::event::{poll_event, Event};
use sdl2::render::Renderer;

use font::Fonts;
use font::{MARGIN, CHAR_WIDTH, CHAR_HEIGHT, LINE_SPACING};

use renderable::{Panel, Renderable, Label, Progress};

mod font;
mod renderable;

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
    let renderer = Rc::new(renderer);st

    let mut fonts = Fonts::new(renderer.clone());
    fonts.set_background_color(23, 54, 89);
    fonts.set_foreground_color(255, 128, 196);

    // Initialize the test scene
    let mut panel = Panel::new(sdl2::rect::Rect::new(0, 0, 32, 10), true);
    let mut child = Panel::new(sdl2::rect::Rect::new(9, 3, 9, 4), true);
    let mut progress = Progress::new(sdl2::rect::Point::new(1, 1), 8);
    progress.value = 5;
    child.add_child(box progress);
    panel.add_child(box child);
    let child = Panel::new(sdl2::rect::Rect::new(3, 2, 10, 4), true);
    panel.add_child(box child);
    let child = Panel::new(sdl2::rect::Rect::new(15, 4, 8, 5), true);
    panel.add_child(box child);
    let child = Label::new(sdl2::rect::Point::new(1, 1), "Hello!");
    panel.add_child(box child);

    loop {
        renderer.clear();
        let should_quit = events();
        if should_quit { break; }
        for y in range(0, NUM_CHARS_Y) {
            for x in range(0, NUM_CHARS_X) {
                if let Some(character) = panel.render(x, y) {
                    fonts.place_character_at_point(character, x, y);
                }
            }
        }
        renderer.present();
        delay(1000/60)
    }

    sdl2::quit()
}

fn events() -> bool {
    loop {
        match poll_event() {
            Event::None => return false,
            Event::Quit(_) => return true,
            _ => (),
        }
    }
}