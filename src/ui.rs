use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};
use iyes_loopless::{prelude::IntoConditionalSystem, state::NextState};

use crate::{game_state::PlayerTurns, GameState};

/// Bundles systems responsible for rendering
#[derive(Debug)]
pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_ui_elements)
            .add_system(render_ui)
            .add_system(gameover_menu.run_in_state(GameState::GameOver));
    }
}

fn setup_ui_elements(mut ctx: ResMut<EguiContext>) {
    const FONT_LABEL: &str = "Sprite Font SDS";
    let mut fonts = egui::FontDefinitions::default();

    fonts.font_data.insert(
        FONT_LABEL.to_owned(),
        egui::FontData::from_static(include_bytes!("../assets/Dawnlike/GUI/SDS_8x8.ttf")),
    );

    fonts
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .insert(0, FONT_LABEL.to_owned());

    ctx.ctx_mut().set_fonts(fonts);
}

fn render_ui(mut ctx: ResMut<EguiContext>, turns: Option<Res<PlayerTurns>>) {
    egui::SidePanel::right("Right panel").show(ctx.ctx_mut(), |ui| {
        if let Some(turns) = turns {
            ui.horizontal(|ui| {
                ui.label("Turns left: ");
                ui.label(turns.get_remaining().to_string());
            });
            ui.horizontal(|ui| {
                ui.label("Turns completed: ");
                ui.label(turns.get_completed().to_string());
            });
        }
    });
}

fn gameover_menu(mut ctx: ResMut<EguiContext>, mut commands: Commands, turns: Res<PlayerTurns>) {
    egui::Area::new("Game over!")
        .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::ZERO)
        .order(egui::Order::Foreground)
        .show(ctx.ctx_mut(), |ui| {
            egui::Frame::popup(ui.style())
                .fill(egui::Color32::BLACK)
                .stroke(egui::Stroke::new(1.0, egui::Color32::DARK_GRAY))
                .show(ui, |ui| {
                    ui.visuals_mut().widgets.inactive.expansion = 2.0;
                    ui.vertical_centered(|ui| {
                        let msg = format!(
                            "You ran out of time after making {} moves...",
                            turns.get_completed()
                        );
                        ui.label(msg);
                        if ui.button("Try again!").clicked() {
                            commands.insert_resource(NextState(GameState::StartGame));
                        }
                    });
                });
        });
}
