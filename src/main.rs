use bevy::prelude::*;

#[cfg(debug_assertions)]
use bevy_inspector_egui::{RegisterInspectable, WorldInspectorPlugin};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
    WaitingForPlayer,
    Ticking,
}

#[cfg(debug_assertions)]
struct DebugPlugin;

#[cfg(debug_assertions)]
impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(WorldInspectorPlugin::new())
            .register_inspectable::<components::Position>()
            .add_plugin(bevy::diagnostic::FrameTimeDiagnosticsPlugin::default())
            .add_plugin(bevy::diagnostic::LogDiagnosticsPlugin::default());
    }
}

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins)
        .add_state(GameState::Ticking)
        .add_plugin(map::MapPlugin)
        .add_plugin(render::RenderPlugin)
        .add_plugin(spawner::SpawningPlugin)
        .add_plugin(actions::ActionPlugin)
        .add_plugin(monster_ai::AIPlugin)
        .add_plugin(input_handler::KeyboardInputPlugin);

    #[cfg(debug_assertions)]
    app.add_plugin(DebugPlugin)
        .add_system(bevy::input::system::exit_on_esc_system);

    app.run();
}

mod actions;
mod components;
mod input_handler;
mod map;
mod monster_ai;
mod render;
mod spawner;
