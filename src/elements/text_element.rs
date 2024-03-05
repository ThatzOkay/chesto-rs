use sdl2::{pixels::Color, render::Canvas};

use crate::{draw_utils::draw_utils, Element};

#[derive(Clone)]
pub struct TextElement {
    pub text: String,
    pub size: u32,
    pub color: Option<Color>,
    pub rendered: bool,
}

impl TextElement {
    pub fn new(text: &str, size: u32, color: Option<Color>) -> Self {
        TextElement {
            text: text.to_string(),
            size,
            color,
            rendered: false,
        }
    }
}

impl Element for TextElement {
    fn render(&mut self, canvas: &mut Canvas<sdl2::video::Window>, ttf_context: &sdl2::ttf::Sdl2TtfContext) {
        println!("Rendering TextElement with text: {}", self.text);
        
        self.rendered = true;
    }
    
    fn process(&self, input_events: crate::events::input_events::InputEvent) {
    }

    fn is_rendered(&self) -> bool {
        self.rendered
    }
}