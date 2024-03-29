#![deny(missing_docs)]

//! Main entrypoint for this roguelike project

use bevy::prelude::*;
use bevy_inspector_egui::{RegisterInspectable, WorldInspectorPlugin};
use clap::Parser;

/// Describes the parameters that may be passed from the CLI
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct CLIArgs {
    /// What map builder to use
    #[clap(short = 'm', long = "map", value_enum, default_value = "rooms")]
    map_builder: level::MapBuilder,

    /// Seed for map building RNG
    #[clap(short = 's', long = "seed", default_value = "42")]
    rng_seed: u64,

    /// Flag to enable WorldInspector
    #[clap(short = 'i', long = "inspector", action, default_value = "false")]
    inspector: bool,
}

/// All possible game states
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
    /// Waiting for player input
    WaitingForPlayer,
    /// Iterating through all active actors to resolve their actions
    Ticking,
    /// Entering a new level (or starting the game)
    EnterNewLevel,
    /// The player ran out of time
    GameOver,
    /// Starting a new game, e.g. upon launch or after a [`GameOver`](GameState::GameOver)
    StartGame,
}

#[cfg(debug_assertions)]
/// Adds various debugging tools
struct DebugPlugin;

#[cfg(debug_assertions)]
impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(bevy::diagnostic::FrameTimeDiagnosticsPlugin::default())
            .add_plugin(bevy::diagnostic::LogDiagnosticsPlugin::default());
    }
}

/// Enables world inspector and related settings
struct InspectorPlugin;

impl Plugin for InspectorPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(WorldInspectorPlugin::new())
            .register_inspectable::<components::Position>();
    }
}

fn main() {
    let args = CLIArgs::parse();

    let mut app = App::new();
    app.add_plugins(DefaultPlugins)
        .add_plugin(bevy_egui::EguiPlugin)
        .insert_resource(WindowDescriptor {
            title: "Roguelike tutorial 2022 - Andreas Hempel".to_string(),
            width: 1422.0,
            height: 800.0,
            // present_mode: PresentMode::AutoVsync,
            ..default()
        })
        .add_plugin(game_state::GameStatePlugin)
        .add_plugin(map::MapPlugin)
        .add_plugin(level::LevelPlugin {
            builder: args.map_builder,
            seed: args.rng_seed,
        })
        .add_plugin(render::RenderPlugin)
        .add_plugin(ui::UIPlugin)
        .add_plugin(spawner::SpawningPlugin)
        .add_plugin(actions::ActionPlugin)
        .add_plugin(monster_ai::AIPlugin)
        .add_plugin(input_handler::KeyboardInputPlugin)
        .add_plugin(player::PlayerPlugin)
        .add_system(visibility::determine_visibility);

    #[cfg(debug_assertions)]
    app.add_plugin(DebugPlugin)
        .add_system(bevy::input::system::exit_on_esc_system);

    if args.inspector {
        app.add_plugin(InspectorPlugin);
    }

    app.run();
}

mod actions;
mod components;
mod game_state;
mod input_handler;
mod level;
mod map;
mod map_builder;
mod monster_ai;
mod motion_resolver;
mod player;
mod render;
mod spawner;
mod ui;
mod visibility;
