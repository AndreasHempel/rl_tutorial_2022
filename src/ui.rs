use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};
use iyes_loopless::{prelude::IntoConditionalSystem, state::NextState};

use crate::{log::LogBuffer, player::Player, GameState};

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

fn render_ui(mut ctx: ResMut<EguiContext>, player: Query<&Player>, logs: Res<LogBuffer>) {
    egui::TopBottomPanel::bottom("UI panel")
        .max_height(200.0)
        .default_height(200.0)
        .show(ctx.ctx_mut(), |ui| {
            ui.vertical(|ui| {
                if let Ok(player) = player.get_single() {
                    ui.horizontal(|ui| {
                        ui.horizontal(|ui| {
                            ui.label("Action points left: ");
                            ui.label(player.get_remaining_ap().to_string());
                        });
                        ui.add_space(50.0);
                        ui.horizontal(|ui| {
                            ui.label("Turns completed: ");
                            ui.label(player.get_completed_turns().to_string());
                        });
                    });
                }
                ui.separator();
                let row_height = ui.text_style_height(&egui::TextStyle::Body);
                let num_logs = logs.len();
                egui::ScrollArea::vertical()
                    .auto_shrink([false; 2])
                    .show_rows(ui, row_height, num_logs, |ui, row_range| {
                        for idx in row_range {
                            // Transform the row index to read the log buffer from the back (newest message at the top)
                            let m = &logs[num_logs - (idx + 1)];
                            ui.label(m);
                        }
                    });
            });
        });
}

fn gameover_menu(mut ctx: ResMut<EguiContext>, mut commands: Commands, player: Query<&Player>) {
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
                            player.single().get_completed_turns()
                        );
                        ui.label(msg);
                        if ui.button("Try again!").clicked() {
                            commands.insert_resource(NextState(GameState::StartGame));
                        }
                    });
                });
        });
}
