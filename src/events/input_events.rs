use sdl2::{event::Event, keyboard::Keycode};

#[derive(Clone)]
pub struct InputEvent {
    pub quit_action: Option<fn() -> ()>,
    input_type: Event,
    key_code: i32,
    noop: bool
}

pub trait InputEvents {
    fn new() -> Self;
    fn update(&mut self, event_pump: &mut sdl2::EventPump) -> bool;
    fn pressed(&mut self, keyboard_button: Option<Keycode>, controller_button: Option<sdl2::controller::Button>) -> bool;
    fn process_sdl_events(&mut self, event_pump: &mut sdl2::EventPump) -> bool;
    fn is_key_down(&mut self) -> bool;
    fn held(&mut self, keyboard_button: Option<Keycode>, controller_button: Option<sdl2::controller::Button>) -> bool;
}

impl InputEvents for InputEvent {
    fn new() -> Self {
        InputEvent {
            quit_action: None,
            input_type: Event::Unknown { type_: 0, timestamp: 0 },
            key_code: -1,
            noop: false,
        }
    }

    fn update(&mut self, event_pump: &mut sdl2::EventPump) -> bool {
        self.input_type = Event::Unknown { type_: 0, timestamp: 0 };
        self.key_code = -1;
        self.noop = false;

        return self.process_sdl_events(event_pump);
    }

    fn pressed(&mut self, keyboard_button: Option<Keycode>, controller_button: Option<sdl2::controller::Button>) -> bool {
        self.is_key_down() && self.held(keyboard_button, controller_button)
    }

    fn process_sdl_events(&mut self, event_pump: &mut sdl2::EventPump) -> bool {
        
        let event = event_pump.poll_event();
        
        if event.is_none() {
            return false;
        }

        let event = event.unwrap();

        self.input_type = event;
        self.noop = false;

        if let Event::Quit { timestamp: _ } = self.input_type {
            if self.quit_action.is_some() {
                self.quit_action.unwrap()();
            }
            return false; //Quitting overrides all other events.
        }

        true
    }

    fn is_key_down(&mut self) -> bool {
        if let Event::KeyDown { keycode: Some(keycode), repeat: false, .. } = self.input_type {
            self.key_code = keycode as i32;
            return true;
        }

        if let Event::ControllerButtonDown { button, .. } = self.input_type {
            self.key_code = button as i32;
            return true;
        }

        false
    }

    fn held(&mut self, keyboard_button: Option<Keycode>, controller_button: Option<sdl2::controller::Button>) -> bool {
        
        // if it's a key event
        if let Event::KeyDown { keycode: Some(keycode), repeat: false, .. } = self.input_type {
            if keyboard_button.is_some() && keyboard_button.unwrap() == keycode {
                return true;
            }
        }
        if let Event::KeyUp { keycode: Some(keycode), repeat: false, .. } = self.input_type {
            if keyboard_button.is_some() && keyboard_button.unwrap() == keycode {
                return false;
            }
        }

        if let Event::ControllerButtonDown { button, .. } = self.input_type {
            if controller_button.is_some() && controller_button.unwrap() == button {
                return true;
            }
        }

        if let Event::ControllerButtonUp { button, .. } = self.input_type {
            if controller_button.is_some() && controller_button.unwrap() == button {
                return false;
            }
        }

        false
    }
}