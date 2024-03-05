use sdl2::render::Canvas;

use crate::{Element, TextElement};


#[derive(Clone)]
pub struct ButtonElement {
    pub text: Option<TextElement>,
    pub rendered: bool,
}

impl ButtonElement {
    pub fn new(text: Option<TextElement>) -> Self {
        ButtonElement {
            text,
            rendered: false,
        }
    }
}

impl Element for ButtonElement {
    fn render(&mut self, canvas: &mut Canvas<sdl2::video::Window>, _ttf_context: &sdl2::ttf::Sdl2TtfContext) {
        


        self.rendered = true;
    }
    
    fn process(&self, input_events: crate::events::input_events::InputEvent) {
        
    }

    fn is_rendered(&self) -> bool {
        self.rendered
    }
}