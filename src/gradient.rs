use bevy::prelude::*;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::bind_groups::GradientBindGroup;
use crate::cycle::SkyTimeSettings;

/// Simple palette parameters for cosine-based color generation.
/// The palette function: `a + b * cos(2*pi * (c * t + d))`
/// where t is the normalized position (0.0 to 1.0).
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Resource, Clone, Debug, Reflect)]
pub struct SkyPalette {
    /// Base color offset
    pub a: Vec3,
    /// Amplitude of color variation
    pub b: Vec3,
    /// Frequency of color oscillation
    pub c: Vec3,
    /// Phase shift
    pub d: Vec3,
}

impl SkyPalette {
    /// Convert to a GradientBindGroup for use in materials
    pub fn to_bind_group(&self) -> GradientBindGroup {
        GradientBindGroup {
            a: self.a,
            b: self.b,
            c: self.c,
            d: self.d,
        }
    }
}

impl Default for SkyPalette {
    fn default() -> Self {
        Self {
            // Nice sky blue gradient
            a: Vec3::new(0.5, 0.6, 0.8),
            b: Vec3::new(0.1, 0.1, 0.2),
            c: Vec3::new(1.0, 1.0, 1.0),
            d: Vec3::new(0.0, 0.0, 0.0),
        }
    }
}

impl SkyPalette {
    pub fn with_a(mut self, a: Vec3) -> Self {
        self.a = a;
        self
    }
    pub fn with_b(mut self, b: Vec3) -> Self {
        self.b = b;
        self
    }
    pub fn with_c(mut self, c: Vec3) -> Self {
        self.c = c;
        self
    }
    pub fn with_d(mut self, d: Vec3) -> Self {
        self.d = d;
        self
    }
}

/// Builder for sky palettes that can animate based on time of day.
/// Uses separate palettes for day and night, with smooth transitions.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Resource)]
pub struct SkyPaletteBuilder {
    /// Palette used during day time (roughly t = 0.0 to 0.5)
    pub day: SkyPalette,
    /// Palette used during night time (roughly t = 0.5 to 1.0)
    pub night: SkyPalette,
}

impl Default for SkyPaletteBuilder {
    fn default() -> Self {
        Self {
            day: SkyPalette {
                a: Vec3::new(0.4, 0.7, 1.0),
                b: Vec3::new(0.2, 0.1, 0.1),
                c: Vec3::new(1.0, 1.0, 1.0),
                d: Vec3::new(0.0, 0.0, 0.0),
            },
            night: SkyPalette {
                a: Vec3::new(0.0, 0.05, 0.2),
                b: Vec3::new(0.05, 0.0, 0.1),
                c: Vec3::new(1.0, 1.0, 1.0),
                d: Vec3::new(0.0, 0.0, 0.0),
            },
        }
    }
}

impl SkyPaletteBuilder {
    /// Sample the palette at a given time, blending between day and night.
    /// 
    /// The time is converted using SkyTimeSettings::time_percent():
    /// - 0.0 to ~0.5: day (sunrise -> day -> sunset)
    /// - ~0.5 to 1.0: night (sunset -> night -> sunrise)
    pub fn sample(&self, time: f32, sky_time_settings: &SkyTimeSettings) -> SkyPalette {
        // Get time percent (0.0 to 1.0) that accounts for day/night cycle settings
        let t = sky_time_settings.time_percent(time);
        
        // Calculate blend factor: 0.0 = full day, 1.0 = full night
        // Day is 0.0 to 0.5, Night is 0.5 to 1.0
        let blend = (t - 0.5).clamp(0.0, 0.5) * 2.0;
        
        SkyPalette {
            a: self.day.a.lerp(self.night.a, blend),
            b: self.day.b.lerp(self.night.b, blend),
            c: self.day.c.lerp(self.night.c, blend),
            d: self.day.d.lerp(self.night.d, blend),
        }
    }

    pub fn with_day(mut self, day: SkyPalette) -> Self {
        self.day = day;
        self
    }
    pub fn with_night(mut self, night: SkyPalette) -> Self {
        self.night = night;
        self
    }
}
