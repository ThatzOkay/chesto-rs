use dyn_clone::DynClone;
use sdl2::{pixels::Color, ttf::Sdl2TtfContext};

use crate::{BaseRootDisplay, Element};

pub trait Screen: DynClone {
    fn render(&mut self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>, ttf_context: &Sdl2TtfContext,) {

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        for element in self.elements() {
            element.render(canvas, ttf_context);
        }

        canvas.present();
    }

    fn process(&mut self, input_events: crate::events::input_events::InputEvent) -> bool {
        for element in self.elements() {
            element.process(input_events.clone());
        }
        true
    
    }

    fn should_switch_screen(&self) -> bool {
        let should_switch = match self.next_subscreen() {
            Some(_) => true,
            None => false,
        };
        should_switch
    }

    fn next_subscreen(&self) -> Option<Box<dyn Screen>>;

    fn elements(&mut self) -> &mut Vec<Box<dyn Element>>;

    fn set_screen(&mut self, screen: Box<dyn Screen>);
}

dyn_clone::clone_trait_object!(Screen);

pub struct BaseScreen {
    pub elements: Vec<Box<dyn Element>>,
    pub root: BaseRootDisplay,
}

impl BaseScreen {
    pub fn render(&mut self, canvas: sdl2::render::Canvas<sdl2::video::Window>, ttf_context: sdl2::ttf::Sdl2TtfContext) {
        let mut canvas = canvas;
        let mut ttf_context = ttf_context;
        for element in &mut self.elements {
            element.render(&mut canvas, &mut ttf_context);
        }
    }
}
