
use sdl2::keyboard::Keycode;
use crate::draw_utils::draw_utils::{self};
use crate::{InputEvent, InputEvents, Screen};

pub struct BaseRootDisplay {
    pub width: u32,
    pub height: u32,
    pub running: bool,
    subscreen: Option<Box<dyn Screen>>,
    next_subscreen: Option<Box<dyn Screen>>,
    pub sdl_context: Option<sdl2::Sdl>,
    pub canvas: Option<sdl2::render::Canvas<sdl2::video::Window>>,
    pub window: Option<sdl2::video::Window>,
    pub ttf_context: Option<sdl2::ttf::Sdl2TtfContext>,
    pub controller: Option<sdl2::controller::GameController>,
    pub event_pump: Option<sdl2::EventPump>,
    pub input_events: crate::events::input_events::InputEvent,
}

pub trait RootDisplay {
    fn process(&mut self, input_events: InputEvent) -> bool;
    fn new(width: u32, height: u32) -> BaseRootDisplay;
    fn main_loop(&mut self);
    fn set_screen(&mut self, new_screen: Box<dyn Screen>);
}

impl RootDisplay for BaseRootDisplay {
    fn new(width: u32, height: u32) -> Self  {
        BaseRootDisplay {
            width,
            height,
            running: true,
            subscreen: None,
            next_subscreen: None,
            sdl_context: None,
            canvas: None,
            window: None,
            ttf_context: None,
            controller: None,
            event_pump: None,
            input_events: InputEvents::new(),
        }
    }

    fn set_screen(&mut self, new_screen: Box<dyn Screen>) {
        self.next_subscreen = Some(new_screen);
    }

    fn process(&mut self, input_events: InputEvent) -> bool {
        
        if self.next_subscreen.is_some() {
            self.subscreen = self.next_subscreen.take();
            return true;
        }

        if self.subscreen.is_some() {
            let subscreen = self.subscreen.as_mut().unwrap();
            if subscreen.should_switch_screen() {
                let next_subscreen = subscreen.next_subscreen().take();
                self.subscreen = next_subscreen;
                return true;
            }
        }
        
        if self.subscreen.is_some() {
            self.subscreen.as_mut().unwrap().process(input_events);
        }
        
        false
    }

    fn main_loop(&mut self) {

        let init_succeded = draw_utils::draw_init(self);
        
        if !init_succeded {
            println!("Error initializing display");
            return;
        }
        
        let mut input_events = self.input_events.clone();

        while self.running {
            let mut at_least_one_new_event = false;
            let mut view_changed = false;

            let frame_start = draw_utils::get_ticks();
            
            while input_events.update(self.event_pump.as_mut().unwrap()) {
                // process the input of the supplied event
                let mut input_events = input_events.clone();
                
                view_changed |= self.process(input_events.clone());
                at_least_one_new_event = true;

                if input_events.pressed(None, Some(sdl2::controller::Button::Back)) || input_events.pressed(Some(Keycode::Escape), None) {
                    if input_events.quit_action.is_some() {
                        input_events.quit_action.unwrap()();
                    } else {
                        self.running = false;
                    }
                }
            }

		    // one more event update if nothing changed or there were no previous events seen
		    // needed to non-input related processing that might update the screen to take place
            if !at_least_one_new_event && !view_changed {
                input_events.update(self.event_pump.as_mut().unwrap());
                view_changed |= self.process(input_events.clone());
            }

            if view_changed {
                self.subscreen.as_mut().unwrap().render(self.canvas.as_mut().unwrap(), self.ttf_context.as_ref().unwrap());
            } else {
                // delay for the remainder of the frame to keep up to 60fps
                // (we only do this if we didn't draw to not waste energy
                // if we did draw, then proceed immediately without waiting for smoother progress bars / scrolling)
                let mut delay_time = draw_utils::get_ticks() - frame_start;
                if delay_time <= 0 {
                    delay_time = 0;
                }
                if delay_time < 16 {
                    std::thread::sleep(std::time::Duration::from_millis((16 - delay_time).into()));
                }
            }
        }
    }   
}