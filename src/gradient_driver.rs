use bevy::prelude::*;

use crate::{
    cycle::{SkyTime, SkyTimeSettings},
    gradient::SkyPaletteBuilder,
    gradient_material::FullGradientMaterial,
};

/// Animates the sky palette based on time of day.
/// Requires CyclePlugin to be enabled.
#[derive(Clone, Default)]
pub struct GradientDriverPlugin {
    pub sky_palette_builder: SkyPaletteBuilder,
}

impl GradientDriverPlugin {
    pub fn with_sky_palette_builder(mut self, builder: SkyPaletteBuilder) -> Self {
        self.sky_palette_builder = builder;
        self
    }
}

impl Plugin for GradientDriverPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(self.sky_palette_builder.clone());
        app.add_systems(Update, drive_palette);
    }
}

/// Drive the sky gradient material with palette parameters based on time of day
fn drive_palette(
    sky_time_settings: Res<SkyTimeSettings>,
    sky_time: Res<SkyTime>,
    sky_palette_builder: Res<SkyPaletteBuilder>,
    skyboxes: Query<&mut MeshMaterial3d<FullGradientMaterial>>,
    mut sky_materials: ResMut<Assets<FullGradientMaterial>>,
) {
    let skybox_material_handle = skyboxes
        .single()
        .expect("1 entity with FullGradientMaterial");
    let skybox_material = sky_materials
        .get_mut(skybox_material_handle)
        .expect("FullGradientMaterial");

    let palette = sky_palette_builder.sample(sky_time.time, &sky_time_settings);
    let mut bind_group = palette.to_bind_group();
    // Use day/night brightness from builder for shader interpolation
    bind_group.brightness_day = sky_palette_builder.day.brightness;
    bind_group.brightness_night = sky_palette_builder.night.brightness;
    skybox_material.gradient_bind_group = bind_group;
}
