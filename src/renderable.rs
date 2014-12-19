use std::rc::Rc;

use sdl2::rect::Rect;

pub trait Renderable {
    fn render(&self, x: i32, y: i32) -> Option<char>;
}

pub struct Panel {
    rect: Rect,
    border: bool,
    children: Vec<Rc<Panel>>,
}

impl Panel {
    pub fn new(rect: Rect, border: bool) -> Panel {
        Panel {
            rect: rect,
            border: border,
            children: Vec::new(),
        }
    }

    pub fn add_child(&mut self, new_child: Rc<Panel>) {
        self.children.push(new_child);
    }
}

impl Renderable for Panel {
    fn render(&self, x: i32, y: i32) -> Option<char> {
        let rect = self.rect;

        if x < rect.x || x > rect.x + rect.w {
            return None
        } else if y < rect.y || y > rect.y + rect.h {
            return None
        }

        if self.border {
            let mut horizontal = false;
            let mut vertical = false;
            if rect.x == x || rect.x + rect.w == x {
                horizontal = true;
            }
            if rect.y == y || rect.y + rect.h == y {
                vertical = true;
            }

            match (horizontal, vertical) {
                (true, true)  => return Some('+'),
                (true, false) => return Some('|'),
                (false, true) => return Some('-'),
                _             => ()
            }            
        }
        for child in self.children.iter() {
            if let Some(character) = child.render(x - rect.x, y - rect.y) {
                return Some(character)
            }
        }
        Some(' ')
    }
}