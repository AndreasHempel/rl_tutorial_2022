use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};

use crate::actions::PlayerTurns;

/// Bundles systems responsible for rendering
#[derive(Debug)]
pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(render_ui);
    }
}

fn render_ui(mut ctx: ResMut<EguiContext>, turns: Res<PlayerTurns>) {
    egui::SidePanel::right("Right panel").show(ctx.ctx_mut(), |ui| {
        ui.horizontal(|ui| {
            ui.label("Turns left: ");
            ui.label(turns.get_remaining().to_string());
        });
        ui.horizontal(|ui| {
            ui.label("Turns completed: ");
            ui.label(turns.get_completed().to_string());
        });
    });
}
