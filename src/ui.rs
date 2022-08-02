use bevy::prelude::*;
use bevy_egui::{egui, egui::TextureId, EguiContext};

/// Bundles systems responsible for rendering
#[derive(Debug)]
pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_ui).add_system(render_ui);
    }
}

#[derive(Debug)]
struct UISprite {
    image: TextureId,
    /// Handle to the underlying image asset to prevent it from being unloaded
    handle: Handle<Image>,
    /// Full size of texture can only be read once loaded into memory
    full_size: egui::Vec2,
    /// Subcoordinates to be read once texture is loaded into memory
    uv_coords: egui::Rect,
}

impl UISprite {
    fn new(image: TextureId, handle: Handle<Image>, idx: u32, cols: u32, rows: u32) -> Self {
        let sprite_size =
            egui::Rect::from_points(&[egui::pos2(0.0, 0.0), egui::pos2(SPRITE_SIZE, SPRITE_SIZE)]);
        let row = idx / cols;
        let col = idx % cols;
        info!("Row: {row}, col: {col}, idx: {idx}");
        let uv_coords = egui::Rect::from_min_max(
            egui::pos2((col as f32) / (cols as f32), (row as f32) / (rows as f32)),
            egui::pos2(
                ((col + 1) as f32) / (cols as f32),
                ((row + 1) as f32) / (rows as f32),
            ),
        );
        Self {
            image,
            handle,
            full_size: sprite_size.size(),
            uv_coords,
        }
    }
}

#[derive(Debug)]
struct UIResources {
    heart: UISprite,
    bar_start: UISprite,
    bar_mid: UISprite,
    bar_end: UISprite,
    bar_fill: UISprite,
}

const SPRITE_SIZE: f32 = 16.0;

fn setup_ui(asset_server: Res<AssetServer>, mut ctx: ResMut<EguiContext>, mut commands: Commands) {
    let texture_handle = asset_server.load("Dawnlike/GUI/GUI0.png");
    // FIXME: Passing a strong handle to add_image _should_ prevent the image from being unloaded
    // based on https://docs.rs/bevy_egui/0.14.0/bevy_egui/struct.EguiContext.html#method.add_image
    // but this does not seem to work (likely because only the handle.id is used in the implementation)
    // of add_image and the handle gets dropped there
    let texture_id = ctx.add_image(texture_handle.clone());

    commands.insert_resource(UIResources {
        heart: UISprite::new(texture_id.clone(), texture_handle.clone(), 0, 16, 19),
        bar_start: UISprite::new(texture_id.clone(), texture_handle.clone(), 6, 16, 19),
        bar_mid: UISprite::new(texture_id.clone(), texture_handle.clone(), 7, 16, 19),
        bar_end: UISprite::new(texture_id.clone(), texture_handle.clone(), 8, 16, 19),
        bar_fill: UISprite::new(texture_id, texture_handle, 22, 16, 19),
    })
}

fn render_ui(ui_images: Res<UIResources>, mut ctx: ResMut<EguiContext>) {
    egui::TopBottomPanel::bottom("Bottom panel").show(ctx.ctx_mut(), |ui| {
        ui.add(
            egui::Image::new(ui_images.heart.image, ui_images.heart.full_size)
                .uv(ui_images.heart.uv_coords),
        );
        ui.horizontal(|ui| {
            ui.add(
                egui::Image::new(ui_images.bar_start.image, ui_images.bar_start.full_size)
                    .uv(ui_images.bar_start.uv_coords),
            );
            ui.add_space(-10.0);
            ui.add(
                egui::Image::new(ui_images.bar_mid.image, ui_images.bar_mid.full_size)
                    .uv(ui_images.bar_mid.uv_coords),
            );
            for h in [
                &ui_images.bar_start,
                &ui_images.bar_mid,
                &ui_images.bar_end,
                &ui_images.bar_fill,
            ] {
                ui.add(egui::Image::new(h.image, h.full_size).uv(h.uv_coords));
            }
        });
    });
}
