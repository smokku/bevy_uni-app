use bevy::app::{App, AppBuilder, AppExit, EventReader, Events, Plugin};
use bevy::input::{
    keyboard::{ElementState, KeyboardInput},
    mouse::MouseButtonInput,
};
use bevy::math::Vec2;
use bevy::window::{
    CursorMoved, WindowCreated, WindowDescriptor, WindowId, WindowMode, WindowResized,
};
use uni_app::App as UniApp;

pub use uni_app::{self as uni_app, AppConfig};

mod converters;
use converters::*;

#[derive(Default)]
pub struct UniAppPlugin;

impl Plugin for UniAppPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.set_runner(runner);
    }
}

fn runner(mut bevy_app: App) {
    log::debug!("Entering uni-app event loop");

    let config = {
        let config = bevy_app.resources.get::<AppConfig>();
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
            if let Some(desc) = bevy_app.resources.get::<WindowDescriptor>() {
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

    {
        let mut window_created_events = bevy_app
            .resources
            .get_mut::<Events<WindowCreated>>()
            .unwrap();
        window_created_events.send(WindowCreated {
            id: WindowId::primary(),
        });
    }

    bevy_app.initialize();

    let mut app_exit_event_reader = EventReader::<AppExit>::default();

    uni_app.run(move |uni_app: &mut UniApp| {
        if let Some(app_exit_events) = bevy_app.resources.get_mut::<Events<AppExit>>() {
            if app_exit_event_reader.latest(&app_exit_events).is_some() {
                uni_app::App::exit();
            }
        }

        for evt in uni_app.events.borrow().iter() {
            match evt {
                &uni_app::AppEvent::Resized((width, height)) => {
                    let mut window_resized_events = bevy_app
                        .resources
                        .get_mut::<Events<WindowResized>>()
                        .unwrap();
                    window_resized_events.send(WindowResized {
                        id: WindowId::primary(),
                        width: width as usize,
                        height: height as usize,
                    });
                }

                uni_app::AppEvent::KeyDown(key_down_event) => {
                    let mut keyboard_input_events = bevy_app
                        .resources
                        .get_mut::<Events<KeyboardInput>>()
                        .unwrap();
                    let input_event = KeyboardInput {
                        scan_code: 0,
                        state: ElementState::Pressed,
                        key_code: convert_key_str(
                            key_down_event.key.as_str(),
                            key_down_event.code.as_str(),
                        ),
                    };
                    keyboard_input_events.send(input_event);
                }
                uni_app::AppEvent::KeyUp(key_up_event) => {
                    let mut keyboard_input_events = bevy_app
                        .resources
                        .get_mut::<Events<KeyboardInput>>()
                        .unwrap();
                    let input_event = KeyboardInput {
                        scan_code: 0,
                        state: ElementState::Released,
                        key_code: convert_key_str(
                            key_up_event.key.as_str(),
                            key_up_event.code.as_str(),
                        ),
                    };
                    keyboard_input_events.send(input_event);
                }

                uni_app::AppEvent::MouseDown(mouse_button_event)
                | uni_app::AppEvent::MouseUp(mouse_button_event) => {
                    let mut mouse_button_input_events = bevy_app
                        .resources
                        .get_mut::<Events<MouseButtonInput>>()
                        .unwrap();
                    mouse_button_input_events.send(MouseButtonInput {
                        button: convert_mouse_button(mouse_button_event.button),
                        state: match evt {
                            &uni_app::AppEvent::MouseDown(_) => ElementState::Pressed,
                            _ => ElementState::Released,
                        },
                    });
                }

                &uni_app::AppEvent::MousePos(mouse_pos) => {
                    let mut cursor_moved_events =
                        bevy_app.resources.get_mut::<Events<CursorMoved>>().unwrap();
                    cursor_moved_events.send(CursorMoved {
                        id: WindowId::primary(),
                        position: Vec2::new(mouse_pos.0 as f32, mouse_pos.1 as f32),
                    });
                }

                &uni_app::AppEvent::CharEvent(_) => {}

                &uni_app::AppEvent::CloseRequested => {
                    uni_app::App::exit();
                }
            }
        }

        bevy_app.update();
    });
}
