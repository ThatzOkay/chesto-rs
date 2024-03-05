pub mod draw_utils {
    use sdl2::{render::Canvas, sys::SDL_GetTicks};

    use crate::BaseRootDisplay;

    pub fn draw_init(root: &mut BaseRootDisplay) -> bool {

        let sdl_context = sdl2::init();

        let sdl_context = match sdl_context {
            Ok(sdl_context) => sdl_context,
            Err(err) => {
                println!("Error initializing SDL: {}", err);
                return false;
            }
        };

        let ttf_context = sdl2::ttf::init();

        let ttf_context = match ttf_context {
            Ok(ttf_context) => ttf_context,
            Err(err) => {
                println!("Error initializing TTF: {}", err);
                return false;
            }
        };

        let controller_subsystem = sdl_context.game_controller();

        let controller_subsystem = match controller_subsystem {
            Ok(controller_subsystem) => controller_subsystem,
            Err(err) => {
                println!("Error initializing controller subsystem: {}", err);
                return false;
            }
        };

        let available = controller_subsystem
            .num_joysticks();

        let available = match available {
            Ok(available) => available,
            Err(err) => {
                println!("Error getting number of joysticks: {}", err);
                return false;
            }
        };

        let controller = (0..available)
            .find_map(|id| {
                if controller_subsystem.is_game_controller(id) {
                    Some(controller_subsystem.open(id))
                } else {
                    None
                }
            });

        let controller = match controller {
            Some(controller) => controller,
            None => {
                println!("Error opening controller: {}", available);
                println!("Controller support is not available");
                Err(sdl2::IntegerOrSdlError::SdlError("Controller support is not available".to_string()))
            }
        };

        let video_subsystem = sdl_context.video();

        let video_subsystem = match video_subsystem {
            Ok(video_subsystem) => video_subsystem,
            Err(err) => {
                println!("Error initializing video subsystem: {}", err);
                return false;
            }
        };

        let window = video_subsystem.window("", root.width, root.height)
            .position_centered()
            .build();

        let window = match window {
            Ok(window) => window,
            Err(err) => {
                println!("Error creating window: {}", err);
                return false;
            }
        };
        
        let canvas = window
            .into_canvas()
            .target_texture()
            .present_vsync()
            .build();

        let canvas = match canvas {
            Ok(canvas) => canvas,
            Err(err) => {
                println!("Error creating canvas: {}", err);
                return false;
            }
        };

        root.event_pump = Some(sdl_context.event_pump().unwrap());
        root.sdl_context = Some(sdl_context);
        root.canvas = Some(canvas);
        //root.window = Some(canvas.window());
        root.ttf_context = Some(ttf_context);
        if controller.is_ok() {
            root.controller = Some(controller.unwrap());
        }

        true
    }

    pub fn get_ticks() -> u32 {
        unsafe { SDL_GetTicks() }
    }

    pub fn draw_line(canvas: &mut Canvas<sdl2::video::Window>, x: i32, y: i32, w: i32, h: i32) -> Result<(), String> {
        let result = canvas.draw_line((x, y), (w, h));
        match result {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }
}