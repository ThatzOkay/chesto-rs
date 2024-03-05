use sdl2::{pixels::Color, rect::Rect, render::Canvas};

use super::Element;

#[derive(Clone)]
pub struct ImageElement {
    pub src: String,
    pub color: Option<Color>,
    pub rendered: bool,
}

impl Element for ImageElement {

    fn render(&mut self, canvas: &mut Canvas<sdl2::video::Window>, _ttf_context: &sdl2::ttf::Sdl2TtfContext) {
        println!("Rendering ImageElement with src: {}", self.src);

        if self.color.is_some() {
            let color = self.color.unwrap();
            canvas.set_draw_color(color);
        } else {
            canvas.set_draw_color(Color::RGB(255, 0, 0));
        }
        
        let result = canvas.draw_rect(Rect::new(0, 0, 100, 100));

        self.rendered = true;
    }
    
    fn process(&self, input_events: crate::events::input_events::InputEvent) {
        
    }
    
    fn is_rendered(&self) -> bool {
        self.rendered
    }
}