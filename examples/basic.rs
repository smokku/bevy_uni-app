use bevy::{input::keyboard::KeyboardInput, input::mouse::MouseButtonInput, prelude::*};
use bevy_uni_app::{AppConfig, UniAppPlugin};

pub fn main() {
    log::info!("Starting basic example");

    App::build()
        .add_default_plugins()
        // plugin stuff
        .add_resource(AppConfig::new("Bevy uni-app", (640, 480)))
        .add_plugin(UniAppPlugin)
        // example stuff
        .add_system(input.system())
        .run();
}

#[derive(Default)]
struct EventsState {
    keyboard_input_event_reader: EventReader<KeyboardInput>,
    mouse_button_event_reader: EventReader<MouseButtonInput>,
    cursor_moved_event_reader: EventReader<CursorMoved>,
}

fn input(
    mut state: Local<EventsState>,
    keyboard_input_events: Res<Events<KeyboardInput>>,
    mouse_button_events: Res<Events<MouseButtonInput>>,
    cursor_moved_events: Res<Events<CursorMoved>>,
) {
    for event in state
        .keyboard_input_event_reader
        .iter(&keyboard_input_events)
    {
        println!("keyboard input {:?}", event);
    }
    for event in state.cursor_moved_event_reader.iter(&cursor_moved_events) {
        println!("mouse moved {:?}", event);
    }
    for event in state.mouse_button_event_reader.iter(&mouse_button_events) {
        println!("mouse button {:?}", event);
    }
}
