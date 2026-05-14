use bevy::prelude::*;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::cycle::{SkyTime, SkyTimeSettings};

/// Settings for ambient lighting palette
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Resource, Clone, Reflect)]
pub struct AmbientPaletteBuilder {
    /// Day palette for ambient color
    pub day_color: Vec3,
    /// Night palette for ambient color
    pub night_color: Vec3,
    /// Day brightness multiplier
    pub day_brightness: f32,
    /// Night brightness multiplier
    pub night_brightness: f32,
}

impl Default for AmbientPaletteBuilder {
    fn default() -> Self {
        Self {
            day_color: Vec3::new(0.8, 0.85, 1.0),
            night_color: Vec3::new(0.3, 0.3, 0.6),
            day_brightness: 7000.0,
            night_brightness: 1500.0,
        }
    }
}

impl AmbientPaletteBuilder {
    /// Sample ambient color and brightness at a given time
    pub fn sample(&self, time: f32, sky_time_settings: &SkyTimeSettings) -> (Vec3, f32) {
        let t = sky_time_settings.time_percent(time);
        // Blend between day and night based on time_percent (0.0 to 1.0)
        // 0.0 to 0.5 is roughly day, 0.5 to 1.0 is roughly night
        let night_blend = (t - 0.5).clamp(0.0, 0.5) * 2.0;

        let color = self.day_color.lerp(self.night_color, night_blend);
        let brightness = self.day_brightness.lerp(self.night_brightness, night_blend);

        (color, brightness)
    }

    pub fn with_day_color(mut self, color: Vec3) -> Self {
        self.day_color = color;
        self
    }
    pub fn with_night_color(mut self, color: Vec3) -> Self {
        self.night_color = color;
        self
    }
    pub fn with_day_brightness(mut self, brightness: f32) -> Self {
        self.day_brightness = brightness;
        self
    }
    pub fn with_night_brightness(mut self, brightness: f32) -> Self {
        self.night_brightness = brightness;
        self
    }
}

/// Plugin that drives ambient lighting based on sky time
#[derive(Clone, Default)]
pub struct AmbientDriverPlugin {
    pub ambient_palette_builder: AmbientPaletteBuilder,
}

impl Plugin for AmbientDriverPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(self.ambient_palette_builder.clone());

        app.add_systems(PostUpdate, drive_ambience);
    }
}

fn drive_ambience(
    sky_time_settings: Res<SkyTimeSettings>,
    sky_time: Res<SkyTime>,
    ambient_palette_builder: Res<AmbientPaletteBuilder>,
    mut ambient_light: ResMut<GlobalAmbientLight>,
) {
    let (color, brightness) = ambient_palette_builder.sample(sky_time.time, &sky_time_settings);

    ambient_light.color = Color::srgb(color.x, color.y, color.z);
    ambient_light.brightness = brightness;
}
