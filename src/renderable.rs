use sdl2::rect::{Rect, Point};

/// The renderable trait provides a way to ask what character is at
/// a specified screen position
pub trait Renderable {
    fn render(&self, x: i32, y: i32) -> Option<char>;
}

/// Panels can have borders and can contain other renderables
pub struct Panel<'a> {
    rect: Rect,
    border: bool,
    children: Vec<Box<Renderable+'a>>,
}

impl<'a> Panel<'a> {
    pub fn new(rect: Rect, border: bool) -> Panel<'a> {
        Panel {
            rect: rect,
            border: border,
            children: Vec::new(),
        }
    }

    pub fn add_child(&mut self, new_child: Box<Renderable+'a>) {
        self.children.push(new_child);
    }
}

impl<'a> Renderable for Panel<'a> {
    fn render(&self, x: i32, y: i32) -> Option<char> {
        let rect = self.rect;

        // If the requested position is outside the bounds
        if x < rect.x || x > rect.x + rect.w {
            return None
        } else if y < rect.y || y > rect.y + rect.h {
            return None
        }

        // Draw the border if it's enabled
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

        // Draw all the children inside the panel
        for child in self.children.iter() {
            if let Some(character) = child.render(x - rect.x, y - rect.y) {
                return Some(character)
            }
        }

        // Panels are solid, so there's a space character instead of None
        // This may change if I add transparent panels
        Some(' ')
    }
}

/// Labels have a position and a width, and can contain text
pub struct Label {
    origin: Point,
    width: i32,
    text: String
}

impl Label {
    pub fn new(origin: Point, text: &str) -> Label {
        Label {
            origin: origin.clone(),
            width: text.char_len() as i32,
            text: String::from_str(text)
        }
    }
}

impl Renderable for Label {
    fn render(&self, x: i32, y: i32) -> Option<char> {
        // An x in the local coordinate system
        let x = x - self.origin.x;

        // If the requested position is outside the label
        if y != self.origin.y || x < 0 || x >= self.width {
            return None
        }

        // Find the character at this point in the label
        let character = self.text.char_at(x as uint);
        Some(character)
    }
}

pub struct Progress {
    origin: Point,
    width: i32,
    pub min_value: i32,
    pub max_value: i32,
    pub value: i32
}

impl Progress {
    pub fn new(origin: Point, width: i32) -> Progress {
        Progress {
            origin: origin,
            width: width,
            min_value: 0,
            max_value: width-2,
            value: 0
        }
    }
}

impl Renderable for Progress {
    fn render(&self, x: i32, y: i32) -> Option<char> {
        // An x in the local coordinate system
        let x = x - self.origin.x;

        // If outside the progress bar
        if y != self.origin.y || x < 0 || x >= self.width {
            return None
        }

        // Render the progress bar
        if x == 0 { 
            Some('[')
        } else if self.width - 1 == x {
            Some(']')
        } else {
            let range = self.max_value - self.min_value;
            // - 2 and + 1 are to account for the brackets
            let param = (self.value - self.min_value)*(self.width-2)/range + 1;
            if x < param {
                Some('=')
            } else {
                Some(' ')
            }
        }
    }
}