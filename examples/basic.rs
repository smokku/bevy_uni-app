use bevy::{input::keyboard::KeyboardInput, prelude::*};
use bevy_uni_app::{AppConfig, UniAppPlugin};

pub fn main() {
    log::info!("Starting basic example");

    App::build()
        .add_default_plugins()
        // plugin stuff
        .add_resource(AppConfig::new("Bevy uni-app", (400, 300)))
        // .add_resource::<InitFn>(Arc::new(Box::new(init)))
        // .add_resource::<RenderFn>(Arc::new(Box::new(render)))
        .add_plugin(UniAppPlugin)
        // example stuff
        .add_system(input.system())
        .run();
}

#[derive(Default)]
struct EventsState {
    keyboard_input_event_reader: EventReader<KeyboardInput>,
    cursor_moved_event_reader: EventReader<CursorMoved>,
}

fn input(
    mut state: Local<EventsState>,
    keyboard_input_events: Res<Events<KeyboardInput>>,
    cursor_moved_events: Res<Events<CursorMoved>>,
) {
    for event in state
        .keyboard_input_event_reader
        .iter(&keyboard_input_events)
    {
        println!("keyboard input {:?}", event);
    }
    for event in state.cursor_moved_event_reader.iter(&cursor_moved_events) {
        println!("mouse input {:?}", event);
    }
}
