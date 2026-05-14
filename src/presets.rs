use bevy::prelude::*;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::{
    aurora_material::AuroraMaterial,
    bind_groups::GradientBindGroup,
    gradient_material::FullGradientMaterial,
    sky_material::FullSkyMaterial,
    sun::SunSettings,
};

/// Default sky palette for the gradient material
pub const DEFAULT_SKY_PALETTE: GradientBindGroup = GradientBindGroup {
    a: Vec3::new(0.4, 0.7, 1.0),
    b: Vec3::new(0.2, 0.1, 0.1),
    c: Vec3::new(1.0, 1.0, 1.0),
    d: Vec3::new(0.0, 0.0, 0.0),
};

/// Data that controls the look of a sky.
/// (None) values will not override current sky settings.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Default)]
pub struct SkyPreset {
    pub gradient_bind_group: Option<GradientBindGroup>,
    pub aurora_settings: Option<crate::bind_groups::AuroraBindGroup>,
    pub sun_settings: Option<SunSettings>,
    pub stars: Option<crate::bind_groups::StarsBindGroup>,
}

pub struct SkyPresetPlugin;

impl Plugin for SkyPresetPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<ApplyPresetEvent>();
        app.add_systems(Update, handle_apply_preset_events);
    }
}

#[derive(Message)]
pub struct ApplyPresetEvent {
    pub sky_preset: SkyPreset,
}

#[allow(clippy::too_many_arguments)]
pub fn handle_apply_preset_events(
    mut events: MessageReader<ApplyPresetEvent>,
    skyboxes: Query<&mut MeshMaterial3d<FullSkyMaterial>>,
    auroras: Query<&mut MeshMaterial3d<AuroraMaterial>>,
    gradient_handles: Query<&mut MeshMaterial3d<FullGradientMaterial>>,
    mut sky_materials: ResMut<Assets<FullSkyMaterial>>,
    mut auroras_materials: ResMut<Assets<AuroraMaterial>>,
    mut gradient_materials: ResMut<Assets<FullGradientMaterial>>,
    mut sun_settings_optional: Option<ResMut<SunSettings>>,
) {
    for event in events.read() {
        if let Some(new_sun_settings) = &event.sky_preset.sun_settings
            && let Some(current_sun_settings) = &mut sun_settings_optional
        {
            **current_sun_settings = new_sun_settings.clone();
        }

        if let Some(star_settings) = &event.sky_preset.stars {
            let skybox_material_handle = skyboxes
                .single()
                .expect("1 entity with SkyGradientMaterial");
            let skybox_material = sky_materials
                .get_mut(skybox_material_handle)
                .expect("SkyBoxMaterial");
            skybox_material.stars = star_settings.clone();
        }

        if let Some(aurora_bind_group) = &event.sky_preset.aurora_settings {
            let aurora_material_handle =
                auroras.single().expect("1 entity with SkyGradientMaterial");
            let aurora_material = auroras_materials
                .get_mut(aurora_material_handle)
                .expect("auroraMaterial");
            aurora_material.aurora_settings = aurora_bind_group.clone();
        }
        if let Some(gradient_bind_group) = &event.sky_preset.gradient_bind_group {
            let gradient_material_handle = gradient_handles
                .single()
                .expect("1 entity with FullGradientMaterial");
            let gradient_material = gradient_materials
                .get_mut(gradient_material_handle)
                .expect("gradientMaterial");
            gradient_material.gradient_bind_group = gradient_bind_group.clone();
        }
    }
}
