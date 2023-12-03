use bevy::app::App;
use bevy_editor::EditorPlugin;

fn main() {
    App::new().add_plugins(EditorPlugin {}).run();
}
