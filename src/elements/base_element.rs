use dyn_clone::DynClone;
use sdl2::ttf::Sdl2TtfContext;

pub trait Element: DynClone {
    fn render(&mut self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>, ttf_context: &Sdl2TtfContext,);
    fn process(&self, input_events: crate::events::input_events::InputEvent);

    fn is_rendered(&self) -> bool {
        false
    }
}

dyn_clone::clone_trait_object!(Element);

#[derive(Clone)]
pub struct BaseElement {
    pub elements: Vec<Box<dyn Element>>,
}