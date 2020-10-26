use bevy::app::{App, AppBuilder, AppExit, EventReader, Events, Plugin};
use bevy::input::{
    keyboard::{ElementState, KeyboardInput},
    mouse::MouseButtonInput,
};
use bevy::math::Vec2;
use bevy::window::{
    CursorMoved, WindowCreated, WindowDescriptor, WindowId, WindowMode, WindowResized,
};
use std::sync::Arc;
use uni_app::App as UniApp;

pub use uni_app::{self as uni_app, AppConfig};

mod converters;
use converters::*;

// pub type InitFn = Arc<Box<dyn Fn(&mut dyn UniAppApi, &mut App) -> () + Send + Sync>>;
// pub type RenderFn = Arc<Box<dyn Fn(&mut App, &mut dyn UniAppApi) -> () + Send + Sync>>;

#[derive(Default)]
pub struct UniAppPlugin;

impl Plugin for UniAppPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.set_runner(runner);
    }
}

fn runner(app: App) {
    log::debug!("Entering uni-app event loop");

    let config = {
        let config = app.resources.get::<AppConfig>();
        if config.is_some() {
            let config = config.unwrap();
            AppConfig {
                title: config.title.clone(),
                size: config.size,
                vsync: config.vsync,
                headless: config.headless,
                fullscreen: config.fullscreen,
                resizable: config.resizable,
                show_cursor: config.show_cursor,
                intercept_close_request: config.intercept_close_request,
            }
        } else {
            let mut config = AppConfig::new("game", (800, 600));
            if let Some(desc) = app.resources.get::<WindowDescriptor>() {
                config.title = desc.title.clone();
                config.size = (desc.width, desc.height);
                config.vsync = desc.vsync;
                config.fullscreen = match desc.mode {
                    WindowMode::Windowed => false,
                    WindowMode::BorderlessFullscreen | WindowMode::Fullscreen { .. } => true,
                };
                config.resizable = desc.resizable;
                config.show_cursor = desc.cursor_visible;
            }

            config
        }
    };

    let uni_app = UniApp::new(config);

    uni_app.run(move |app: &mut UniApp| {
        for evt in app.events.borrow().iter() {
            // print on stdout (native) or js console (web)
            uni_app::App::print(format!("{:?}\n", evt));
            // exit on key ou mouse press
            match evt {
                &uni_app::AppEvent::KeyUp(_) => {
                    uni_app::App::exit();
                }
                &uni_app::AppEvent::MouseUp(_) => {
                    uni_app::App::exit();
                }
                _ => (),
            }
        }
    });
}

// #[derive(Default, Debug)]
// pub struct Window {
//     pub width: usize,
//     pub height: usize,
//     pub cursor_x: usize,
//     pub cursor_y: usize,
// }

// impl Window {
//     fn new(width: usize, height: usize) -> Self {
//         Window {
//             width,
//             height,
//             ..Window::default()
//         }
//     }
// }

// pub fn UniApp_runner(app: App) {}

// struct BevyEngine {
//     app: App,
//     app_exit_event_reader: EventReader<AppExit>,
//     last_mouse_pos: (f32, f32),
// }

// impl BevyEngine {
//     pub fn new(app: App) -> Self {
//         let app_exit_event_reader = EventReader::<AppExit>::default();

//         BevyEngine {
//             app,
//             app_exit_event_reader,
//             last_mouse_pos: (0.0, 0.0),
//         }
//     }
// }

// impl Engine for BevyEngine {
//     fn init(&mut self, api: &mut dyn UniAppApi) {
//         let (width, height) = api.get_screen_size();

//         self.app
//             .resources
//             .insert(Window::new(width as usize, height as usize));

//         {
//             let mut window_created_events = self
//                 .app
//                 .resources
//                 .get_mut::<Events<WindowCreated>>()
//                 .unwrap();
//             window_created_events.send(WindowCreated {
//                 id: WindowId::primary(),
//             });
//         }

//         self.app.initialize();

//         let init_function = if let Some(fn_ref) = self.app.resources.get::<InitFn>() {
//             Some((*fn_ref).clone())
//         } else {
//             None
//         };

//         if let Some(init_function) = init_function {
//             init_function(api, &mut self.app);
//         }
//     }

//     fn resize(&mut self, api: &mut dyn UniAppApi) {
//         let (width, height) = api.get_screen_size();

//         let mut window_resized_events = self
//             .app
//             .resources
//             .get_mut::<Events<WindowResized>>()
//             .unwrap();
//         window_resized_events.send(WindowResized {
//             id: WindowId::primary(),
//             width: width as usize,
//             height: height as usize,
//         });
//     }

//     fn update(&mut self, api: &mut dyn UniAppApi) -> Option<UpdateEvent> {
//         if let Some(app_exit_events) = self.app.resources.get_mut::<Events<AppExit>>() {
//             if self
//                 .app_exit_event_reader
//                 .latest(&app_exit_events)
//                 .is_some()
//             {
//                 return Some(UpdateEvent::Exit);
//             }
//         }

//         let input = api.input();

//         for key in input.keys_pressed() {
//             let mut keyboard_input_events = self
//                 .app
//                 .resources
//                 .get_mut::<Events<KeyboardInput>>()
//                 .unwrap();
//             let input_event = KeyboardInput {
//                 scan_code: 0,
//                 state: ElementState::Pressed,
//                 key_code: convert_key_str(key),
//             };
//             keyboard_input_events.send(input_event);
//         }

//         for key in input.keys_released() {
//             let mut keyboard_input_events = self
//                 .app
//                 .resources
//                 .get_mut::<Events<KeyboardInput>>()
//                 .unwrap();
//             let input_event = KeyboardInput {
//                 scan_code: 0,
//                 state: ElementState::Released,
//                 key_code: convert_key_str(key),
//             };
//             keyboard_input_events.send(input_event);
//         }

//         for button in 0..3 {
//             if input.mouse_button_pressed(button) {
//                 let mut mouse_button_input_events = self
//                     .app
//                     .resources
//                     .get_mut::<Events<MouseButtonInput>>()
//                     .unwrap();
//                 mouse_button_input_events.send(MouseButtonInput {
//                     button: convert_mouse_button(button),
//                     state: ElementState::Pressed,
//                 });
//             }

//             if input.mouse_button_released(button) {
//                 let mut mouse_button_input_events = self
//                     .app
//                     .resources
//                     .get_mut::<Events<MouseButtonInput>>()
//                     .unwrap();
//                 mouse_button_input_events.send(MouseButtonInput {
//                     button: convert_mouse_button(button),
//                     state: ElementState::Released,
//                 });
//             }
//         }

//         let mouse_pos = input.mouse_pos();
//         if self.last_mouse_pos != mouse_pos {
//             let mut cursor_moved_events =
//                 self.app.resources.get_mut::<Events<CursorMoved>>().unwrap();
//             cursor_moved_events.send(CursorMoved {
//                 id: WindowId::primary(),
//                 position: Vec2::new(mouse_pos.0, mouse_pos.1),
//             });
//         }

//         self.app.update();

//         None
//     }

//     fn render(&mut self, api: &mut dyn UniAppApi) {
//         let render_function = {
//             let fn_ref = self
//                 .app
//                 .resources
//                 .get::<RenderFn>()
//                 .expect("Cannot find render function resource");
//             (*fn_ref).clone()
//         };

//         render_function(&mut self.app, api);
//     }
// }
