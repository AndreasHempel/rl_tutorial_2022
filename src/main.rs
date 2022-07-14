#![deny(missing_docs)]

//! Main entrypoint for this roguelike project

use bevy::prelude::*;
use clap::Parser;

#[cfg(debug_assertions)]
use bevy_inspector_egui::{RegisterInspectable, WorldInspectorPlugin};

/// Describes the parameters that may be passed from the CLI
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct CLIArgs {
    /// What map builder to use
    #[clap(short = 'm', long = "map", value_enum, default_value = "rooms")]
    map_builder: map::MapBuilder,
}

/// All possible game states
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
    /// Waiting for player input
    WaitingForPlayer,
    /// Iterating through all active actors to resolve their actions
    Ticking,
}

#[cfg(debug_assertions)]
/// Adds various debugging tools
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
    let args = CLIArgs::parse();

    let mut app = App::new();
    app.add_plugins(DefaultPlugins)
        .add_state(GameState::Ticking)
        .add_plugin(map::MapPlugin {
            builder: args.map_builder,
        })
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
mod map_builder;
mod monster_ai;
mod render;
mod spawner;
