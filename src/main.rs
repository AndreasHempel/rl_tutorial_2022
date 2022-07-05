use bevy::prelude::*;
use bevy_inspector_egui::{RegisterInspectable, WorldInspectorPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new())
        .add_system(bevy::input::system::exit_on_esc_system)
        .add_startup_system(render::setup_camera)
        .add_startup_system(spawner::setup_player)
        .add_startup_system(spawner::spawn_monster)
        .add_state(GameState::Ticking)
        .add_system(monster_ai::monster_actors)
        .add_system(input_handler::keyboard_event_handler)
        .add_system_set(SystemSet::on_update(GameState::Ticking).with_system(actions::move_actors))
        .add_system_set(
            SystemSet::on_enter(GameState::Ticking).with_system(actions::enqueue_actors),
        )
        .add_system_set(
            SystemSet::on_update(GameState::Ticking).with_system(actions::wait_for_player),
        )
        .add_system(render::place_characters)
        .register_inspectable::<components::Position>()
        .run();
}

mod actions;
mod components;
mod input_handler;
mod monster_ai;
mod render;
mod spawner;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
    WaitingForPlayer,
    Ticking,
}
